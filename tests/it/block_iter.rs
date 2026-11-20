//! Tests for BlockIter: indexed(), indices(), of() and the iterators built on it.

use crate::common::load;

use nifly::{
    extra_data::BsxFlags,
    file::{Index, NifFile},
    iter::BlockIter,
    nodes::NiNode,
    objects::{NiAvObject, NiObject, NiObjectNet, NiType},
    shaders::{BsShaderTextureSet, NiShader},
};

const ASSETS: [&str; 5] = ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"];

/// Checks `indexed()` and `indices()` against plain iteration, and that each index is the
/// item's own block.
fn check<T: NiType, I: Iterator<Item = Index> + Clone>(
    nif: &NifFile,
    iter: BlockIter<'_, T, I>,
    what: &str,
) {
    let plain: Vec<*const T> = iter.clone().map(|b| b as *const T).collect();
    let indexed: Vec<(Index, *const T)> = iter
        .clone()
        .indexed()
        .map(|(i, b)| (i, b as *const T))
        .collect();
    let indices: Vec<Index> = iter.indices().collect();

    assert_eq!(
        indexed.iter().map(|&(_, b)| b).collect::<Vec<_>>(),
        plain,
        "{what}"
    );
    assert_eq!(
        indexed.iter().map(|&(i, _)| i).collect::<Vec<_>>(),
        indices,
        "{what}"
    );
    for (i, b) in indexed {
        let at_index = nif.block(i).unwrap().downcast::<T>().unwrap() as *const T;
        assert_eq!(at_index, b, "{what}: index {i}");
    }
}

#[test]
fn every_iterator_reports_its_indices() {
    for asset in ASSETS {
        let nif = load(asset);
        check(&nif, nif.blocks(), &format!("{asset} blocks"));
        check(&nif, nif.nodes(), &format!("{asset} nodes"));
        check(&nif, nif.shapes(), &format!("{asset} shapes"));
        check(&nif, nif.tree(), &format!("{asset} tree"));
        for (i, block) in nif.blocks().indexed() {
            check(
                &nif,
                nif.child_blocks(block),
                &format!("{asset} child_blocks({i})"),
            );
            check(
                &nif,
                nif.child_blocks_of::<NiShader>(block),
                &format!("{asset} child_blocks_of({i})"),
            );
            check(&nif, nif.subtree(block), &format!("{asset} subtree({i})"));
            if let Some(obj) = block.downcast::<NiObjectNet>() {
                check(
                    &nif,
                    nif.view_of(obj).unwrap().extra_data(),
                    &format!("{asset} extra_data({i})"),
                );
            }
            if let Some(node) = block.downcast::<NiNode>() {
                check(
                    &nif,
                    nif.view_of(node).unwrap().children(),
                    &format!("{asset} children({i})"),
                );
                check(
                    &nif,
                    nif.view_of(node).unwrap().children_of::<NiNode, _>(),
                    &format!("{asset} children_of({i})"),
                );
            }
        }
    }
}

#[test]
fn known_indices() {
    let ico = load("Icosphere");
    // Root -> Indirect1 -> Indirect2 -> shape.
    let mut chain = vec![];
    let mut node = ico.root_node().unwrap().get();
    loop {
        let children: Vec<_> = ico.view_of(node).unwrap().children().indexed().collect();
        let [(i, child)] = children[..] else { break };
        chain.push(i);
        match child.downcast::<NiNode>() {
            Some(n) => node = n,
            None => break,
        }
    }
    assert_eq!(chain, [1, 2, 3]);

    let suzanne = load("Suzanne");
    let root = suzanne.root_node().unwrap();
    assert_eq!(root.extra_data().indices().collect::<Vec<_>>(), [1]);
    assert_eq!(
        suzanne.tree().indices().collect::<Vec<_>>(),
        [0, 1, 6, 5, 4, 3, 2, 7, 8, 9]
    );
}

#[test]
fn of_retypes_the_same_source() {
    let nif = load("Suzanne");
    let shape = nif.shapes().next().unwrap();
    let via_of: Vec<_> = nif.child_blocks(shape).of::<NiShader>().indices().collect();
    let direct: Vec<_> = nif.child_blocks_of::<NiShader>(shape).indices().collect();
    assert_eq!(via_of, direct);
    assert_eq!(via_of, [8]);

    let nodes: Vec<_> = nif.blocks().of::<NiNode>().indices().collect();
    assert_eq!(nodes, nif.nodes().indices().collect::<Vec<_>>());
    let bsx: Vec<_> = nif.blocks().of::<BsxFlags>().indices().collect();
    assert_eq!(bsx, [1]);
}

#[test]
fn reverse_and_clone() {
    let nif = load("Suzanne");
    let forward: Vec<_> = nif.blocks().indices().collect();
    let mut backward: Vec<_> = nif.blocks().indices().rev().collect();
    backward.reverse();
    assert_eq!(forward, backward);

    let root = nif.root_node().unwrap();
    let fwd: Vec<_> = nif.child_blocks(&root).indexed().map(|(i, _)| i).collect();
    let mut rev: Vec<_> = nif
        .child_blocks(&root)
        .indexed()
        .rev()
        .map(|(i, _)| i)
        .collect();
    rev.reverse();
    assert_eq!(fwd, rev);
    assert_eq!(
        nif.child_blocks(&root).next_back().map(|b| b.block_name()),
        Some("BSTriShape")
    );

    let iter = nif.shapes();
    assert_eq!(iter.clone().count(), iter.count());
}

#[test]
fn av_object_and_object_net_downcasts() {
    let nif = load("Suzanne");
    let root: &NiObject = nif.root_node().unwrap().get();
    assert!(root.downcast::<NiAvObject>().is_some());
    assert!(root.downcast::<NiObjectNet>().is_some());
    let texture_set = nif.blocks_of::<BsShaderTextureSet>().next().unwrap();
    let texture_set: &NiObject = texture_set;
    assert!(texture_set.downcast::<NiAvObject>().is_none());
    assert!(texture_set.downcast::<NiObjectNet>().is_none());
    // Extra data has a name but isn't an NiObjectNET.
    let bsx: &NiObject = nif.blocks_of::<BsxFlags>().next().unwrap();
    assert!(bsx.downcast::<NiObjectNet>().is_none());
    assert_eq!(nif.blocks_of::<NiAvObject>().count(), 2);
}

#[test]
fn rename_without_block_index() {
    let mut nif = load("Suzanne");
    let shapes: Vec<_> = nif.shapes().indices().collect();
    for (n, i) in shapes.into_iter().enumerate() {
        nif.block_of_mut::<nifly::objects::NiObjectNet>(i)
            .unwrap()
            .set_name(format!("Shape{n}"));
    }
    assert_eq!(nif.shape_names().collect::<Vec<_>>(), [b"Shape0"]);
}
