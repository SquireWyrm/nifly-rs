//! Tests for BSWeakReferenceNode weak and water references.

use crate::common::{reload};

use nifly::{
    file::{NifFile, NifVersion},
    nodes::{BsWeakReferenceNode, ResourceId, WaterReference, WeakReference, WeakReferenceMaterial},
    object3d::Transform,
};

fn node(nif: &NifFile) -> &BsWeakReferenceNode {
    nif.blocks_of::<BsWeakReferenceNode>().next().unwrap()
}

fn matrix(k: f32) -> [[f32; 4]; 4] {
    std::array::from_fn(|r| std::array::from_fn(|c| k + (r * 4 + c) as f32))
}

fn resource(n: u32) -> ResourceId {
    ResourceId { file_hash: n, extension: *b"nif\0", dir_hash: n * 2 }
}

fn weak_references() -> Vec<WeakReference> {
    vec![
        WeakReference {
            form_id: 0x0100_0ABC,
            resource: resource(1),
            transforms: vec![matrix(1.0), matrix(20.0)],
            materials: vec![WeakReferenceMaterial {
                biome_form_id: 7,
                dir_hash: 8,
                file_hash: 9,
                name: "rock.mat".into(),
            }],
        },
        WeakReference { form_id: 2, resource: resource(2), transforms: vec![], materials: vec![] },
    ]
}

fn with_node(version: NifVersion) -> (NifFile, usize) {
    let mut nif = NifFile::create(version);
    let index = nif.add_node_of::<BsWeakReferenceNode>("Weak", Transform::default(), None).unwrap();
    (nif, index)
}

#[test]
fn weak_references_round_trip() {
    let (mut nif, index) = with_node(NifVersion::new(0x14020007, 12, 173));
    assert!(nif.block_of_mut::<BsWeakReferenceNode>(index).unwrap().set_weak_references(&weak_references()));
    assert_eq!(node(&reload(&mut nif)).weak_references(), weak_references());
}

#[test]
fn form_id_not_saved_before_stream_173() {
    let (mut nif, index) = with_node(NifVersion::STARFIELD);
    assert!(nif.block_of_mut::<BsWeakReferenceNode>(index).unwrap().set_weak_references(&weak_references()));
    let saved = node(&reload(&mut nif)).weak_references();
    assert_eq!(saved[0].form_id, 0);
    assert_eq!(saved[0].transforms, weak_references()[0].transforms);
}

#[test]
fn rejects_zero_in_material_name() {
    let (mut nif, index) = with_node(NifVersion::STARFIELD);
    let n = nif.block_of_mut::<BsWeakReferenceNode>(index).unwrap();
    assert!(n.set_weak_references(&weak_references()));
    let mut bad = weak_references();
    bad[0].materials[0].name = "a\0b".into();
    assert!(!n.set_weak_references(&bad));
    assert_eq!(node(&nif).weak_references().len(), 2);
}

#[test]
fn water_references_round_trip() {
    let (mut nif, index) = with_node(NifVersion::STARFIELD);
    let water = vec![
        WaterReference { transform: matrix(3.0), resource: resource(5), unknown: 9, material: "water.mat".into() },
        WaterReference { transform: matrix(4.0), resource: resource(6), unknown: 0, material: "".into() },
    ];
    let n = nif.block_of_mut::<BsWeakReferenceNode>(index).unwrap();
    n.set_water_references(&water);
    n.set_unknown(42);
    let mut saved = reload(&mut nif);
    assert_eq!(node(&saved).water_references(), water);
    assert_eq!(node(&saved).unknown(), 42);

    let index = saved.blocks_of::<BsWeakReferenceNode>().indices().next().unwrap();
    saved.block_of_mut::<BsWeakReferenceNode>(index).unwrap().set_water_references(&water[1..]);
    assert_eq!(node(&reload(&mut saved)).water_references(), &water[1..]);
}
