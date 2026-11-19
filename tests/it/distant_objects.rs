//! Tests for BSDistantObjectInstancedNode instances and texture arrays.

use crate::common::{reload};

use nifly::{
    file::{Index, NifFile, NifVersion},
    nodes::{BsDistantObjectInstancedNode, DistantObjectInstance, DistantObjectUnknown, ResourceId},
    object3d::Transform,
};

fn node(nif: &NifFile) -> &BsDistantObjectInstancedNode {
    nif.blocks_of::<BsDistantObjectInstancedNode>().next().unwrap()
}

fn setup() -> (NifFile, Index) {
    let mut nif = NifFile::create(NifVersion::FALLOUT_76);
    let node = nif.add_node_of::<BsDistantObjectInstancedNode>("Distant", Transform::default(), None).unwrap();
    (nif, node)
}

fn instance(seed: f32, copies: usize) -> DistantObjectInstance {
    let matrix = |k: f32| std::array::from_fn(|r| std::array::from_fn(|c| k + (r * 4 + c) as f32));
    DistantObjectInstance {
        resource: ResourceId { file_hash: seed as u32, extension: *b"nif\0", dir_hash: 0xDEADBEEF },
        unknown: (0..copies)
            .map(|i| DistantObjectUnknown { unknown_1: (seed as u64) << 40 | i as u64, unknown_2: u32::MAX - i as u32 })
            .collect(),
        transforms: (0..copies).map(|i| matrix(seed + i as f32 * 100.0)).collect(),
    }
}

#[test]
fn instances_round_trip() {
    let (mut nif, index) = setup();
    let mut n = nif.block_of_mut::<BsDistantObjectInstancedNode>(index).unwrap();
    n.as_mut().push_instance(&instance(1.0, 2));
    n.as_mut().push_instance(&instance(2.0, 0));
    n.push_instance(&instance(3.0, 1));

    let mut saved = reload(&mut nif);
    let n = node(&saved);
    assert_eq!(n.instance_count(), 3);
    assert_eq!(n.instance(0), Some(instance(1.0, 2)));
    assert_eq!(n.instance(1), Some(instance(2.0, 0)));
    assert_eq!(n.instance(3), None);

    let index = saved.blocks_of::<BsDistantObjectInstancedNode>().indices().next().unwrap();
    let mut n = saved.block_of_mut::<BsDistantObjectInstancedNode>(index).unwrap();
    assert!(n.as_mut().set_instance(1, &instance(5.0, 1)));
    assert!(n.as_mut().remove_instance(0));
    assert!(!n.as_mut().remove_instance(2));
    assert!(!n.set_instance(2, &instance(6.0, 1)));
    let n = node(&saved);
    assert_eq!((n.instance(0), n.instance(1)), (Some(instance(5.0, 1)), Some(instance(3.0, 1))));
}

#[test]
fn texture_arrays_round_trip() {
    let (mut nif, index) = setup();
    let mut n = nif.block_of_mut::<BsDistantObjectInstancedNode>(index).unwrap();
    assert!(n.as_mut().set_texture_arrays(1, [vec!["a.dds", "b.dds"], vec!["c.dds"]]));
    assert!(!n.as_mut().set_texture_arrays(3, [["x.dds"]]));
    assert_eq!(n.texture_arrays_unknown(1), Some(1));
    assert!(n.as_mut().set_texture_arrays_unknown(1, 7));
    assert!(!n.set_texture_arrays_unknown(3, 7));

    let saved = reload(&mut nif);
    let n = node(&saved);
    assert_eq!(n.texture_arrays(0).unwrap(), Vec::<Vec<&[u8]>>::new());
    assert_eq!(n.texture_arrays(1).unwrap(), [vec!["a.dds", "b.dds"], vec!["c.dds"]]);
    assert_eq!(n.texture_arrays(3), None);
    assert_eq!((n.texture_arrays_unknown(1), n.texture_arrays_unknown(3)), (Some(7), None));
}
