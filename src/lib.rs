//! Reading, editing and writing NIF files, the mesh format of Bethesda's games from Morrowind to
//! Starfield.
//!
//! A [`NifFile`] holds a file's blocks. Borrow them by type for their fields, and through a
//! [`View`](handle::View) or [`Edit`](handle::Edit) for what involves other blocks: children,
//! shaders, skins. Each block type's docs list its handle methods.
//!
//! ```
//! use nifly::prelude::*;
//! # use std::{fs::File, io::Cursor};
//!
//! let mut nif = NifFile::load(File::open("assets/Cube.nif")?, None)?;
//! for node in nif.blocks_of::<NiNode>() {
//!     println!("{}", node.name().to_str_lossy());
//! }
//! let root = nif.root_node().unwrap().index();
//! let node = nif.block_of_mut::<NiNode>(root).unwrap();
//! node.set_transform(Transform::new(Vec3::new(0.0, 0.0, 10.0), Matrix3::IDENTITY, 1.0));
//! nif.save(Cursor::new(Vec::new()), None)?;
//! # Ok::<(), nifly::Error>(())
//! ```
//!
//! [`prelude`] has the common types. Names and paths are byte strings ([`bstr`]), since files
//! aren't always UTF-8.
//!
//! Each game's version constant lists the types the game uses:
//! [`MORROWIND`](NifVersion::MORROWIND), [`OBLIVION`](NifVersion::OBLIVION),
//! [`FALLOUT_3`](NifVersion::FALLOUT_3), [`SKYRIM`](NifVersion::SKYRIM),
//! [`SKYRIM_SE`](NifVersion::SKYRIM_SE), [`FALLOUT_4`](NifVersion::FALLOUT_4),
//! [`FALLOUT_76`](NifVersion::FALLOUT_76) and [`STARFIELD`](NifVersion::STARFIELD).

pub use bstr;

pub use error::{Error, NifResult};
pub use file::{NifFile, NifVersion};

#[macro_use]
mod macros;

pub mod error;
pub mod extra_data;
pub mod handle;
pub mod iter;
pub mod object3d;
pub mod objects;
pub mod prelude;

// The order of these sets the order of `NifFile`'s doc sections, one per module.
pub mod file;
pub mod nodes;
pub mod geometry;
pub mod skin;
pub mod shaders;
pub mod effects;
pub mod collision;
pub mod animation;
pub mod particles;
