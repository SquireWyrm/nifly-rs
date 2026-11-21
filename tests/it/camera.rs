//! Tests for NiCamera, NifFile::add_camera and the camera's scene.

use crate::common::{reload_with, upstream};

use nifly::{
    file::{NifFile, NifVersion, SaveOptions},
    nodes::{Frustum, NiCamera, NiNode, Viewport},
    object3d::{Transform, Vec3},
};

/// Keeps block indices the same.
const KEEP: Option<SaveOptions> = Some(SaveOptions {
    optimize: false,
    sort_blocks: false,
});

const FRUSTUM: Frustum = Frustum {
    left: -1.0,
    right: 2.0,
    top: 3.0,
    bottom: -4.0,
    near: 5.0,
    far: 600.0,
};
const VIEWPORT: Viewport = Viewport {
    left: 0.25,
    right: 0.75,
    top: 0.5,
    bottom: 0.125,
};

fn set_fields(nif: &mut NifFile, camera: usize) {
    let mut edit = nif.edit::<NiCamera>(camera).unwrap();
    let c = &mut *edit;
    c.set_raw_flags(7);
    c.set_frustum(FRUSTUM);
    c.set_orthographic(true);
    c.set_viewport(VIEWPORT);
    c.set_lod_adjust(2.5);
}

#[test]
fn add_camera_defaults() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let root = nif.root_index().unwrap();
    let count = nif.blocks().count();
    let transform = Transform {
        translation: Vec3::new(1.0, 2.0, 3.0),
        ..Transform::default()
    };
    let camera = nif.add_camera("Camera", transform, None).unwrap();
    assert_eq!(camera, count);
    assert_eq!(
        nif.view::<NiNode>(root)
            .unwrap()
            .children()
            .indices()
            .next_back(),
        Some(camera)
    );

    let c = nif.view::<NiCamera>(camera).unwrap();
    assert_eq!(c.name(), "Camera");
    assert_eq!(c.transform(), transform);
    assert_eq!(
        c.frustum(),
        Frustum {
            left: -0.63707,
            right: 0.63707,
            top: 0.385714,
            bottom: -0.385714,
            near: 1.0,
            far: 5000.0
        }
    );
    assert_eq!(
        c.viewport(),
        Viewport {
            left: 0.0,
            right: 1.0,
            top: 1.0,
            bottom: 0.0
        }
    );
    assert_eq!(
        (c.raw_flags(), c.orthographic(), c.lod_adjust()),
        (0, false, 1.0)
    );
    assert!(c.scene().is_none());
}

#[test]
fn add_camera_under_a_node() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let camera = nif
        .add_camera("Cam", Transform::default(), Some(2))
        .unwrap();
    assert_eq!(
        nif.view::<NiCamera>(camera)
            .unwrap()
            .parent()
            .unwrap()
            .index(),
        2
    );

    let count = nif.blocks().count();
    // Block 3 isn't a node; the others don't exist.
    for bad in [3, count, usize::MAX] {
        assert!(
            nif.add_camera("Cam", Transform::default(), Some(bad))
                .is_none(),
            "{bad}"
        );
    }
    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn fields_survive_save() {
    for (name, version_saves_flags) in [
        ("TestNifFile_Static_MW", false),
        ("TestNifFile_Skinned_OB", true),
        ("TestNifFile_Static_SE", true),
    ] {
        let mut nif = upstream(name);
        let camera = nif
            .add_camera("Camera", Transform::default(), None)
            .unwrap();
        set_fields(&mut nif, camera);

        let saved = reload_with(&mut nif, KEEP);
        let c = saved.view::<NiCamera>(camera).unwrap();
        assert_eq!(c.name(), "Camera", "{name}");
        assert_eq!(
            (c.frustum(), c.viewport(), c.lod_adjust()),
            (FRUSTUM, VIEWPORT, 2.5),
            "{name}"
        );
        // Morrowind doesn't save the flags or the projection type.
        if version_saves_flags {
            assert_eq!((c.raw_flags(), c.orthographic()), (7, true), "{name}");
        } else {
            assert_eq!((c.raw_flags(), c.orthographic()), (0, false), "{name}");
        }
    }
}

#[test]
fn scene() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let camera = nif
        .add_camera("Cam", Transform::default(), Some(2))
        .unwrap();
    let mut edit = nif.edit::<NiCamera>(camera).unwrap();
    // Block 4 is a texture; the others don't exist.
    for bad in [4, 100, usize::MAX] {
        assert!(!edit.set_scene(Some(bad)), "{bad}");
    }
    assert!(edit.scene().is_none());

    // A shape works, as does a node.
    assert!(edit.set_scene(Some(5)));
    assert_eq!(edit.view().scene().map(|s| s.index()), Some(5));
    assert!(edit.set_scene(Some(10)));
    edit.scene().unwrap().set_name("Scene");
    assert_eq!(
        nif.view::<NiCamera>(camera)
            .unwrap()
            .scene()
            .unwrap()
            .name(),
        "Scene"
    );
    let saved = reload_with(&mut nif, KEEP);
    assert_eq!(
        saved
            .view::<NiCamera>(camera)
            .unwrap()
            .scene()
            .map(|s| s.index()),
        Some(10)
    );

    let mut edit = nif.edit::<NiCamera>(camera).unwrap();
    assert!(edit.set_scene(None));
    assert!(edit.view().scene().is_none());
}

#[test]
fn scene_can_be_an_ancestor() {
    // A camera usually renders the scene it's in, so its scene ref points back up the tree.
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    let node = nif.add_node("Node", Transform::default(), None).unwrap();
    let camera = nif
        .add_camera("Camera", Transform::default(), Some(node))
        .unwrap();
    assert!(nif.edit::<NiCamera>(camera).unwrap().set_scene(Some(root)));

    // Optimizing and sorting still finish, and keep the ref.
    let saved = reload_with(&mut nif, None);
    let camera = saved.blocks_of::<NiCamera>().views().next().unwrap();
    assert_eq!(
        camera.scene().map(|s| s.index()),
        saved.root_node().map(|r| r.index())
    );
    assert_eq!(camera.parent().unwrap().name(), "Node");
}
