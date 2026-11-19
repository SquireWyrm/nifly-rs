//! Tests against assets/Icosphere.nif: a chain of nested, rotated nodes.
//!
//! Block layout:
//!   0 BSFadeNode "Icosphere" -> [1]   identity
//!   1 NiNode "Indirect1" -> [2]       t = (1.342, -0.128, 0.594), rotated, scale 0.5
//!   2 NiNode "Indirect2" -> [3]       t = (-1.310, 0.099, 0.575), rotated, scale 1.25
//!   3 BSTriShape "IcosphereMesh"      t = (-2.155, 0.192, 0.128)
//!
//! The node scales were set in NifSkope, because PyNifly's exporter drops NiNode scale.
//! The shape is named "IcosphereMesh". (NifSkope renames a shape that shares the root's name.)
//! Indirect2 is the node whose global transform needs real composition, because its parent
//! (Indirect1) isn't identity.

use std::{fs::File, ptr};

use nifly::{
    file::NifFile,
    geometry::NiShape,
    nodes::NiNode,
    object3d::{Matrix3, Transform, Vec3},
    objects::{NiAvObject, NiObject},
};

const EPS: f32 = 1e-3;

fn icosphere() -> NifFile {
    NifFile::load(File::open("assets/Icosphere.nif").unwrap(), None).unwrap()
}

fn block(nif: &NifFile, i: usize) -> &NiObject {
    nif.blocks().nth(i).unwrap()
}

fn node(nif: &NifFile, i: usize) -> &NiNode {
    block(nif, i).downcast::<NiNode>().unwrap()
}

fn shape(nif: &NifFile) -> &NiShape {
    nif.blocks_of::<NiShape>().next().unwrap()
}

fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

fn assert_vec_eq(a: Vec3, b: Vec3) {
    let close = (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS && (a.z - b.z).abs() < EPS;
    assert!(close, "{a:?} != {b:?}");
}

fn assert_rotation_eq(actual: Matrix3, expected: [Vec3; 3]) {
    for (row, expected) in actual.rows.into_iter().zip(expected) {
        assert_vec_eq(row, expected);
    }
}

fn assert_transform_eq(a: &Transform, b: &Transform) {
    assert_vec_eq(a.translation, b.translation);
    assert_rotation_eq(a.rotation, b.rotation.rows);
    assert!((a.scale - b.scale).abs() < EPS, "scale {} != {}", a.scale, b.scale);
}

/// Independent reimplementation of nifly's `MatTransform::ComposeTransforms`:
/// `parent ∘ child`, with row-major rotation matrices applied to column vectors.
fn compose(parent: &Transform, child: &Transform) -> Transform {
    let dot = |a: Vec3, b: Vec3| a.x * b.x + a.y * b.y + a.z * b.z;
    let mul_vec = |m: &[Vec3; 3], p: Vec3| v(dot(m[0], p), dot(m[1], p), dot(m[2], p));
    let col = |m: &[Vec3; 3], c: usize| {
        let pick = |r: Vec3| [r.x, r.y, r.z][c];
        v(pick(m[0]), pick(m[1]), pick(m[2]))
    };

    let (p, c) = (&parent.rotation.rows, &child.rotation.rows);
    let rows = p.map(|row| v(dot(row, col(c, 0)), dot(row, col(c, 1)), dot(row, col(c, 2))));
    let s = parent.scale;
    let scaled = v(child.translation.x * s, child.translation.y * s, child.translation.z * s);
    let offset = mul_vec(&parent.rotation.rows, scaled);
    let t = parent.translation;

    Transform {
        translation: v(t.x + offset.x, t.y + offset.y, t.z + offset.z),
        rotation: Matrix3 { rows },
        scale: parent.scale * child.scale,
    }
}

/// Global transform of `obj`, composed by walking `parent_node` up to the root.
fn global_via_parents(nif: &NifFile, obj: &NiAvObject) -> Transform {
    let mut global = obj.transform();
    let mut parent = nif.view_of(obj).and_then(|v| v.parent());
    while let Some(p) = parent {
        global = compose(&p.transform(), &global);
        parent = p.parent();
    }
    global
}

#[test]
fn parent_chain() {
    let nif = icosphere();
    let root = nif.root_node().unwrap();
    let indirect1 = node(&nif, 1);
    let indirect2 = node(&nif, 2);

    let parent = |obj: &NiNode| nif.view_of(obj).and_then(|v| v.parent()).unwrap().get();
    assert!(ptr::eq(nif.view_of(shape(&nif)).and_then(|v| v.parent()).unwrap().get(), indirect2));
    assert!(ptr::eq(parent(indirect2), indirect1));
    assert!(ptr::eq(parent(indirect1), root.get()));
    assert!(root.parent().is_none());
}

#[test]
fn transform_to_global_composes() {
    let nif = icosphere();
    let global = nif.node_transform_to_global("Indirect2").unwrap();

    // Indirect1 ∘ Indirect2, since the root is identity. Checked by hand, e.g.
    //   scale  = s1 * s2                    = 0.5 * 1.25                           = 0.625
    //   t.x    = t1.x + s1 * (R1[0] · t2)   = 1.3418 + 0.5 * (0.2443 - 0.0897 + 0.2175) ≈ 1.5279
    //   r[0].x = R1[0] · R2 column 0        = -0.1319 + 0.6400 + 0.0156             ≈ 0.5237
    assert!((global.scale - 0.625).abs() < EPS, "scale {}", global.scale);
    assert_vec_eq(global.translation, v(1.528, -0.616, 1.085));
    assert_rotation_eq(
        global.rotation,
        [v(0.524, -0.849, -0.071), v(0.682, 0.368, 0.632), v(-0.510, -0.379, 0.772)],
    );

    // And it differs from the local transform, so composition actually happened.
    let local = nif.node_transform_to_parent("Indirect2").unwrap();
    assert!((global.translation.x - local.translation.x).abs() > 0.5);
}

#[test]
fn transform_to_global_matches_parent_chain() {
    let nif = icosphere();
    for (name, i) in [("Icosphere", 0), ("Indirect1", 1), ("Indirect2", 2)] {
        let expected = global_via_parents(&nif, node(&nif, i));
        let actual = nif.node_transform_to_global(name).unwrap();
        assert_transform_eq(&actual, &expected);
    }
}
