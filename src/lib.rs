//! NIF files: the model format of Gamebryo-based games, used by Bethesda from Morrowind to
//! Starfield. A file is a list of blocks forming a scene graph: nodes and the shapes under them,
//! with shaders, skinning, collision, animation, particles and extra data.
//!
//! This crate reads and writes NIF files from each of those games, edits their blocks, adds new
//! ones and creates files from scratch, and converts Skyrim meshes between
//! [LE and SE](NifFile::optimize_for).
//!
//! ```
//! use nifly::prelude::*;
//! use std::{fs::File, io::Cursor};
//!
//! let mut nif = NifFile::load(File::open("assets/Cube.nif")?, None)?;
//! for shape in nif.blocks_of::<NiShape>().views() {
//!     println!("{}: {:?}", shape.name().to_str_lossy(), shape.texture_paths());
//! }
//! let mut root = nif.root_node_mut().unwrap();
//! root.set_transform(Transform::new(Vec3::new(0.0, 0.0, 10.0), Matrix3::IDENTITY, 1.0));
//!
//! let mut output = Cursor::new(Vec::new());
//! nif.save(&mut output, None)?;
//! let bytes = output.into_inner();
//! # assert!(!bytes.is_empty());
//! # Ok::<(), nifly::Error>(())
//! ```
//!
//! # Games
//!
//! Each game's version constant lists the types it uses:
//!
//! - [Morrowind](NifVersion::MORROWIND)
//! - [Oblivion](NifVersion::OBLIVION)
//! - [Fallout 3 and New Vegas](NifVersion::FALLOUT_3)
//! - [Skyrim](NifVersion::SKYRIM)
//! - [Skyrim Special Edition](NifVersion::SKYRIM_SE)
//! - [Fallout 4](NifVersion::FALLOUT_4)
//! - [Fallout 76](NifVersion::FALLOUT_76)
//! - [Starfield](NifVersion::STARFIELD)
//!
//! # Modules
//!
//! - Files and blocks: [`file`](mod@file), [`objects`], [`handle`], [`iter`]
//! - Scene graph: [`nodes`], [`geometry`], [`skin`], with transforms in [`object3d`]
//! - Appearance: [`shaders`], [`effects`]
//! - Physics: [`collision`]
//! - Motion: [`animation`], [`particles`]
//! - Metadata: [`extra_data`]
//!
//! # Features
//!
//! - `mint`: conversions for [`Vec3`](object3d::Vec3) and [`Matrix3`](object3d::Matrix3).

#![cfg_attr(docsrs, feature(doc_cfg))]

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
