//! Fog, vertex color and z-buffer properties, and adding properties.

use super::*;

/// A property type that [`Edit::add_property`] can create.
///
/// This trait is sealed.
pub trait NewProperty: NiType {
    /// Adds a default property to `target`'s property list and returns its index (`u32::MAX` on
    /// failure).
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32;
}

macro_rules! new_property {
    ($($ty:ty => $add:ident;)*) => {$(
        impl NewProperty for $ty {
            fn add(file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
                nif_ffi::$add(file, target)
            }
        }
    )*};
}

new_property! {
    NiMaterialProperty => add_material_property;
    NiStencilProperty => add_stencil_property;
    NiTexturingProperty => add_texturing_property;
    NiSpecularProperty => add_specular_property;
    NiVertexColorProperty => add_vertex_color_property;
    NiDitherProperty => add_dither_property;
    NiFogProperty => add_fog_property;
    NiWireframeProperty => add_wireframe_property;
    NiZBufferProperty => add_z_buffer_property;
    BsShaderPpLightingProperty => add_bs_shader_pp_lighting_property;
    Lighting30ShaderProperty => add_lighting30_shader_property;
    BsShaderNoLightingProperty => add_bs_shader_no_lighting_property;
    SkyShaderProperty => add_sky_shader_property;
    TileShaderProperty => add_tile_shader_property;
    TallGrassShaderProperty => add_tall_grass_shader_property;
    WaterShaderProperty => add_water_shader_property;
    HairShaderProperty => add_hair_shader_property;
    DistantLodShaderProperty => add_distant_lod_shader_property;
    BsDistantTreeShaderProperty => add_bs_distant_tree_shader_property;
    VolumetricFogShaderProperty => add_volumetric_fog_shader_property;
    BsWaterShaderProperty => add_bs_water_shader_property;
    BsSkyShaderProperty => add_bs_sky_shader_property;
}

/// `flags` getter and setter, plus `enabled` over bit 0, for properties whose only setting is a
/// switch.
macro_rules! switch_property {
    ($($ty:ty: $what:literal, $get:ident, $set:ident;)*) => {$(
        impl $ty {
            #[doc = concat!("Whether ", $what, " is on: bit 0 of [`raw_flags`](Self::raw_flags).")]
            pub fn enabled(&self) -> bool {
                self.raw_flags() & 1 != 0
            }

            /// The raw flags. Only bit 0 is used.
            pub fn raw_flags(&self) -> u16 {
                ffi::$get(&self.0)
            }

            pub fn set_enabled(&mut self, enabled: bool) {
                let flags = (self.raw_flags() & !1) | u16::from(enabled);
                self.set_raw_flags(flags);
            }

            pub fn set_raw_flags(&mut self, flags: u16) {
                ffi::$set(self.ffi_mut(), flags);
            }
        }
    )*};
}

switch_property! {
    NiSpecularProperty: "specular lighting", specular_flags, set_specular_flags;
    NiDitherProperty: "dithering", dither_flags, set_dither_flags;
    NiWireframeProperty: "wireframe drawing", wireframe_flags, set_wireframe_flags;
}

impl NiFogProperty {
    const FUNCTION: BitField = (1, 2);

    fields! {
        /// The raw flags: bit 0 turns fog on, bits 1–2 hold the [function](Self::function).
        raw_flags, set_raw_flags: u16 = fog_flags, set_fog_flags, same, same;
        /// How far the fog reaches.
        depth, set_depth: f32 = fog_depth, set_fog_depth, same, same;
        /// The fog color, as `[r, g, b]`.
        color, set_color: [f32; 3] = fog_color, set_fog_color, rgb, rgb_to_ffi;
    }

    /// Whether fog is on.
    pub fn enabled(&self) -> bool {
        self.raw_flags() & 1 != 0
    }

    /// How fog thickens with distance. `None` if the bits hold 3.
    pub fn function(&self) -> Option<FogFunction> {
        FogFunction::try_from(get_bits(self.raw_flags(), Self::FUNCTION)).ok()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        let flags = (self.raw_flags() & !1) | u16::from(enabled);
        self.set_raw_flags(flags);
    }

    pub fn set_function(&mut self, function: FogFunction) {
        let flags = set_bits(self.raw_flags(), Self::FUNCTION, function.into());
        self.set_raw_flags(flags);
    }
}

/// How fog thickens with distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum FogFunction {
    /// Linear in the depth.
    ZLinear = 0,
    /// By the squared distance.
    RangeSquared = 1,
    /// From the vertex alpha.
    VertexAlpha = 2,
}

impl NiVertexColorProperty {
    /// The packed settings, used from Fallout 3 on.
    pub fn flags(&self) -> VertexColorFlags {
        VertexColorFlags::from_bits(ffi::vertex_color_flags(&self.0))
    }

    /// Which material colors the vertex colors replace (Oblivion and older). `None` if the stored
    /// value isn't a mode.
    pub fn vertex_mode(&self) -> Option<VertexColorMode> {
        VertexColorMode::try_from(ffi::vertex_color_vertex_mode(&self.0)).ok()
    }

    /// Which lighting terms are used (Oblivion and older). `None` if the stored value isn't a
    /// mode.
    pub fn lighting_mode(&self) -> Option<LightingMode> {
        LightingMode::try_from(ffi::vertex_color_lighting_mode(&self.0)).ok()
    }

    pub fn set_flags(&mut self, flags: VertexColorFlags) {
        ffi::set_vertex_color_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_vertex_mode(&mut self, mode: VertexColorMode) {
        ffi::set_vertex_color_vertex_mode(self.ffi_mut(), mode.into());
    }

    pub fn set_lighting_mode(&mut self, mode: LightingMode) {
        ffi::set_vertex_color_lighting_mode(self.ffi_mut(), mode.into());
    }
}

/// Which material colors vertex colors replace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum VertexColorMode {
    /// Vertex colors are ignored.
    Ignore = 0,
    /// They replace the emissive color.
    Emissive = 1,
    /// They replace the ambient and diffuse colors.
    AmbientDiffuse = 2,
}

/// Which lighting terms an [`NiVertexColorProperty`] uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LightingMode {
    /// Emissive only.
    Emissive = 0,
    /// Emissive, ambient and diffuse.
    EmissiveAmbientDiffuse = 1,
}

/// The packed settings of an [`NiVertexColorProperty`], used from Fallout 3 on.
///
/// Bits:
/// - 3: lighting mode
/// - 4–5: vertex mode
///
/// The `with_*` methods replace one setting and keep the others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VertexColorFlags(u16);

impl VertexColorFlags {
    const LIGHTING: BitField = (3, 1);
    const VERTEX: BitField = (4, 2);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    pub fn lighting_mode(self) -> LightingMode {
        LightingMode::try_from(get_bits(self.0, Self::LIGHTING)).expect("1 bit is always a lighting mode")
    }

    /// `None` if the bits hold 3.
    pub fn vertex_mode(self) -> Option<VertexColorMode> {
        VertexColorMode::try_from(get_bits(self.0, Self::VERTEX)).ok()
    }

    #[must_use]
    pub fn with_lighting_mode(self, mode: LightingMode) -> Self {
        Self(set_bits(self.0, Self::LIGHTING, mode.into()))
    }

    #[must_use]
    pub fn with_vertex_mode(self, mode: VertexColorMode) -> Self {
        Self(set_bits(self.0, Self::VERTEX, mode.into()))
    }
}

impl NiZBufferProperty {
    /// The packed settings.
    pub fn flags(&self) -> ZBufferFlags {
        ZBufferFlags::from_bits(ffi::z_buffer_flags(&self.0))
    }

    /// The depth test function (Oblivion and older, but not Morrowind). `None` if the stored
    /// value isn't a test function.
    pub fn function(&self) -> Option<DepthTest> {
        DepthTest::try_from(ffi::z_buffer_function(&self.0)).ok()
    }

    pub fn set_flags(&mut self, flags: ZBufferFlags) {
        ffi::set_z_buffer_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_function(&mut self, function: DepthTest) {
        ffi::set_z_buffer_function(self.ffi_mut(), function.into());
    }
}

/// A depth test function: how a pixel's depth compares with the stored depth for the pixel to
/// be drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum DepthTest {
    /// Always drawn.
    Always = 0,
    /// Drawn if its depth is less than the stored depth.
    Less = 1,
    /// Drawn if they're equal.
    Equal = 2,
    /// Drawn if its depth is less than or equal to the stored depth.
    LessOrEqual = 3,
    /// Drawn if its depth is greater than the stored depth.
    Greater = 4,
    /// Drawn if they differ.
    NotEqual = 5,
    /// Drawn if its depth is greater than or equal to the stored depth.
    GreaterOrEqual = 6,
    /// Never drawn.
    Never = 7,
}

/// The packed settings of an [`NiZBufferProperty`].
///
/// Bits:
/// - 0: depth testing
/// - 1: depth writing
/// - 2–5: test function, used from Fallout 3 on
///
/// The `with_*` methods replace one setting and keep the others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ZBufferFlags(u16);

impl ZBufferFlags {
    const TESTING: BitField = (0, 1);
    const WRITING: BitField = (1, 1);
    const TEST: BitField = (2, 4);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Whether depth testing is on.
    pub fn testing(self) -> bool {
        get_bits(self.0, Self::TESTING) != 0
    }

    /// Whether drawn pixels write their depth.
    pub fn writing(self) -> bool {
        get_bits(self.0, Self::WRITING) != 0
    }

    /// `None` if the bits hold 8 or more.
    pub fn function(self) -> Option<DepthTest> {
        DepthTest::try_from(get_bits(self.0, Self::TEST)).ok()
    }

    #[must_use]
    pub fn with_testing(self, on: bool) -> Self {
        Self(set_bits(self.0, Self::TESTING, on.into()))
    }

    #[must_use]
    pub fn with_writing(self, on: bool) -> Self {
        Self(set_bits(self.0, Self::WRITING, on.into()))
    }

    #[must_use]
    pub fn with_function(self, function: DepthTest) -> Self {
        Self(set_bits(self.0, Self::TEST, function.into()))
    }
}
