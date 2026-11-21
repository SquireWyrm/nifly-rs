# nifly

[![crates.io](https://img.shields.io/crates/v/nifly.svg)](https://crates.io/crates/nifly)
[![downloads](https://img.shields.io/crates/d/nifly.svg)](https://crates.io/crates/nifly)
[![docs.rs](https://img.shields.io/docsrs/nifly)](https://docs.rs/nifly)

Safe Rust bindings to [nifly](https://github.com/ousnius/nifly), for reading, editing and writing
NIF files from Bethesda's games: Morrowind, Oblivion, Fallout 3 and New Vegas, Skyrim (LE and SE),
Fallout 4, Fallout 76 and Starfield.

```toml
[dependencies]
nifly = "0.1"
```

The API may still change between 0.x versions.

## Building

- Rust with edition 2024 support
- A C++17 compiler: nifly's C++ sources are built with the crate
- From this repository, also the nifly submodule: `git clone --recursive`, or
  `git submodule update --init`

## Example

```rust
use nifly::prelude::*;

let mut nif = NifFile::load(std::fs::File::open("model.nif")?, None)?;
for shape in nif.blocks_of::<NiShape>().views() {
    println!("{}: {:?}", shape.name().to_str_lossy(), shape.texture_paths());
}
let mut root = nif.root_node_mut().unwrap();
root.set_transform(Transform::new(Vec3::new(0.0, 0.0, 10.0), Matrix3::IDENTITY, 1.0));
nif.save(std::fs::File::create("moved.nif")?, None)?;
```

The [docs](https://docs.rs/nifly) have the rest. The `mint` feature adds conversions to and from
`mint` types.

## AI use

Much of this crate's code, tests and documentation was written with Claude, Anthropic's AI coding
assistant, directed and reviewed by the maintainer.

## License

GPL-3.0-or-later. nifly, which is built into the crate, is licensed under the GPL v3.
