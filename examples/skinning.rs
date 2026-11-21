//! Prints the bones and weights of each skinned shape.
//!
//! cargo run --example skinning [-- <path>]

use std::{env, error::Error};

use nifly::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap_or("assets/Bones.nif".into());
    let nif = NifFile::open(&path, None)?;

    for shape in nif.shapes().views() {
        let name = shape.name().to_str_lossy();
        let bones = shape.bone_list();
        let Some(skin) = shape.skin_instance().filter(|_| !bones.is_empty()) else {
            println!("{name}: not skinned");
            continue;
        };
        println!(
            "{name}: {}, {} weights per vertex",
            skin.block_name(),
            shape.weights_per_vertex()
        );

        // Bone nodes' block indices, in the same order as the names.
        let ids = shape.bone_ids();
        for (b, bone) in bones.iter().enumerate() {
            let weights = shape.bone_weights(b);
            let max = weights.iter().map(|w| w.weight).fold(0.0, f32::max);
            print!(
                "  {bone} (block {}): {} vertices, max weight {max:.2}",
                ids[b],
                weights.len()
            );
            if let Some(bounds) = shape.bone_bounds(b) {
                print!(", bounds radius {:.2}", bounds.radius);
            }
            println!();
        }
    }
    Ok(())
}
