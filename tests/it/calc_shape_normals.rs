//! Tests for NifFile::calc_shape_normals and BsShaderProperty's flags.

use crate::common::{first, load, reload, shape_index, upstream};

use nifly::{
    file::NifFile,
    geometry::{NiShape, NormalOptions},
    object3d::Vec3,
    shaders::{BsShaderProperty, SkyrimShaderFlags1},
};

const MODEL_SPACE: SkyrimShaderFlags1 = SkyrimShaderFlags1::MODEL_SPACE_NORMALS;

fn normals(nif: &NifFile, name: &str) -> Vec<Vec3> {
    nif.view_of(nif.shapes().find(|s| s.name() == name).unwrap())
        .unwrap()
        .normals()
        .unwrap()
}

/// Normals calculated with `options` on a fresh copy of the file.
fn calculated(file: fn() -> NifFile, name: &str, options: Option<NormalOptions>) -> Vec<Vec3> {
    let mut nif = file();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, name))
            .is_some_and(|mut e| e.calc_normals(options))
    );
    normals(&nif, name)
}

/// Pairs of vertices at the same position whose normals differ.
fn split_seams(nif: &NifFile, name: &str) -> usize {
    let shape = nif.shapes().find(|s| s.name() == name).unwrap();
    let (v, n) = (
        nif.view_of(shape).unwrap().vertices().unwrap(),
        nif.view_of(shape).unwrap().normals().unwrap(),
    );
    (0..v.len())
        .flat_map(|a| (a + 1..v.len()).map(move |b| (a, b)))
        .filter(|&(a, b)| v[a] == v[b] && n[a].distance(n[b]) > 1e-4)
        .count()
}

fn cuirass() -> NifFile {
    upstream("TestNifFile_Skinned_OB")
}

fn cube() -> NifFile {
    load("Cube")
}

#[test]
fn calculates_from_triangles() {
    // NiTriShapeData: overwrite the normals, then calculate them back.
    let mut nif = cuirass();
    let shape = shape_index(&nif, "Cuirass");
    let expected = calculated(cuirass, "Cuirass", None);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_normals(&vec![Vec3::Z; 1354]))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.calc_normals(None))
    );
    let got = normals(&nif, "Cuirass");
    assert_eq!(got, expected);
    assert!(got.iter().all(|n| (n.length() - 1.0).abs() < 1e-4));
    assert!(got.iter().any(|n| n.distance(Vec3::Z) > 0.5));
    assert_eq!(normals(&reload(&mut nif), "Cuirass"), expected);

    // BSTriShape, stored at reduced precision.
    let mut nif = cube();
    let shape = shape_index(&nif, "Cube");
    let expected = calculated(cube, "Cube", None);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_normals(&[Vec3::Z; 14]))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.calc_normals(None))
    );
    let got = normals(&nif, "Cube");
    assert_eq!(got, expected);
    assert!(got.iter().all(|n| (n.length() - 1.0).abs() < 0.02));
}

#[test]
fn smoothing_merges_seams() {
    let unsmoothed = Some(NormalOptions {
        smooth: false,
        ..Default::default()
    });
    let threshold = |degrees| {
        Some(NormalOptions {
            smooth_threshold: degrees,
            ..Default::default()
        })
    };

    let split = |options| {
        let mut nif = cuirass();
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cuirass"))
                .is_some_and(|mut e| e.calc_normals(options))
        );
        split_seams(&nif, "Cuirass")
    };
    // Seams within 60° are merged by default, and more within 180°.
    let (none, default, wide) = (split(unsmoothed), split(None), split(threshold(180.0)));
    assert!(none > default && default > wide, "{none} {default} {wide}");

    // The cube's seams are 90° corners, so the default doesn't merge them.
    assert_eq!(
        calculated(cube, "Cube", None),
        calculated(cube, "Cube", unsmoothed)
    );
}

#[test]
fn skips_model_space_unless_forced() {
    let mut nif = cube();
    let shape = shape_index(&nif, "Cube");
    let shader = {
        let s = nif.block_of::<NiShape>(shape).unwrap();
        nif.view_of(s).unwrap().shader().unwrap().index()
    };
    let flags = nif.block_of_mut::<BsShaderProperty>(shader).unwrap();
    let model_space = flags.flags_1::<SkyrimShaderFlags1>() | MODEL_SPACE;
    flags.set_flags_1(model_space);
    assert_eq!(
        nif.block_of::<BsShaderProperty>(shader)
            .unwrap()
            .raw_flags_1(),
        model_space.bits()
    );

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_normals(&[Vec3::Z; 14]))
    );
    let before = normals(&nif, "Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.calc_normals(None))
    );
    assert_eq!(normals(&nif, "Cube"), before);

    let forced = Some(NormalOptions {
        force_model_space: true,
        ..Default::default()
    });
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.calc_normals(forced))
    );
    assert_eq!(normals(&nif, "Cube"), calculated(cube, "Cube", None));

    // The flags survive a save.
    let saved = reload(&mut nif);
    let s = saved.shapes().find(|s| s.name() == "Cube").unwrap();
    let flags = saved
        .view_of(s)
        .unwrap()
        .shader()
        .unwrap()
        .downcast::<BsShaderProperty>()
        .unwrap()
        .flags_1::<SkyrimShaderFlags1>();
    assert_eq!(flags, model_space);
}

#[test]
fn set_raw_flags_2() {
    let mut nif = cube();
    let (shader, before) = nif
        .blocks_of::<BsShaderProperty>()
        .indexed()
        .map(|(i, s)| (i, s.raw_flags_2()))
        .next()
        .unwrap();
    nif.block_of_mut::<BsShaderProperty>(shader)
        .unwrap()
        .set_raw_flags_2(before ^ 0x10);
    let saved = reload(&mut nif);
    assert_eq!(
        first::<BsShaderProperty>(&saved).raw_flags_2(),
        before ^ 0x10
    );
}

#[test]
fn false_without_triangle_data() {
    let mut nif = upstream("TestNifFile_Particles_MW");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Blizzard"))
            .is_some_and(|mut e| e.calc_normals(None))
    );

    // Starfield geometry is left alone.
    let mut nif = upstream("TestNifFile_SF");
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.calc_normals(None))
    );

    let mut nif = cube();
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.calc_normals(None)),
            "{i}"
        );
    }
}

#[test]
fn flag_sets_by_game() {
    use nifly::shaders::{Fallout4ShaderFlags1, Fallout4ShaderFlags2, SkyrimShaderFlags2};
    let nif = upstream("TestNifFile_Skinned_SE");
    let shader = first::<BsShaderProperty>(&nif);
    assert!(
        shader
            .flags_1::<SkyrimShaderFlags1>()
            .contains(SkyrimShaderFlags1::SKINNED)
    );
    assert!(
        shader
            .flags_2::<SkyrimShaderFlags2>()
            .contains(SkyrimShaderFlags2::Z_BUFFER_WRITE)
    );

    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let (index, shader) = nif
        .blocks_of::<BsShaderProperty>()
        .indexed()
        .next()
        .unwrap();
    assert!(
        shader
            .flags_1::<Fallout4ShaderFlags1>()
            .contains(Fallout4ShaderFlags1::SKINNED)
    );
    let flags = Fallout4ShaderFlags2::DOUBLE_SIDED | Fallout4ShaderFlags2::TREE_ANIM;
    nif.block_of_mut::<BsShaderProperty>(index)
        .unwrap()
        .set_flags_2(flags);
    let saved = reload(&mut nif);
    assert_eq!(
        saved
            .block_of::<BsShaderProperty>(index)
            .unwrap()
            .flags_2::<Fallout4ShaderFlags2>(),
        flags
    );
}
