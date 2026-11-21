//! Tests for the LOD, segmented, line and screen element shape fields.
//!
//! No test file has these shapes, so each test saves a quad made by `create_shape` and patches
//! the saved bytes into the wanted block types.

use crate::common::{first_index, save};

use nifly::{
    file::{Index, NifFile, NifVersion},
    geometry::{
        BsLodTriShape, BsMeshLodTriShape, BsSegmentedTriShape, GeometrySegment, NiLinesData,
        NiScreenElementsData, NiShape, ScreenPolygon,
    },
    object3d::Vec3,
};

const VERTICES: [Vec3; 4] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
];
const TRIANGLES: [[u16; 3]; 2] = [[0, 1, 2], [0, 2, 3]];
/// The bytes `NiTriShapeData` stores after `NiGeometryData` for the quad: triangle count, point
/// count, has-triangles flag, triangles and match group count.
const TRI_SHAPE_DATA_TAIL: usize = 2 + 4 + 1 + 2 * 6 + 2;

fn u32s(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u16s(values: &[u16]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn reload(nif: &mut NifFile) -> NifFile {
    NifFile::load(save(nif).as_slice(), None).unwrap()
}

/// A block type change: the block gets type `name`, loses `cut` bytes from its end and gains
/// `append`.
struct Retype {
    block: Index,
    name: &'static str,
    cut: usize,
    append: Vec<u8>,
}

/// Applies `changes` to a saved Bethesda 20.2.0.7 file (stream version up to 130).
fn retype(bytes: &[u8], changes: &[Retype]) -> Vec<u8> {
    let mut pos = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;
    let u32_at = |pos: usize| u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
    let u16_at = |pos: usize| u16::from_le_bytes(bytes[pos..pos + 2].try_into().unwrap());
    pos += 4 + 1 + 4; // file version, endianness, user version
    let num_blocks = u32_at(pos) as usize;
    pos += 4 + 4; // block count, stream version
    for _ in 0..3 {
        pos += 1 + bytes[pos] as usize; // creator and export info
    }

    let types_pos = pos;
    let num_types = u16_at(pos);
    pos += 2;
    for _ in 0..num_types {
        pos += 4 + u32_at(pos) as usize;
    }
    let types_end = pos;
    let indices_pos = pos;
    let mut type_indices: Vec<u16> = (0..num_blocks)
        .map(|i| u16_at(indices_pos + 2 * i))
        .collect();
    pos += 2 * num_blocks;
    let sizes_pos = pos;
    let mut sizes: Vec<usize> = (0..num_blocks)
        .map(|i| u32_at(sizes_pos + 4 * i) as usize)
        .collect();
    pos += 4 * num_blocks;
    let strings_start = pos;
    let num_strings = u32_at(pos);
    pos += 8;
    for _ in 0..num_strings {
        pos += 4 + u32_at(pos) as usize;
    }
    pos += 4 + 4 * u32_at(pos) as usize; // groups
    let strings_end = pos;

    let mut blocks = Vec::new();
    for &size in &sizes {
        blocks.push(bytes[pos..pos + size].to_vec());
        pos += size;
    }
    let footer = &bytes[pos..];

    let mut types = bytes[types_pos + 2..types_end].to_vec();
    let mut num_types = num_types;
    for change in changes {
        types.extend(u32s(&[change.name.len() as u32]));
        types.extend(change.name.as_bytes());
        type_indices[change.block] = num_types;
        num_types += 1;
        let block = &mut blocks[change.block];
        block.truncate(block.len() - change.cut);
        block.extend(&change.append);
        sizes[change.block] = block.len();
    }

    let mut out = bytes[..types_pos].to_vec();
    out.extend(num_types.to_le_bytes());
    out.extend(types);
    out.extend(u16s(&type_indices));
    out.extend(u32s(&sizes.iter().map(|&s| s as u32).collect::<Vec<_>>()));
    out.extend(&bytes[strings_start..strings_end]);
    blocks.iter().for_each(|b| out.extend(b));
    out.extend(footer);
    out
}

/// Saves a file with the quad and loads it back with `changes`, which get the shape's and its
/// data's block indices as saved. Returns the file and the shape's index.
fn patched(
    version: NifVersion,
    changes: impl FnOnce(Index, Option<Index>) -> Vec<Retype>,
) -> (NifFile, Index) {
    let mut nif = NifFile::create(version);
    nif.create_shape("Quad", &VERTICES, &TRIANGLES, None, None)
        .unwrap();
    // Saving sorts the blocks, so look them up afterwards.
    let bytes = save(&mut nif);
    let shape = nif.shapes().indices().next().unwrap();
    let data = nif
        .view_of(nif.block_of::<NiShape>(shape).unwrap())
        .unwrap()
        .geometry_data()
        .map(|d| nif.block_index(d).unwrap());
    let bytes = retype(&bytes, &changes(shape, data));
    (NifFile::load(bytes.as_slice(), None).unwrap(), shape)
}

#[test]
fn lod_tri_shape() {
    let (mut nif, shape) = patched(NifVersion::SKYRIM, |shape, _| {
        vec![Retype {
            block: shape,
            name: "BSLODTriShape",
            cut: 0,
            append: u32s(&[1, 1, 0]),
        }]
    });
    assert_eq!(
        nif.block_of::<BsLodTriShape>(shape).unwrap().lod_sizes(),
        [1, 1, 0]
    );

    nif.block_of_mut::<BsLodTriShape>(shape)
        .unwrap()
        .set_lod_sizes([2, 0, 0]);
    let saved = reload(&mut nif);
    assert_eq!(
        saved.block_of::<BsLodTriShape>(shape).unwrap().lod_sizes(),
        [2, 0, 0]
    );
}

#[test]
fn mesh_lod_tri_shape() {
    let (mut nif, shape) = patched(NifVersion::SKYRIM_SE, |shape, _| {
        vec![Retype {
            block: shape,
            name: "BSMeshLODTriShape",
            cut: 0,
            append: u32s(&[2, 0, 0]),
        }]
    });
    assert_eq!(
        nif.block_of::<BsMeshLodTriShape>(shape)
            .unwrap()
            .lod_sizes(),
        [2, 0, 0]
    );

    nif.block_of_mut::<BsMeshLodTriShape>(shape)
        .unwrap()
        .set_lod_sizes([1, 1, 0]);
    let saved = reload(&mut nif);
    assert_eq!(
        saved
            .block_of::<BsMeshLodTriShape>(shape)
            .unwrap()
            .lod_sizes(),
        [1, 1, 0]
    );

    // Deleting vertices resets them.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.delete_vertices(&[1]))
    );
    assert_eq!(
        nif.block_of::<BsMeshLodTriShape>(shape)
            .unwrap()
            .lod_sizes(),
        [0, 0, 1]
    );
}

#[test]
fn segmented_tri_shape() {
    let (mut nif, shape) = patched(NifVersion::SKYRIM, |shape, _| {
        vec![Retype {
            block: shape,
            name: "BSSegmentedTriShape",
            cut: 0,
            append: u32s(&[0]),
        }]
    });
    assert!(
        nif.block_of::<BsSegmentedTriShape>(shape)
            .unwrap()
            .segments()
            .is_empty()
    );

    let segments = [
        GeometrySegment {
            raw_flags: 1,
            index: 0,
            triangle_count: 1,
        },
        GeometrySegment {
            raw_flags: 0,
            index: 3,
            triangle_count: 1,
        },
    ];
    assert!(
        nif.block_of_mut::<BsSegmentedTriShape>(shape)
            .unwrap()
            .set_segments(&segments)
    );
    let saved = reload(&mut nif);
    assert_eq!(
        saved
            .block_of::<BsSegmentedTriShape>(shape)
            .unwrap()
            .segments(),
        segments
    );

    // Past the last triangle.
    let s = nif.block_of_mut::<BsSegmentedTriShape>(shape).unwrap();
    assert!(!s.set_segments(&[GeometrySegment {
        raw_flags: 0,
        index: 3,
        triangle_count: 2
    }]));
    assert!(!s.set_segments(&[GeometrySegment {
        raw_flags: 0,
        index: u32::MAX,
        triangle_count: u32::MAX
    }]));
    assert_eq!(s.segments(), segments);
}

#[test]
fn lines_data() {
    let (mut nif, _) = patched(NifVersion::SKYRIM, |shape, data| {
        vec![
            Retype {
                block: shape,
                name: "NiLines",
                cut: 0,
                append: Vec::new(),
            },
            Retype {
                block: data.unwrap(),
                name: "NiLinesData",
                cut: TRI_SHAPE_DATA_TAIL,
                append: vec![1, 1, 1, 0],
            },
        ]
    });
    let index = first_index::<NiLinesData>(&nif);
    assert_eq!(
        nif.block_of::<NiLinesData>(index).unwrap().line_flags(),
        [true, true, true, false]
    );

    let data = nif.block_of_mut::<NiLinesData>(index).unwrap();
    assert!(data.set_line_flags(&[false, true, false, true]));
    assert!(!data.set_line_flags(&[true]));
    let saved = reload(&mut nif);
    assert_eq!(
        saved.block_of::<NiLinesData>(index).unwrap().line_flags(),
        [false, true, false, true]
    );
}

#[test]
fn screen_elements_data() {
    let polygon = ScreenPolygon {
        vertex_count: 4,
        vertex_offset: 0,
        triangle_count: 2,
        triangle_offset: 0,
    };
    let mut tail = u16s(&[1, 4, 0, 2, 0, 0]); // max polygons, the polygon, its index
    tail.extend(u16s(&[1, 1, 4, 1, 6, 1])); // grow-bys, polygon count, max vertices and indices
    let (mut nif, _) = patched(NifVersion::SKYRIM, |shape, data| {
        vec![
            Retype {
                block: shape,
                name: "NiScreenElements",
                cut: 0,
                append: Vec::new(),
            },
            Retype {
                block: data.unwrap(),
                name: "NiScreenElementsData",
                cut: 0,
                append: tail,
            },
        ]
    });
    let index = first_index::<NiScreenElementsData>(&nif);
    let data = nif.block_of::<NiScreenElementsData>(index).unwrap();
    assert_eq!(data.polygons(), [polygon]);
    assert_eq!(data.polygon_indices(), [0]);
    assert_eq!(data.polygon_count(), 1);
    assert_eq!(
        [
            data.polygon_grow_by(),
            data.max_vertices(),
            data.vertices_grow_by(),
            data.max_indices(),
            data.indices_grow_by()
        ],
        [1, 4, 1, 6, 1]
    );

    let half = ScreenPolygon {
        vertex_count: 3,
        vertex_offset: 0,
        triangle_count: 1,
        triangle_offset: 0,
    };
    let other = ScreenPolygon {
        vertex_offset: 1,
        triangle_offset: 1,
        ..half
    };
    let data = nif.block_of_mut::<NiScreenElementsData>(index).unwrap();
    assert!(data.set_polygons(&[half, other], &[0, 1]));
    assert!(data.set_polygon_count(2));
    data.set_max_indices(9);
    // Mismatched lengths, fewer slots than the polygon count, a count above the slots.
    assert!(!data.set_polygons(&[half], &[0, 1]));
    assert!(!data.set_polygons(&[half], &[0]));
    assert!(!data.set_polygon_count(3));

    let saved = reload(&mut nif);
    let data = saved.block_of::<NiScreenElementsData>(index).unwrap();
    assert_eq!(data.polygons(), [half, other]);
    assert_eq!(data.polygon_indices(), [0, 1]);
    assert_eq!((data.polygon_count(), data.max_indices()), (2, 9));
}
