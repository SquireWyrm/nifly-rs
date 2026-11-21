//! Tests for node subclass fields, tree bones and NifFile::add_node_of.

use crate::common::{UPSTREAM, first, first_index, open, reload};

use nifly::{
    file::{Index, NifFile, NifVersion},
    nodes::{
        BillboardMode, BsBlastNode, BsOrderedNode, BsRangeNode, BsTreeNode, BsValueNode,
        NiBillboardNode, NiNode, NiSortAdjustNode, NiSwitchNode, SortingMode, SwitchNodeFlags,
        ValueNodeFlags,
    },
    object3d::Transform,
};

fn load(name: &str) -> NifFile {
    open(format!("{UPSTREAM}/{name}.nif"))
}

fn add<T: nifly::nodes::NewNode>(nif: &mut NifFile, name: &str) -> Index {
    nif.add_node_of::<T>(name, Transform::default(), None)
        .unwrap()
}

#[test]
fn reads_existing() {
    let nif = load("TestNifFile_OrderedNode_SE");
    let ordered = first::<BsOrderedNode>(&nif);
    assert_eq!(ordered.alpha_sort_bound(), [0.0; 4]);
    assert!(ordered.is_static_bound());
    let billboard = nif.find_block_by_name::<NiBillboardNode>("jet01").unwrap();
    assert_eq!(billboard.mode(), Some(BillboardMode::RotateAboutUp));
}

#[test]
fn skyrim_round_trip() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let value = add::<BsValueNode>(&mut nif, "Value");
    let ordered = add::<BsOrderedNode>(&mut nif, "Ordered");
    let range = add::<BsBlastNode>(&mut nif, "Range");
    let billboard = add::<NiBillboardNode>(&mut nif, "Billboard");

    let v = nif.block_of_mut::<BsValueNode>(value).unwrap();
    v.set_value(-7);
    v.set_flags(ValueNodeFlags::USE_PLAYER_ADJUST);
    let o = nif.block_of_mut::<BsOrderedNode>(ordered).unwrap();
    o.set_alpha_sort_bound([1.0, 2.0, 3.0, 4.0]);
    o.set_static_bound(true);
    let r = nif.block_of_mut::<BsBlastNode>(range).unwrap();
    r.set_min(1);
    r.set_max(5);
    r.set_current(3);
    nif.block_of_mut::<NiBillboardNode>(billboard)
        .unwrap()
        .set_mode(BillboardMode::RotateAboutUp2);

    let saved = reload(&mut nif);
    let v = saved.find_block_by_name::<BsValueNode>("Value").unwrap();
    assert_eq!(
        (v.value(), v.flags()),
        (-7, ValueNodeFlags::USE_PLAYER_ADJUST)
    );
    let o = saved
        .find_block_by_name::<BsOrderedNode>("Ordered")
        .unwrap();
    assert_eq!(o.alpha_sort_bound(), [1.0, 2.0, 3.0, 4.0]);
    assert!(o.is_static_bound());
    let r = saved.find_block_by_name::<BsBlastNode>("Range").unwrap();
    assert_eq!((r.min(), r.max(), r.current()), (1, 5, 3));
    let b = saved
        .find_block_by_name::<NiBillboardNode>("Billboard")
        .unwrap();
    assert_eq!(b.mode(), Some(BillboardMode::RotateAboutUp2));
}

#[test]
fn oblivion_round_trip() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let switch = add::<NiSwitchNode>(&mut nif, "Switch");
    let sort = add::<NiSortAdjustNode>(&mut nif, "Sort");

    let s = nif.block_of_mut::<NiSwitchNode>(switch).unwrap();
    s.set_flags(SwitchNodeFlags::all());
    s.set_active_index(2);
    nif.block_of_mut::<NiSortAdjustNode>(sort)
        .unwrap()
        .set_sorting_mode(SortingMode::Off);

    let saved = reload(&mut nif);
    let s = saved.find_block_by_name::<NiSwitchNode>("Switch").unwrap();
    assert_eq!((s.flags(), s.active_index()), (SwitchNodeFlags::all(), 2));
    let sort = saved
        .find_block_by_name::<NiSortAdjustNode>("Sort")
        .unwrap();
    assert_eq!(sort.sorting_mode(), Some(SortingMode::Off));
}

#[test]
fn range_node_reads_as_unknown() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    add::<BsRangeNode>(&mut nif, "Range");
    let saved = reload(&mut nif);
    assert!(saved.has_unknown());
    assert_eq!(saved.blocks_of::<BsRangeNode>().count(), 0);
}

#[test]
fn morrowind_billboard_mode_not_saved() {
    let mut nif = load("TestNifFile_Billboard_MW");
    let index = first_index::<NiBillboardNode>(&nif);
    nif.block_of_mut::<NiBillboardNode>(index)
        .unwrap()
        .set_mode(BillboardMode::RigidFaceCenter);
    let saved = reload(&mut nif);
    let b = first::<NiBillboardNode>(&saved);
    assert_eq!(b.mode(), Some(BillboardMode::AlwaysFaceCamera));
}

#[test]
fn tree_bones() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let tree = add::<BsTreeNode>(&mut nif, "Tree");
    let a = nif.add_node("A", Transform::default(), Some(tree)).unwrap();
    let b = nif.add_node("B", Transform::default(), Some(tree)).unwrap();
    let mut edit = nif.edit::<BsTreeNode>(tree).unwrap();
    assert!(edit.set_bones_1(&[a, b]));
    assert!(edit.set_bones_2(&[b]));

    let names = |nif: &NifFile, list: u8| -> Vec<String> {
        let tree = nif.blocks_of::<BsTreeNode>().views().next().unwrap();
        let bones = if list == 1 {
            tree.bones_1()
        } else {
            tree.bones_2()
        };
        bones.map(|n| n.name().to_string()).collect()
    };
    let saved = reload(&mut nif);
    assert_eq!(names(&saved, 1), ["A", "B"]);
    assert_eq!(names(&saved, 2), ["B"]);

    // Clearing a list.
    let tree = first_index::<BsTreeNode>(&nif);
    assert!(nif.edit::<BsTreeNode>(tree).unwrap().set_bones_1(&[]));
    assert!(names(&nif, 1).is_empty());
}

#[test]
fn tree_bones_reject_invalid() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let tree = add::<BsTreeNode>(&mut nif, "Tree");
    let plain = nif.add_node("Plain", Transform::default(), None).unwrap();
    let extra = nif
        .edit::<nifly::objects::NiAvObject>(tree)
        .and_then(|mut e| e.assign_extra_data(nifly::extra_data::ExtraFloat("F", 1.0)))
        .unwrap();
    let count = nif.blocks().count();

    let mut edit = nif.edit::<BsTreeNode>(tree).unwrap();
    assert!(edit.set_bones_1(&[plain]));
    // A bone that isn't a node, and out-of-range bones.
    assert!(!edit.set_bones_1(&[plain, extra]));
    assert!(!edit.set_bones_1(&[count]));
    assert!(!edit.set_bones_1(&[usize::MAX]));
    assert_eq!(edit.view().bones_1().count(), 1);
    // Only tree nodes have bone lists.
    assert!(nif.edit::<BsTreeNode>(plain).is_none());
}

#[test]
fn add_node_of_parents() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let range = add::<BsRangeNode>(&mut nif, "Range");
    let child = nif
        .add_node_of::<NiNode>("Child", Transform::default(), Some(range))
        .unwrap();
    let range_node = nif.block_of::<BsRangeNode>(range).unwrap();
    assert_eq!(
        nif.view_of(range_node)
            .unwrap()
            .children()
            .indices()
            .collect::<Vec<_>>(),
        [child]
    );

    let blocks = nif.blocks().count();
    let extra = nif
        .edit::<nifly::objects::NiAvObject>(range)
        .and_then(|mut e| e.assign_extra_data(nifly::extra_data::ExtraFloat("F", 1.0)))
        .unwrap();
    assert_eq!(
        nif.add_node_of::<BsRangeNode>("X", Transform::default(), Some(extra)),
        None
    );
    assert_eq!(
        nif.add_node_of::<BsRangeNode>("X", Transform::default(), Some(usize::MAX)),
        None
    );
    assert_eq!(nif.blocks().count(), blocks + 1);
}
