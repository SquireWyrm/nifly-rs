use std::fs;
use std::path::PathBuf;

fn main() {
    let nifly_dir = PathBuf::from("vendor/nifly");
    let nifly_src = nifly_dir.join("src");
    let nifly_inc = nifly_dir.join("include");
    let nifly_ext = nifly_dir.join("external"); // Required for half.hpp and Miniball.hpp

    let mut cpp_files = vec![];
    for entry in fs::read_dir(&nifly_src).expect("Failed to read nifly/src") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            cpp_files.push(path);
        }
    }

    for entry in fs::read_dir("src").expect("Failed to read src") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            cpp_files.push(path);
        }
    }

    let bridge_files = [
        "src/animation.rs",
        "src/collision.rs",
        "src/effects.rs",
        "src/extra_data.rs",
        "src/extra_data_types.rs",
        "src/geometry.rs",
        "src/nif_file.rs",
        "src/nodes.rs",
        "src/object_3d.rs",
        "src/objects.rs",
        "src/particles.rs",
        "src/read_bridge.rs",
        "src/shaders.rs",
        "src/skin.rs",
        "src/write_bridge.rs",
    ];

    cxx_build::bridges(bridge_files)
        .files(&cpp_files)
        .include("include")
        .include(&nifly_inc)
        .include(&nifly_ext)
        .std("c++17")
        .flag_if_supported("/bigobj")
        .flag_if_supported("/EHsc")
        .compile("nifly-bridge");

    build_deps::rerun_if_changed_paths("src/*").unwrap();
    build_deps::rerun_if_changed_paths("src").unwrap();
    build_deps::rerun_if_changed_paths("include/*").unwrap();
    build_deps::rerun_if_changed_paths("include").unwrap();
}
