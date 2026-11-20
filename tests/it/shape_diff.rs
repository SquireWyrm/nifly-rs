//! Tests for NifFile::shape_vertex_diff and NifFile::shape_uv_diff.

use crate::common::{load, upstream};

use nifly::{file::NifFile, geometry::NiShape, object3d::Vec3};

fn first(nif: &NifFile) -> &NiShape {
    nif.shapes().next().unwrap()
}

#[test]
fn vertex_diff() {
    let nif = load("Cube");
    let s = first(&nif);
    let mut target = nif.view_of(s).unwrap().vertices().unwrap();
    assert!(
        nif.view_of(s)
            .unwrap()
            .vertex_diff(&target)
            .unwrap()
            .is_empty()
    );

    target[2] += Vec3::new(1.0, 0.0, -0.5);
    target[9].y += 0.25;
    // Under the threshold on every axis.
    target[5] += Vec3::new(0.00005, -0.00005, 0.0);
    let diff = nif.view_of(s).unwrap().vertex_diff(&target).unwrap();
    assert_eq!(diff.len(), 2);
    assert_eq!(diff[0].0, 2);
    assert!(diff[0].1.distance(Vec3::new(1.0, 0.0, -0.5)) < 1e-5);
    assert_eq!(diff[1].0, 9);
    assert!(diff[1].1.distance(Vec3::new(0.0, 0.25, 0.0)) < 1e-5);

    // Applying the offsets reaches the target.
    let mut moved = nif.view_of(s).unwrap().vertices().unwrap();
    for (i, d) in diff {
        moved[i] += d;
    }
    assert!(
        moved
            .iter()
            .zip(&target)
            .all(|(a, b)| a.distance(*b) < 1e-4)
    );
}

#[test]
fn uv_diff() {
    let nif = load("Cube");
    let s = first(&nif);
    let mut target = nif.view_of(s).unwrap().uvs().unwrap();
    assert!(nif.view_of(s).unwrap().uv_diff(&target).unwrap().is_empty());

    target[0][0] += 0.5;
    target[13][1] -= 0.125;
    target[7][1] += 0.00005;
    let diff = nif.view_of(s).unwrap().uv_diff(&target).unwrap();
    assert_eq!(diff.iter().map(|(i, _)| *i).collect::<Vec<_>>(), [0, 13]);
    assert!((diff[0].1[0] - 0.5).abs() < 1e-5 && diff[0].1[1] == 0.0);
    assert!(diff[1].1[0] == 0.0 && (diff[1].1[1] + 0.125).abs() < 1e-5);
}

#[test]
fn none_without_data_or_matching_length() {
    let nif = load("Cube");
    let s = first(&nif);
    let vertices = nif.view_of(s).unwrap().vertices().unwrap();
    let uvs = nif.view_of(s).unwrap().uvs().unwrap();
    assert_eq!(nif.view_of(s).unwrap().vertex_diff(&vertices[1..]), None);
    assert_eq!(
        nif.view_of(s)
            .unwrap()
            .vertex_diff(&[vertices.clone(), vertices].concat()),
        None
    );
    assert_eq!(nif.view_of(s).unwrap().uv_diff(&uvs[1..]), None);
    assert_eq!(nif.view_of(s).unwrap().uv_diff(&[]), None);

    // Particles have neither.
    let nif = upstream("TestNifFile_Particles_MW");
    let s = nif.shapes().find(|s| s.name() == "Blizzard").unwrap();
    assert_eq!(nif.view_of(s).unwrap().vertex_diff(&[]), None);
    assert_eq!(nif.view_of(s).unwrap().uv_diff(&[]), None);
}
