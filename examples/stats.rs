//! Prints everything the bindings can currently read from a NIF file.
//!
//! cargo run --example stats -- <path>

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    error::Error,
    fs, process, ptr,
};

use nifly::{
    extra_data::{
        BsClothExtraData, BsCollisionQueryProxyExtraData, NiBooleanExtraData, NiExtraData,
        NiFloatExtraData, NiIntegerExtraData, NiStringExtraData,
    },
    file::NifFile,
    geometry::NiShape,
    nodes::NiNode,
    object3d::Transform,
    objects::{NiAvObject, NiObject},
};

fn main() -> Result<(), Box<dyn Error>> {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: stats <path>");
        process::exit(2);
    };
    let nif = NifFile::load(fs::File::open(&path)?, None)?;

    // Block index by address, so references can be printed as indices.
    let index: HashMap<*const NiObject, usize> = nif
        .blocks()
        .enumerate()
        .map(|(i, b)| (ptr::from_ref(b), i))
        .collect();
    let idx = |b: &NiObject| index[&ptr::from_ref(b)];

    println!("{path}");
    println!("  valid:            {}", nif.is_valid());
    println!("  unknown blocks:   {}", nif.has_unknown());
    println!("  terrain:          {}", nif.is_terrain());
    println!("  SSE compatible:   {}", nif.is_all_sse_compatible());
    println!("  vertex limit:     {}", NifFile::vertex_limit());
    println!("  triangle limit:   {}", nif.triangle_limit());
    println!("  blocks:           {}", index.len());
    println!("  nodes:            {}", nif.nodes().count());

    println!("\nBlock types:");
    let mut counts = BTreeMap::new();
    for block in nif.blocks() {
        *counts.entry(block.block_name()).or_insert(0) += 1;
    }
    for (name, count) in &counts {
        println!("  {count:4}  {name}");
    }

    println!("\nBlocks (index, type, referenced blocks):");
    for (i, block) in nif.blocks().enumerate() {
        let refs: Vec<_> = nif.child_blocks(block).map(idx).collect();
        if refs.is_empty() {
            println!("  {i:4}  {}", block.block_name());
        } else {
            println!("  {i:4}  {} -> {refs:?}", block.block_name());
        }
    }

    println!("\nScene graph:");
    match nif.root_node() {
        Some(root) => print_tree(&nif, &idx, root.get(), 1),
        None => println!("  (no root node)"),
    }

    let reachable: HashSet<_> = nif.tree().map(idx).collect();
    println!(
        "\nTree order from root: {:?}",
        nif.tree().map(idx).collect::<Vec<_>>()
    );
    let unreachable: Vec<_> = (0..index.len())
        .filter(|i| !reachable.contains(i))
        .collect();
    if unreachable.is_empty() {
        println!("All blocks reachable from root.");
    } else {
        println!("Unreachable from root: {unreachable:?}");
    }

    Ok(())
}

fn print_tree(nif: &NifFile, idx: &dyn Fn(&NiObject) -> usize, obj: &NiAvObject, depth: usize) {
    let pad = depth * 2;
    println!(
        "{:pad$}[{}] {} \"{}\"",
        "",
        idx(obj),
        obj.block_name(),
        obj.name()
    );
    println!("{:pad$}    flags:  {:#x}", "", obj.raw_flags());
    println!("{:pad$}    local:  {}", "", fmt_transform(&obj.transform()));

    if let Some(node) = obj.downcast::<NiNode>() {
        // Looked up by name like nifly does, so duplicate names resolve to the first node.
        if let Some(global) = nif.node_transform_to_global(node.name()) {
            println!("{:pad$}    global: {}", "", fmt_transform(&global));
        }
        println!("{:pad$}    child refs: {}", "", node.child_count());
    }

    if let Some(shape) = obj.downcast::<NiShape>() {
        match nif.view_of(shape).unwrap().shader() {
            Some(shader) => println!(
                "{:pad$}    shader: [{}] {}",
                "",
                shader.index(),
                shader.block_name()
            ),
            None => println!("{:pad$}    shader: (none)", ""),
        }
        for (slot, path) in nif
            .view_of(shape)
            .unwrap()
            .texture_paths()
            .iter()
            .enumerate()
        {
            if !path.is_empty() {
                println!("{:pad$}    texture {slot}: {path}", "");
            }
        }
        let bones = nif.view_of(shape).unwrap().bone_list();
        let bone_ids = nif.view_of(shape).unwrap().bone_ids();
        if bones.is_empty() && bone_ids.is_empty() {
            println!("{:pad$}    bones:  (not skinned)", "");
        } else {
            println!("{:pad$}    bones:  {bones:?}", "");
            println!("{:pad$}    bone blocks: {bone_ids:?}", "");
            let counts: Vec<_> = (0..bones.len())
                .map(|b| nif.view_of(shape).unwrap().bone_weights(b).len())
                .collect();
            println!("{:pad$}    weighted vertices per bone: {counts:?}", "");
        }
    }

    for extra in nif.view_of(obj).unwrap().extra_data() {
        println!(
            "{:pad$}    extra:  [{}] {}{}",
            "",
            idx(extra),
            extra.block_name(),
            fmt_extra(extra)
        );
    }

    if let Some(node) = obj.downcast::<NiNode>() {
        for child in nif.view_of(node).unwrap().children() {
            print_tree(nif, idx, child, depth + 1);
        }
    }
}

fn fmt_transform(t: &Transform) -> String {
    let v = |v: &nifly::object3d::Vec3| format!("({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
    format!(
        "t={} r=[{}, {}, {}] s={:.3}",
        v(&t.translation),
        v(&t.rotation.rows[0]),
        v(&t.rotation.rows[1]),
        v(&t.rotation.rows[2]),
        t.scale,
    )
}

/// ` "name" = value` for the extra data types the bindings can read.
fn fmt_extra(obj: &NiObject) -> String {
    if let Some(cloth) = obj.downcast::<BsClothExtraData>() {
        return format!(" ({} bytes)", cloth.data().len());
    }
    if let Some(proxy) = obj.downcast::<BsCollisionQueryProxyExtraData>() {
        return format!(" ({} bytes)", proxy.data().len());
    }
    let Some(extra) = obj.downcast::<NiExtraData>() else {
        return String::new();
    };

    let value = if let Some(s) = obj.downcast::<NiStringExtraData>() {
        format!(" = {:?}", s.value())
    } else if let Some(i) = obj.downcast::<NiIntegerExtraData>() {
        format!(" = {:#x}", i.value())
    } else if let Some(f) = obj.downcast::<NiFloatExtraData>() {
        format!(" = {}", f.value())
    } else if let Some(b) = obj.downcast::<NiBooleanExtraData>() {
        format!(" = {}", b.value())
    } else {
        String::new()
    };
    format!(" {:?}{value}", extra.name())
}
