use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let nifly_dir = PathBuf::from("vendor/nifly");
    let nifly_src = nifly_dir.join("src");
    let nifly_inc = nifly_dir.join("include");
    let nifly_ext = nifly_dir.join("external"); // Required for half.hpp and Miniball.hpp

    let cpp_files_in = |dir: &Path| -> Vec<PathBuf> {
        let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read {dir:?}: {e}"));
        entries
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("cpp"))
            .collect()
    };
    let common = |build: &mut cc::Build| {
        build
            .include(&nifly_inc)
            .include(&nifly_ext)
            .std("c++17")
            .flag_if_supported("/bigobj")
            .flag_if_supported("/EHsc")
            // As nifly's own build: no fused multiply-adds, which round differently, so computed
            // values such as bounds match across compilers. MSVC doesn't fuse by default.
            .flag_if_supported("-ffp-contract=off");
    };

    let bridge_files = [
        "src/animation.rs",
        "src/collision.rs",
        "src/effects.rs",
        "src/extra_data.rs",
        "src/extra_data_types.rs",
        "src/geometry.rs",
        "src/nif_file.rs",
        "src/nif_file_extra_data.rs",
        "src/nif_file_refs.rs",
        "src/nodes.rs",
        "src/object_3d.rs",
        "src/objects.rs",
        "src/particles.rs",
        "src/read_bridge.rs",
        "src/shaders.rs",
        "src/skin.rs",
        "src/write_bridge.rs",
    ];

    // The bridges and shims. Built first, so it comes before nifly on the link line.
    let mut bridge = cxx_build::bridges(bridge_files);
    common(&mut bridge);
    // Declares the Vec specializations before any shim uses them (see the header). Not for
    // nifly's sources, whose `using namespace nifly` clashes with its declarations.
    if bridge.get_compiler().is_like_msvc() {
        bridge.flag("/FIrust_vec.h");
    } else {
        bridge.flag("-include").flag("rust_vec.h");
    }
    bridge
        .files(cpp_files_in(Path::new("src")))
        .include("include")
        .compile("nifly-bridge");

    let mut nifly = cc::Build::new();
    common(&mut nifly);
    nifly
        .cpp(true)
        .files(cpp_files_in(&nifly_src))
        .compile("nifly");

    // Cargo scans each directory's whole tree, so a submodule bump rebuilds nifly too.
    for dir in ["src", "include", "vendor/nifly/src", "vendor/nifly/include"] {
        println!("cargo:rerun-if-changed={dir}");
    }
}
