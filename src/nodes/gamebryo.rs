//! Camera, LOD data, billboard, switch and sort adjust node fields.

use super::*;

/// The volume an [`NiCamera`] sees, in its own coordinates: the edges of its view plane at
/// distance 1 (at any distance for an orthographic camera), and the near and far distances.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Frustum {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

/// The part of the screen an [`NiCamera`] renders to, as fractions of its width and height
/// from the bottom left.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Viewport {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl NiCamera {
    fields! {
        /// Flags with no known use. Saved from Oblivion on.
        raw_flags, set_raw_flags: u16 = camera_flags, set_camera_flags;
        /// Whether the projection is orthographic rather than perspective. Saved from Oblivion
        /// on; Morrowind cameras are always perspective.
        orthographic, set_orthographic: bool = camera_orthographic, set_camera_orthographic;
        /// Scales the distances used to pick levels of detail.
        lod_adjust, set_lod_adjust: f32 = camera_lod_adjust, set_camera_lod_adjust;
        /// A count of screen polygons, which aren't stored; 0 in game files.
        screen_polygon_count, set_screen_polygon_count: u32 = camera_screen_polygons, set_camera_screen_polygons;
        /// A count of screen textures, which aren't stored; 0 in game files. Saved from version
        /// 4.2.1.0 on.
        screen_texture_count, set_screen_texture_count: u32 = camera_screen_textures, set_camera_screen_textures;
    }

    pub fn frustum(&self) -> Frustum {
        let mut v = [0.0; 6];
        ffi::camera_frustum(&self.0, &mut v);
        let [left, right, top, bottom, near, far] = v;
        Frustum {
            left,
            right,
            top,
            bottom,
            near,
            far,
        }
    }

    /// Sets the [frustum](Self::frustum).
    pub fn set_frustum(&mut self, f: Frustum) {
        ffi::set_camera_frustum(
            self.ffi_mut(),
            &[f.left, f.right, f.top, f.bottom, f.near, f.far],
        );
    }

    pub fn viewport(&self) -> Viewport {
        let mut v = [0.0; 4];
        ffi::camera_viewport(&self.0, &mut v);
        let [left, right, top, bottom] = v;
        Viewport {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Sets the [viewport](Self::viewport).
    pub fn set_viewport(&mut self, v: Viewport) {
        ffi::set_camera_viewport(self.ffi_mut(), &[v.left, v.right, v.top, v.bottom]);
    }
}

/// The distances from the camera between which an [`NiLodNode`] child is shown.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LodRange {
    pub near: f32,
    pub far: f32,
}

impl NiRangeLodData {
    vec3_fields! {
        /// The point distances are measured from, in the node's coordinates.
        center, set_center = range_lod_center, set_range_lod_center;
    }

    /// The range of each level, usually one per child, in child order.
    pub fn levels(&self) -> Vec<LodRange> {
        let flat = ffi::range_lod_levels(&self.0);
        flat.as_chunks()
            .0
            .iter()
            .map(|&[near, far]| LodRange { near, far })
            .collect()
    }

    /// Sets the [levels](Self::levels). Not checked against the child count.
    pub fn set_levels(&mut self, levels: &[LodRange]) {
        let flat: Vec<f32> = levels.iter().flat_map(|r| [r.near, r.far]).collect();
        ffi::set_range_lod_levels(self.ffi_mut(), &flat);
    }
}

impl NiScreenLodData {
    /// The bound whose screen size is measured, in the node's coordinates.
    pub fn bound(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::screen_lod_bound(&self.0))
    }

    /// Sets the [bound](Self::bound).
    pub fn set_bound(&mut self, bound: BoundingSphere) {
        ffi::set_screen_lod_bound(self.ffi_mut(), bound.to_ffi());
    }

    /// The [bound](Self::bound) in world coordinates.
    pub fn world_bound(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::screen_lod_world_bound(&self.0))
    }

    /// Sets the [world bound](Self::world_bound).
    pub fn set_world_bound(&mut self, bound: BoundingSphere) {
        ffi::set_screen_lod_world_bound(self.ffi_mut(), bound.to_ffi());
    }

    /// The screen proportion at which each level starts.
    pub fn proportion_levels(&self) -> Vec<f32> {
        ffi::screen_lod_proportion_levels(&self.0)
    }

    /// Sets the [proportion levels](Self::proportion_levels). Not checked against the child
    /// count.
    pub fn set_proportion_levels(&mut self, levels: &[f32]) {
        ffi::set_screen_lod_proportion_levels(self.ffi_mut(), levels);
    }
}

/// An LOD data type that [`Edit::assign_lod_data`] can create.
///
/// This trait is sealed.
pub trait NewLodData: NiType {
    /// Gives `node` a new default `Self` and returns its index (`u32::MAX` on failure).
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32;
}

impl NewLodData for NiRangeLodData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32 {
        nif_ffi::assign_range_lod_data(file, node)
    }
}

impl NewLodData for NiScreenLodData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32 {
        nif_ffi::assign_screen_lod_data(file, node)
    }
}

impl NiBillboardNode {
    /// How the node turns to face the camera. `None` if the stored value isn't a mode.
    ///
    /// Not saved in Morrowind, whose files keep the mode in bits 5–6 of the object flags instead.
    pub fn mode(&self) -> Option<BillboardMode> {
        BillboardMode::try_from(ffi::billboard_mode(&self.0)).ok()
    }

    pub fn set_mode(&mut self, mode: BillboardMode) {
        ffi::set_billboard_mode(self.ffi_mut(), mode.into());
    }
}

/// How an [`NiBillboardNode`] turns to face the camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u16)]
pub enum BillboardMode {
    /// Faces the camera.
    AlwaysFaceCamera = 0,
    /// Turns only around its up axis.
    RotateAboutUp = 1,
    /// Faces the camera without rolling.
    RigidFaceCamera = 2,
    /// Faces the camera's position.
    AlwaysFaceCenter = 3,
    /// Faces the camera's position without rolling.
    RigidFaceCenter = 4,
    /// Turns only around its up axis (Bethesda variant).
    BsRotateAboutUp = 5,
    /// Turns only around its up axis.
    RotateAboutUp2 = 9,
}

impl NiSwitchNode {
    pub fn flags(&self) -> SwitchNodeFlags {
        SwitchNodeFlags::from_bits_retain(ffi::switch_node_flags(&self.0))
    }

    pub fn set_flags(&mut self, flags: SwitchNodeFlags) {
        ffi::set_switch_node_flags(self.ffi_mut(), flags.bits());
    }

    fields! {
        /// The position of the active child among the node's children. Not checked against the
        /// number of children.
        active_index, set_active_index: u32 = switch_node_index, set_switch_node_index;
    }
}

bitflags! {
    /// The flags of an [`NiSwitchNode`]. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SwitchNodeFlags: u16 {
        /// Update only the active child.
        const UPDATE_ONLY_ACTIVE_CHILD = 1;
        const UPDATE_CONTROLLERS = 1 << 1;
        const _ = !0;
    }
}

impl NiSortAdjustNode {
    /// Whether the children are sorted. `None` if the stored value isn't a mode.
    pub fn sorting_mode(&self) -> Option<SortingMode> {
        SortingMode::try_from(ffi::sorting_mode(&self.0)).ok()
    }

    pub fn set_sorting_mode(&mut self, mode: SortingMode) {
        ffi::set_sorting_mode(self.ffi_mut(), mode.into());
    }
}

/// Whether an [`NiSortAdjustNode`]'s children are sorted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum SortingMode {
    /// As the parent sorts.
    Inherit = 0,
    /// Not sorted.
    Off = 1,
}
