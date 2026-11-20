//! Tests for the less common fields of nodes, effects and skins: unknown values, screen counts,
//! particle systems, per-bone skin data, skin bones and partition details.

use std::io::Cursor;

use nifly::{
    effects::NiTextureEffect,
    extra_data::ExtraInteger,
    file::{Index, NifFile, NifVersion, SaveOptions},
    geometry::{NiShape, VertexWeight},
    nodes::{BsFaceGenNiNode, BsMasterParticleSystem, NiCamera, NiNode},
    object3d::{BoundingSphere, Matrix3, Transform, Vec3},
    skin::{BsDismemberSkinInstance, BsSkinBoneData, NiBoneContainer, NiSkinData, NiSkinPartition},
};

use crate::common::{load, upstream};

/// Saves without optimizing or sorting, so block indices stay.
fn reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(
        &mut cur,
        Some(SaveOptions {
            optimize: false,
            sort_blocks: false,
        }),
    )
    .unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

fn first<T: nifly::objects::NiType>(nif: &NifFile) -> Index {
    nif.blocks_of::<T>().indices().next().unwrap()
}

fn transform(x: f32) -> Transform {
    Transform::new(Vec3::new(x, 2.0, 3.0), Matrix3::IDENTITY, 1.5)
}

#[test]
fn face_gen_and_master_particle_system() {
    let mut nif = NifFile::create(NifVersion::SKYRIM);
    let face = nif
        .add_node_of::<BsFaceGenNiNode>("Face", Transform::default(), None)
        .unwrap();
    let master = nif
        .add_node_of::<BsMasterParticleSystem>("Master", Transform::default(), None)
        .unwrap();
    let system = nif
        .add_node_of::<NiNode>("System", Transform::default(), Some(master))
        .unwrap();
    nif.block_of_mut::<BsFaceGenNiNode>(face)
        .unwrap()
        .set_unknown(7);
    nif.block_of_mut::<BsMasterParticleSystem>(master)
        .unwrap()
        .set_max_emitter_objects(5);
    let mut edit = nif.edit::<BsMasterParticleSystem>(master).unwrap();
    let extra = edit.assign_extra_data(ExtraInteger("", 1)).unwrap();
    assert!(!edit.set_particle_systems(&[system, extra]));
    assert!(edit.set_particle_systems(&[system]));

    let saved = reload(&mut nif);
    assert_eq!(
        saved.block_of::<BsFaceGenNiNode>(face).unwrap().unknown(),
        7
    );
    let view = saved.view::<BsMasterParticleSystem>(master).unwrap();
    assert_eq!(view.max_emitter_objects(), 5);
    assert_eq!(
        view.particle_systems().indices().collect::<Vec<_>>(),
        [system]
    );
}

#[test]
fn camera_screen_counts() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let camera = nif
        .add_camera("Camera", Transform::default(), None)
        .unwrap();
    let block = nif.block_of_mut::<NiCamera>(camera).unwrap();
    block.set_screen_polygon_count(3);
    block.set_screen_texture_count(4);
    let saved = reload(&mut nif);
    let camera = saved.block_of::<NiCamera>(camera).unwrap();
    assert_eq!(
        (camera.screen_polygon_count(), camera.screen_texture_count()),
        (3, 4)
    );
}

#[test]
fn texture_effect_raw_fields() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let effect = first::<NiTextureEffect>(&nif);
    let block = nif.block_of_mut::<NiTextureEffect>(effect).unwrap();
    block.set_clipping_value(2);
    block.set_unknown(0x1234);
    let saved = reload(&mut nif);
    let effect = saved.block_of::<NiTextureEffect>(effect).unwrap();
    assert_eq!(
        (effect.clipping_value(), effect.clipping(), effect.unknown()),
        (2, true, 0x1234)
    );
}

#[test]
fn skin_data_per_bone() {
    let mut nif = load("Bones");
    let data = first::<NiSkinData>(&nif);
    let bounds = BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
    let weights = [
        VertexWeight {
            vertex: 3,
            weight: 0.25,
        },
        VertexWeight {
            vertex: 1,
            weight: 1.5,
        },
    ];
    let block = nif.block_of_mut::<NiSkinData>(data).unwrap();
    assert!(block.set_transform_skin_to_bone(1, transform(5.0)));
    assert!(block.set_bounds(1, bounds));
    assert!(block.set_vertex_weights(2, &weights));
    assert!(!block.set_transform_skin_to_bone(3, transform(5.0)));
    assert!(!block.set_bounds(3, bounds));
    assert!(!block.set_vertex_weights(3, &weights));
    assert!(!block.set_vertex_weights(0, &vec![weights[0]; 65536]));

    let saved = reload(&mut nif);
    let block = saved.block_of::<NiSkinData>(data).unwrap();
    assert_eq!(
        (block.transform_skin_to_bone(1), block.bounds(1)),
        (Some(transform(5.0)), Some(bounds))
    );
    assert_eq!(block.vertex_weights(2).unwrap(), weights);
    assert_eq!(block.vertex_weights(0).unwrap().len(), 102);
    assert_eq!(block.vertex_weights(3), None);
}

#[test]
fn skin_bone_data_per_bone() {
    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let data = first::<BsSkinBoneData>(&nif);
    let bounds = BoundingSphere::new(Vec3::new(-1.0, 0.0, 1.0), 9.0);
    let count = nif.block_of::<BsSkinBoneData>(data).unwrap().bone_count();
    let block = nif.block_of_mut::<BsSkinBoneData>(data).unwrap();
    assert!(block.set_transform_skin_to_bone(0, transform(7.0)));
    assert!(block.set_bounds(0, bounds));
    assert!(!block.set_transform_skin_to_bone(count, transform(7.0)));
    assert!(!block.set_bounds(count, bounds));
    let saved = reload(&mut nif);
    let block = saved.block_of::<BsSkinBoneData>(data).unwrap();
    assert_eq!(
        (block.transform_skin_to_bone(0), block.bounds(0)),
        (Some(transform(7.0)), Some(bounds))
    );
}

#[test]
fn skin_bones() {
    // Bones.nif: the skin uses bones 1, 2 and 3.
    let mut nif = load("Bones");
    let shape = nif.shapes().indices().next().unwrap();
    let skin = nif
        .view::<NiShape>(shape)
        .unwrap()
        .skin_instance()
        .unwrap()
        .index();
    let mut edit = nif.edit::<NiBoneContainer>(skin).unwrap();
    assert_eq!(edit.view().bones().indices().collect::<Vec<_>>(), [1, 2, 3]);
    assert!(!edit.set_bones(&[3, 2]), "the data has three bones");
    assert!(!edit.set_bones(&[3, 2, shape]), "not a node");
    assert!(edit.set_bones(&[3, 2, 1]));
    let saved = reload(&mut nif);
    assert_eq!(saved.view::<NiShape>(shape).unwrap().bone_ids(), [3, 2, 1]);
}

#[test]
fn dismember_body_part() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let skin = first::<BsDismemberSkinInstance>(&nif);
    let block = nif.block_of_mut::<BsDismemberSkinInstance>(skin).unwrap();
    assert!(block.set_body_part(0, 42));
    assert!(!block.set_body_part(1, 42));
    assert_eq!(
        reload(&mut nif)
            .block_of::<BsDismemberSkinInstance>(skin)
            .unwrap()
            .body_part(0),
        Some(42)
    );
}

#[test]
fn partition_details() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let part = first::<NiSkinPartition>(&nif);
    let block = nif.block_of::<NiSkinPartition>(part).unwrap();
    assert_eq!(
        (
            block.has_vertex_map(0),
            block.has_vertex_weights(0),
            block.has_faces(0),
            block.has_bone_indices(0)
        ),
        (Some(true), Some(true), Some(true), Some(true))
    );
    assert_eq!(block.has_faces(1), None);
    let desc = block.vertex_desc();
    let reversed: Vec<[u16; 3]> = block
        .true_triangles(0)
        .unwrap()
        .iter()
        .map(|&[a, b, c]| [a, c, b])
        .collect();

    let block = nif.block_of_mut::<NiSkinPartition>(part).unwrap();
    assert!(block.set_has_bone_indices(0, false));
    assert!(!block.set_has_faces(1, false));
    assert!(block.set_partition_vertex_desc(0, 0x1234));
    assert!(!block.set_true_triangles(0, &reversed[1..]));
    assert!(block.set_true_triangles(0, &reversed));
    block.set_vertex_desc(desc);

    let saved = reload(&mut nif);
    let block = saved.block_of::<NiSkinPartition>(part).unwrap();
    assert_eq!(block.has_bone_indices(0), Some(false));
    assert_eq!(block.bone_indices(0), Some(&[][..]));
    assert_eq!(block.true_triangles(0).unwrap(), reversed);
    // Saving copies the shape's vertex format into the partition it uses.
    assert_eq!(
        (block.vertex_desc(), block.partition_vertex_desc(0)),
        (desc, Some(desc))
    );

    // A partition no shape uses keeps its own.
    let skin = first::<BsDismemberSkinInstance>(&nif);
    assert!(
        nif.edit::<nifly::skin::NiSkinInstance>(skin)
            .unwrap()
            .set_partition(None)
    );
    let block = nif.block_of_mut::<NiSkinPartition>(part).unwrap();
    block.set_vertex_desc(0x5678);
    assert!(block.set_partition_vertex_desc(0, 0x1234));
    let saved = reload(&mut nif);
    let block = saved.block_of::<NiSkinPartition>(part).unwrap();
    assert_eq!(
        (block.vertex_desc(), block.partition_vertex_desc(0)),
        (0x5678, Some(0x1234))
    );
}
