//! Tests for the block handles: NifFile::view, NifFile::edit and BlockIter::views.

use nifly::{
    file::NifFile,
    geometry::NiShape,
    nodes::NiNode,
    objects::{NiAvObject, NiObject},
    shaders::NiMaterialProperty,
};

use crate::common::open;

fn load() -> NifFile {
    let path = "nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif";
    open(path)
}

#[test]
fn view_checks_index_and_type() {
    let nif = load();
    let root = nif.root_node().unwrap().index();
    let view = nif.view::<NiNode>(root).unwrap();
    assert_eq!(view.index(), root);
    assert_eq!(view.name(), nif.root_node().unwrap().name());
    assert!(std::ptr::eq(view.get(), nif.root_node().unwrap().get()));
    assert!(std::ptr::eq(view.file(), &nif));

    assert!(nif.view::<NiShape>(root).is_none());
    assert!(nif.view::<NiNode>(nif.blocks().count()).is_none());
    assert!(nif.view::<NiNode>(usize::MAX).is_none());
}

#[test]
fn view_casts() {
    let nif = load();
    let shape = nif.shapes().indices().next().unwrap();
    let view = nif.view::<NiShape>(shape).unwrap();
    let base = view.cast::<NiAvObject>().unwrap();
    assert_eq!(base.index(), shape);
    assert!(base.cast::<NiShape>().is_some());
    assert!(view.cast::<NiNode>().is_none());
    assert!(view.cast::<NiObject>().is_some());
}

#[test]
fn views_match_indexed() {
    let nif = load();
    let views: Vec<_> = nif
        .blocks_of::<NiAvObject>()
        .views()
        .map(|v| (v.index(), v.name().to_string()))
        .collect();
    let indexed: Vec<_> = nif
        .blocks_of::<NiAvObject>()
        .indexed()
        .map(|(i, b)| (i, b.name().to_string()))
        .collect();
    assert_eq!(views, indexed);
    assert!(!views.is_empty());
    assert_eq!(
        nif.blocks_of::<NiAvObject>()
            .views()
            .next_back()
            .map(|v| v.index()),
        indexed.last().map(|p| p.0)
    );
}

#[test]
fn edit_sets_and_casts() {
    let mut nif = load();
    let index = nif
        .blocks_of::<NiMaterialProperty>()
        .indices()
        .next()
        .unwrap();
    let mut edit = nif.edit::<NiMaterialProperty>(index).unwrap();
    edit.set_alpha(0.25);
    edit.set_glossiness(3.0);
    assert_eq!(edit.alpha(), 0.25);
    assert_eq!(edit.index(), index);
    // Derefs to the block, the same one the view has, and sees each change.
    assert_eq!((edit.alpha(), edit.glossiness()), (0.25, 3.0));
    assert!(std::ptr::eq(&*edit, edit.view().get()));
    edit.set_alpha(0.5);
    assert_eq!(edit.alpha(), 0.5);
    edit.set_alpha(0.25);

    // A failed cast gives the edit back.
    let edit = edit.into_cast::<NiNode>().unwrap_err();
    let edit = edit.into_cast::<NiObject>().unwrap();
    assert_eq!(edit.index(), index);
    assert_eq!(
        nif.block_of::<NiMaterialProperty>(index).unwrap().alpha(),
        0.25
    );

    assert!(nif.edit::<NiNode>(index).is_none());
    assert!(nif.edit::<NiNode>(usize::MAX).is_none());
}

#[test]
fn base_methods_reach_subtypes() {
    use nifly::{nodes::BsFadeNode, shaders::NiZBufferProperty};

    let mut nif = open("assets/Suzanne.nif");
    // The root is a BSFadeNode: node and object-net methods work on it without casting.
    let root = nif.view::<BsFadeNode>(0).unwrap();
    assert_eq!(root.children().count(), 1);
    assert!(root.extra_data().count() > 0);
    assert!(root.parent().is_none());
    let child = root.children().views().next().unwrap();
    assert_eq!(child.parent().unwrap().index(), 0);
    assert_eq!(root.upcast::<NiNode, _>().index(), 0);

    let mut edit = nif.edit::<BsFadeNode>(0).unwrap();
    let copy = edit.clone_node();
    let mut copy = nif.edit::<BsFadeNode>(copy).unwrap();
    assert!(copy.set_parent(Some(0)));
    copy.add_property::<NiZBufferProperty, _>();
    let extra = copy
        .assign_extra_data(nifly::extra_data::ExtraFloat("F", 1.0))
        .unwrap();
    let copy = copy.into_upcast::<NiAvObject, _>().index();
    let copy = nif.view::<NiAvObject>(copy).unwrap();
    assert_eq!(copy.parent().unwrap().index(), 0);
    assert!(copy.extra_data().indices().any(|i| i == extra));
}

#[test]
fn set_parent_rejects_cycles_and_non_nodes() {
    let mut nif = open("assets/Icosphere.nif");
    // Root -> Indirect1 (1) -> Indirect2 (2) -> shape (3).
    let mut indirect1 = nif.edit::<NiNode>(1).unwrap();
    assert!(!indirect1.set_parent(Some(2)), "into its own subtree");
    assert!(!indirect1.set_parent(Some(1)), "under itself");
    assert!(!indirect1.set_parent(Some(3)), "under a shape");
    assert!(!indirect1.set_parent(Some(usize::MAX)));
    assert_eq!(indirect1.parent().unwrap().index(), 0);
}

#[test]
fn foreign_blocks_are_not_contained() {
    let (a, b) = (load(), load());
    let root = b.root_node().unwrap();
    assert!(b.contains(&root));
    assert!(!a.contains(&root));
    assert!(a.view_of(root.get()).is_none());
}
