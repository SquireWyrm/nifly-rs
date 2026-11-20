//! Tests for BSGeometry's own fields and its meshes' data.

use std::fs;

use nifly::{
    file::{Index, NifFile},
    geometry::{BsGeometry, CullData, MeshInfo, MeshWeight, Meshlet, NiShape},
    object3d::{BoundingSphere, Vec3},
};

use crate::common::{UPSTREAM, reload_in_place as reload, upstream};

/// TestNifFile_SF.nif with its first mesh loaded and made internal.
fn internal() -> (NifFile, Index) {
    let mut nif = upstream("TestNifFile_SF");
    let shape = nif.blocks_of::<BsGeometry>().indices().next().unwrap();
    let mesh = fs::read(format!("{UPSTREAM}/TestNifFile_SF.mesh")).unwrap();
    assert!(
        nif.edit::<NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh.as_slice())
            .unwrap()
    );
    nif.block_of_mut::<BsGeometry>(shape)
        .unwrap()
        .set_internal_geometry(true);
    (nif, shape)
}

fn geometry(nif: &NifFile, index: Index) -> &BsGeometry {
    nif.block_of::<BsGeometry>(index).unwrap()
}

#[test]
fn shape_fields() {
    let mut nif = upstream("TestNifFile_SF");
    let index = nif.blocks_of::<BsGeometry>().indices().next().unwrap();
    let sphere = BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
    let info = MeshInfo {
        index_count: 30,
        vertex_count: 12,
        raw_flags: 64,
    };
    let count = geometry(&nif, index).mesh_count();
    assert!(!geometry(&nif, index).has_internal_geometry());

    let block = nif.block_of_mut::<BsGeometry>(index).unwrap();
    block.set_bounds(sphere);
    block.set_bound_min_max([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    assert!(block.set_mesh_info(0, info));
    assert!(!block.set_mesh_info(count, info));
    while block.mesh_count() < 4 {
        assert!(block.add_mesh());
    }
    assert!(!block.add_mesh());

    let saved = reload(&mut nif);
    let block = geometry(&saved, index);
    assert_eq!(
        (block.bounds(), block.bound_min_max()),
        (sphere, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    );
    assert_eq!(
        block.mesh_info(0),
        Some(info),
        "kept: the mesh data isn't loaded"
    );
    assert_eq!(block.mesh_count(), 4);
    assert_eq!(block.mesh_info(4), None);
}

#[test]
fn counts_of_mesh_data() {
    let (nif, index) = internal();
    let view = nif.view::<NiShape>(index).unwrap();
    let data = view.geometry_data().unwrap();
    assert_eq!((data.vertex_count(), data.triangle_count()), (6616, 12132));
    let mesh = geometry(&nif, index).mesh(0).unwrap();
    assert_eq!(mesh.triangles().len(), 12132);
    assert!(geometry(&nif, index).mesh(4).is_none());
}

#[test]
fn internal_mesh_data_round_trips() {
    let (mut nif, index) = internal();
    let n = geometry(&nif, index).mesh(0).unwrap().vertex_count();
    let tangents = geometry(&nif, index).mesh(0).unwrap().tangents().len();
    let weights = vec![
        vec![
            MeshWeight {
                bone: 1,
                weight: 65535
            },
            MeshWeight { bone: 0, weight: 0 }
        ];
        n
    ];
    let meshlets = [Meshlet {
        vertex_count: 3,
        vertex_offset: 0,
        primitive_count: 1,
        primitive_offset: 0,
    }];
    let cull = [CullData {
        center: Vec3::new(1.0, 2.0, 3.0),
        expand: Vec3::new(0.5, 0.5, 0.5),
    }];
    let lods = [vec![[0, 1, 2]], vec![[2, 1, 0], [0, 1, 3]]];

    let block = nif.block_of_mut::<BsGeometry>(index).unwrap();
    let mesh = block.mesh_mut(0).unwrap();
    assert!(!mesh.set_second_uvs(&vec![[0.5, 0.25]; n - 1]));
    assert!(mesh.set_second_uvs(&vec![[0.5, 0.25]; n]));
    assert!(mesh.set_colors(&vec![[1, 2, 3, 4]; n]));
    assert!(!mesh.set_tangent_ws(&vec![4; tangents]));
    assert!(mesh.set_tangent_ws(&vec![2; tangents]));
    assert!(!mesh.set_skin_weights(&weights[1..]));
    let nine = vec![vec![MeshWeight { bone: 0, weight: 0 }; 9]; n];
    assert!(!mesh.set_skin_weights(&nine), "more than 8");
    assert!(mesh.set_skin_weights(&weights));
    assert!(!mesh.set_lods(&[vec![[0, 1, n as u16]]]));
    assert!(mesh.set_lods(&lods));
    mesh.set_meshlets(&meshlets);
    mesh.set_cull_data(&cull);
    mesh.set_version(1);

    let saved = reload(&mut nif);
    assert!(geometry(&saved, index).has_internal_geometry());
    let mesh = geometry(&saved, index).mesh(0).unwrap();
    assert_eq!(mesh.vertex_count(), n);
    assert_eq!(mesh.second_uvs(), vec![[0.5, 0.25]; n]);
    assert_eq!(mesh.colors(), vec![[1, 2, 3, 4]; n]);
    assert_eq!(mesh.tangent_ws(), vec![2; tangents]);
    assert_eq!(mesh.skin_weights(), weights);
    assert_eq!(mesh.lods(), lods);
    assert_eq!(
        (mesh.meshlets(), mesh.cull_data()),
        (meshlets.to_vec(), cull.to_vec())
    );
    assert_eq!(mesh.version(), 1);
    assert_eq!(
        geometry(&saved, index).mesh_info(0).unwrap().vertex_count as usize,
        n,
        "set on save"
    );
}

#[test]
fn weights_per_vertex() {
    let w = |bone, weight| MeshWeight { bone, weight };
    let (mut nif, index) = internal();
    let n = geometry(&nif, index).mesh(0).unwrap().vertex_count();
    // The first vertex drops only a weight of 0; the others drop a weight of 10000.
    let mut weights = vec![vec![w(0, 10000), w(1, 40000), w(2, 15535)]; n];
    weights[0] = vec![w(0, 0), w(1, 65535), w(2, 0)];

    let mesh = nif
        .block_of_mut::<BsGeometry>(index)
        .unwrap()
        .mesh_mut(0)
        .unwrap();
    assert!(mesh.set_skin_weights::<Vec<MeshWeight>>(&[]));
    assert_eq!(mesh.weights_per_vertex(), 0);
    assert!(!mesh.set_weights_per_vertex(2), "no weights");
    assert!(mesh.set_skin_weights(&weights));
    assert_eq!(mesh.weights_per_vertex(), 3);
    assert!(!mesh.set_weights_per_vertex(0));
    assert!(!mesh.set_weights_per_vertex(9));
    assert_eq!(
        mesh.skin_weights(),
        weights,
        "rejected calls change nothing"
    );

    assert!(mesh.set_weights_per_vertex(2));
    let kept = mesh.skin_weights();
    assert_eq!(
        kept[0],
        [w(1, 65535), w(0, 0)],
        "strongest first, ties in order"
    );
    assert_eq!(kept[1].iter().map(|w| w.bone).collect::<Vec<_>>(), [1, 2]);
    let sum: u32 = kept[1].iter().map(|w| u32::from(w.weight)).sum();
    assert!(sum.abs_diff(65535) <= 1, "{sum}");
    assert!(kept[1][0].weight > 40000, "rescaled");

    assert!(mesh.set_weights_per_vertex(4));
    assert_eq!(mesh.weights_per_vertex(), 4);
    let grown = mesh.skin_weights();
    assert_eq!(grown[1][..2], kept[1][..]);
    assert_eq!(grown[1][2..], [w(0, 0), w(0, 0)]);

    let saved = reload(&mut nif);
    let mesh = geometry(&saved, index).mesh(0).unwrap();
    assert_eq!(mesh.weights_per_vertex(), 4);
    assert_eq!(mesh.skin_weights(), grown);
}

#[test]
fn shape_weights_per_vertex() {
    let other = upstream("TestNifFile_Skinned_SE");
    let shape = other.shapes().views().next().unwrap();
    assert_eq!(
        (shape.max_weights_per_vertex(), shape.weights_per_vertex()),
        (4, 4)
    );

    let (mut nif, index) = internal();
    let view = nif.view::<NiShape>(index).unwrap();
    assert_eq!(view.max_weights_per_vertex(), 8);
    let n = geometry(&nif, index).mesh(0).unwrap().vertex_count();
    let mesh = nif
        .block_of_mut::<BsGeometry>(index)
        .unwrap()
        .mesh_mut(0)
        .unwrap();
    assert!(mesh.set_skin_weights::<Vec<MeshWeight>>(&[]));
    assert_eq!(
        nif.view::<NiShape>(index).unwrap().weights_per_vertex(),
        4,
        "4 without weights"
    );
    let mesh = nif
        .block_of_mut::<BsGeometry>(index)
        .unwrap()
        .mesh_mut(0)
        .unwrap();
    assert!(mesh.set_skin_weights(&vec![
        vec![
            MeshWeight {
                bone: 0,
                weight: 65535
            };
            6
        ];
        n
    ]));
    assert_eq!(nif.view::<NiShape>(index).unwrap().weights_per_vertex(), 6);
}

#[test]
fn replace_vertices_and_triangles() {
    let (mut nif, index) = internal();
    let block = nif.block_of_mut::<BsGeometry>(index).unwrap();
    let mesh = block.mesh_mut(0).unwrap();
    let few = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    assert!(
        !mesh.replace_vertices(&few),
        "the triangles use more vertices"
    );
    assert!(!mesh.set_triangles(&[[0, 1, 6616]]));
    assert!(mesh.set_lods::<Vec<[u16; 3]>>(&[]));
    assert!(mesh.set_triangles(&[[0, 1, 2]]));
    assert!(mesh.replace_vertices(&few));
    assert_eq!(
        (mesh.vertex_count(), mesh.normals().len(), mesh.uvs().len()),
        (3, 3, 3)
    );

    // More than 65535 vertices, far outside the stored scale.
    let many: Vec<Vec3> = (0..70000)
        .map(|i| Vec3::new(i as f32, 0.0, -1000.0))
        .collect();
    assert!(mesh.replace_vertices(&many));
    let scale = mesh.scale();
    assert_eq!(mesh.vertex_count(), 70000);

    let saved = reload(&mut nif);
    assert_eq!(
        geometry(&nif, index).mesh(0).unwrap().scale(),
        scale,
        "saving doesn't change it"
    );
    let mesh = geometry(&saved, index).mesh(0).unwrap();
    assert_eq!(mesh.vertex_count(), 70000);
    let last = mesh.vertices()[69999];
    assert!(
        (last.x - 69999.0).abs() < 2.0 && (last.z + 1000.0).abs() < 2.0,
        "{last:?}"
    );
    assert_eq!(mesh.triangles(), [[0, 1, 2]]);
}

#[test]
fn generate_meshlets() {
    let (mut nif, index) = internal();
    let block = nif.block_of_mut::<BsGeometry>(index).unwrap();
    let mesh = block.mesh_mut(0).unwrap();
    assert!(mesh.generate_meshlets(64, 64));
    let meshlets = mesh.meshlets();
    assert!(!meshlets.is_empty());
    assert_eq!(
        meshlets.iter().map(|m| m.primitive_count).sum::<u32>(),
        12132
    );
    assert_eq!(mesh.cull_data().len(), meshlets.len());
}

#[test]
fn internal_save_matches_expected() {
    // Upstream's "to internal mesh" test: load each external mesh, make the data internal,
    // save, reload and save again.
    let mut nif = upstream("TestNifFile_ToInternalMesh_SF");
    let shapes: Vec<_> = nif.blocks_of::<BsGeometry>().indices().collect();
    for &shape in &shapes {
        let paths: Vec<String> = nif
            .view::<NiShape>(shape)
            .unwrap()
            .mesh_paths()
            .iter()
            .map(|p| p.to_string())
            .collect();
        for (lod, path) in paths.iter().enumerate() {
            let mesh = fs::read(format!("{UPSTREAM}/{path}.mesh")).unwrap();
            assert!(
                nif.edit::<NiShape>(shape)
                    .unwrap()
                    .load_mesh(lod, mesh.as_slice())
                    .unwrap()
            );
        }
        nif.block_of_mut::<BsGeometry>(shape)
            .unwrap()
            .set_internal_geometry(true);
    }
    let mut saved = crate::common::reload(&mut nif);
    let expected =
        fs::read("nifly-sys/vendor/nifly/tests/expected/TestNifFile_ToInternalMesh_SF.nif")
            .unwrap();
    assert!(crate::common::save(&mut saved) == expected);
}
