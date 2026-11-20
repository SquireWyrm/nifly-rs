//! Properties and shaders: `NiProperty`, `NiShader` and its subclasses.
//!
//! ```text
//! BsLightingShaderProperty -> BsShaderProperty -> NiShadeProperty -> NiShader -> NiProperty -> NiObjectNet
//! ```

use std::pin::Pin;

use bitflags::{bitflags, Flags};
use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    nif_file::ffi as nif_ffi,
    nif_file_refs::ffi::{RefKind, RefList},
    shaders::ffi,
};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, Index, NifFile},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    geometry::{NiShape, TextureSource},
    object3d::Vec3,
    objects::{IsA, NiAvObject, NiObject, NiObjectNet, NiType, Sealed},
};

/// Getter and setter pairs over the bridge's field shims, converting with `$to` and `$from`.
macro_rules! fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident, $to:expr, $from:expr;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ($to)(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), ($from)(value));
        }
    )*};
}

fn same<T>(x: T) -> T {
    x
}

fn uv(v: ffi::Vec2) -> [f32; 2] {
    [v.x, v.y]
}

fn uv_to_ffi([x, y]: [f32; 2]) -> ffi::Vec2 {
    ffi::Vec2 { x, y }
}

fn rgba(c: ffi::Rgba) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

fn rgba_to_ffi([r, g, b, a]: [f32; 4]) -> ffi::Rgba {
    ffi::Rgba { r, g, b, a }
}

ni_types! { ffi;
    /// Base class of properties attached to shapes and nodes (shaders, alpha, ...).
    NiProperty(NiProperty): NiObjectNet => upcast_ni_property, downcast_ni_property;
    /// Base class of shaders. Subclasses are also returned as `NiShader`; use
    /// [`block_name`](crate::objects::NiObject::block_name) to see the concrete type.
    NiShader(NiShader): NiProperty => upcast_ni_shader, downcast_ni_shader;
    NiShadeProperty(NiShadeProperty): NiShader => upcast_ni_shade_property, downcast_ni_shade_property;
    /// Base class of Bethesda shaders.
    BsShaderProperty(BSShaderProperty): NiShadeProperty => upcast_bs_shader_property, downcast_bs_shader_property;
    WaterShaderProperty(WaterShaderProperty): BsShaderProperty => upcast_water_shader_property, downcast_water_shader_property;
    HairShaderProperty(HairShaderProperty): BsShaderProperty => upcast_hair_shader_property, downcast_hair_shader_property;
    /// `DistantLODShaderProperty`.
    DistantLodShaderProperty(DistantLODShaderProperty): BsShaderProperty => upcast_distant_lod_shader_property, downcast_distant_lod_shader_property;
    /// `BSDistantTreeShaderProperty`.
    BsDistantTreeShaderProperty(BSDistantTreeShaderProperty): BsShaderProperty => upcast_bs_distant_tree_shader_property, downcast_bs_distant_tree_shader_property;
    TallGrassShaderProperty(TallGrassShaderProperty): BsShaderProperty => upcast_tall_grass_shader_property, downcast_tall_grass_shader_property;
    VolumetricFogShaderProperty(VolumetricFogShaderProperty): BsShaderProperty => upcast_volumetric_fog_shader_property, downcast_volumetric_fog_shader_property;
    /// The usual shader from Skyrim on.
    BsLightingShaderProperty(BSLightingShaderProperty): BsShaderProperty => upcast_bs_lighting_shader_property, downcast_bs_lighting_shader_property;
    /// The shader for effects such as particles, glows and falloff, from Skyrim on.
    BsEffectShaderProperty(BSEffectShaderProperty): BsShaderProperty => upcast_bs_effect_shader_property, downcast_bs_effect_shader_property;
    /// `BSWaterShaderProperty`.
    BsWaterShaderProperty(BSWaterShaderProperty): BsShaderProperty => upcast_bs_water_shader_property, downcast_bs_water_shader_property;
    /// `BSSkyShaderProperty`.
    BsSkyShaderProperty(BSSkyShaderProperty): BsShaderProperty => upcast_bs_sky_shader_property, downcast_bs_sky_shader_property;
    /// `BSShaderLightingProperty`.
    BsShaderLightingProperty(BSShaderLightingProperty): BsShaderProperty => upcast_bs_shader_lighting_property, downcast_bs_shader_lighting_property;
    SkyShaderProperty(SkyShaderProperty): BsShaderLightingProperty => upcast_sky_shader_property, downcast_sky_shader_property;
    TileShaderProperty(TileShaderProperty): BsShaderLightingProperty => upcast_tile_shader_property, downcast_tile_shader_property;
    /// `BSShaderNoLightingProperty`.
    BsShaderNoLightingProperty(BSShaderNoLightingProperty): BsShaderLightingProperty => upcast_bs_shader_no_lighting_property, downcast_bs_shader_no_lighting_property;
    /// The usual shader in Fallout 3 and New Vegas.
    BsShaderPpLightingProperty(BSShaderPPLightingProperty): BsShaderLightingProperty => upcast_bs_shader_pp_lighting_property, downcast_bs_shader_pp_lighting_property;
    Lighting30ShaderProperty(Lighting30ShaderProperty): BsShaderPpLightingProperty => upcast_lighting30_shader_property, downcast_lighting30_shader_property;
    /// Material colors and alpha (files before Skyrim), from
    /// [`NifFile::material_property`](crate::handle::View::material_property).
    /// [`NifFile::shader`](crate::handle::View::shader) only returns it when a shape has no other
    /// shader.
    ///
    /// Colors are `[r, g, b]`, usually in `0.0..=1.0`. To change one, get it with
    /// [`NifFile::material_property`](crate::handle::Edit::material_property).
    NiMaterialProperty(NiMaterialProperty): NiShader => upcast_ni_material_property, downcast_ni_material_property;
    /// Alpha blending and testing of a shape, from
    /// [`NifFile::alpha_property`](crate::handle::View::alpha_property).
    NiAlphaProperty(NiAlphaProperty): NiProperty => upcast_ni_alpha_property, downcast_ni_alpha_property;
    /// Stencil testing and which sides of a shape are drawn (files before Skyrim), from
    /// [`NifFile::stencil_property`](crate::handle::View::stencil_property).
    ///
    /// Oblivion and older files save the settings as separate fields
    /// ([`enabled`](Self::enabled), [`function`](Self::function), the actions and
    /// [`draw_mode`](Self::draw_mode)). Fallout 3 and later files pack them into
    /// [`flags`](Self::flags) instead. Each layout is only read and saved in its own versions, and
    /// they aren't converted into each other. [`reference`](Self::reference) and
    /// [`mask`](Self::mask) are saved in both.
    NiStencilProperty(NiStencilProperty): NiProperty => upcast_ni_stencil_property, downcast_ni_stencil_property;
    /// The texture maps of a shape (files before Skyrim), from
    /// [`NifFile::texturing_property`](crate::handle::View::texturing_property). The maps'
    /// paths are read and set with [`NifFile::texture_slot`](crate::handle::View::texture_slot)
    /// and [`NifFile::set_texture_slot`](crate::handle::Edit::set_texture_slot).
    ///
    /// Which maps are saved depends on [`texture_count`](Self::texture_count) and the file
    /// version.
    NiTexturingProperty(NiTexturingProperty): NiProperty => upcast_ni_texturing_property, downcast_ni_texturing_property;
    /// Specular highlights on or off (files before Skyrim).
    NiSpecularProperty(NiSpecularProperty): NiProperty => upcast_ni_specular_property, downcast_ni_specular_property;
    /// How vertex colors are used in lighting (files before Skyrim).
    ///
    /// Oblivion and older files save the settings as separate fields
    /// ([`vertex_mode`](Self::vertex_mode), [`lighting_mode`](Self::lighting_mode)). Fallout 3
    /// and later files don't save those, and pack the settings into [`flags`](Self::flags)
    /// instead. The two aren't converted into each other.
    NiVertexColorProperty(NiVertexColorProperty): NiProperty => upcast_ni_vertex_color_property, downcast_ni_vertex_color_property;
    /// Dithering on or off (files before Skyrim).
    NiDitherProperty(NiDitherProperty): NiProperty => upcast_ni_dither_property, downcast_ni_dither_property;
    /// Fog (files before Skyrim).
    NiFogProperty(NiFogProperty): NiProperty => upcast_ni_fog_property, downcast_ni_fog_property;
    /// Wireframe drawing on or off (files before Skyrim).
    NiWireframeProperty(NiWireframeProperty): NiProperty => upcast_ni_wireframe_property, downcast_ni_wireframe_property;
    /// Depth testing and writing (files before Skyrim).
    ///
    /// Before Fallout 3, the test function is a separate field ([`function`](Self::function),
    /// not saved in Morrowind). Fallout 3 and later files don't save it, and pack the test
    /// function into [`flags`](Self::flags) instead. The two aren't converted into each other.
    NiZBufferProperty(NiZBufferProperty): NiProperty => upcast_ni_z_buffer_property, downcast_ni_z_buffer_property;
    /// Base class of textures.
    NiTexture(NiTexture): NiObjectNet => upcast_ni_texture, downcast_ni_texture;
    /// A texture, usually in an external file, used by an [`NiTexturingProperty`] (Oblivion and
    /// older; see [`View::map_source`]).
    NiSourceTexture(NiSourceTexture): NiTexture => upcast_ni_source_texture, downcast_ni_source_texture;
    /// An [`NiSourceTexture`] holding a cube map.
    NiSourceCubeMap(NiSourceCubeMap): NiSourceTexture => upcast_ni_source_cube_map, downcast_ni_source_cube_map;
    /// Pixel data of an [`NiSourceTexture`]: the pixel format and mipmaps of an
    /// [`NiPixelData`] or [`NiPersistentSrcTextureRendererData`].
    TextureRenderData(TextureRenderData): NiObject => upcast_texture_render_data, downcast_texture_render_data;
    /// A texture's pixels, stored in the file.
    NiPixelData(NiPixelData): TextureRenderData => upcast_ni_pixel_data, downcast_ni_pixel_data;
    /// A texture's pixels, kept after loading (Fallout 3 and later).
    NiPersistentSrcTextureRendererData(NiPersistentSrcTextureRendererData): TextureRenderData =>
        upcast_ni_persistent_src_texture_renderer_data, downcast_ni_persistent_src_texture_renderer_data;
    /// The 16 or 256 colors of a palettized texture, from [`View::palette`].
    NiPalette(NiPalette): NiObject => upcast_ni_palette, downcast_ni_palette;
    /// The texture paths of a [`BsLightingShaderProperty`] or [`BsShaderPpLightingProperty`],
    /// from [`View::texture_set`].
    BsShaderTextureSet(BSShaderTextureSet): NiObject => upcast_bs_shader_texture_set, downcast_bs_shader_texture_set;
}

impl BsShaderTextureSet {
    /// Number of slots, including empty ones, as stored in the file. New sets have 9 slots for
    /// Skyrim (LE and SE), 10 for Fallout 4, 13 for Fallout 76 and 6 for older games.
    pub fn len(&self) -> usize {
        ffi::texture_set_len(&self.0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The path in slot `index` (empty if the slot is unused), or `None` if out of
    /// range. Slot meanings depend on the shader: for Skyrim, 0 is diffuse, 1 normal, 2 glow or
    /// skin tint, 3 height (parallax), 4 environment, 5 environment mask, 6 subsurface or face
    /// tint, 7 back light or specular.
    pub fn path(&self, index: usize) -> Option<&BStr> {
        (index < self.len()).then(|| ffi::texture_set_path(&self.0, index).as_bstr())
    }

    /// Every slot's path, in order, including empty slots.
    pub fn paths(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..self.len()).map(|i| ffi::texture_set_path(&self.0, i).as_bstr())
    }

    /// Sets the path in slot `index`. Returns `false`, changing nothing, if it's out of range.
    pub fn set_path(&mut self, index: usize, path: impl AsRef<[u8]>) -> bool {
        ffi::set_texture_set_path(self.ffi_mut(), index, path.as_ref())
    }

    /// Replaces every slot, changing the number of slots to the number of paths.
    pub fn set_paths<P: AsRef<[u8]>>(&mut self, paths: impl IntoIterator<Item = P>) {
        let paths: Vec<P> = paths.into_iter().collect();
        ffi::set_texture_set_len(self.ffi_mut(), paths.len());
        for (i, path) in paths.iter().enumerate() {
            ffi::set_texture_set_path(self.ffi_mut(), i, path.as_ref());
        }
    }
}

impl NiShadeProperty {
    fields! {
        /// The shading flags, as raw bits: bit 0 is smooth shading. Only saved before Skyrim.
        shading_raw_flags, set_shading_raw_flags: u16 = shade_flags, set_shade_flags, same, same;
    }
}

impl BsShaderProperty {
    /// The first shader flags field, as raw bits. Their meaning depends on the game; read them
    /// with [`flags_1`](Self::flags_1).
    ///
    /// Not saved from stream version 132 on (Fallout 76, Starfield and the latest Fallout 4
    /// files), which save [`crc_flags_1`](Self::crc_flags_1) instead. The two aren't converted
    /// into each other.
    pub fn raw_flags_1(&self) -> u32 {
        ffi::shader_flags_1(&self.0)
    }

    /// The second shader flags field, as raw bits. Their meaning depends on the game; read
    /// them with [`flags_2`](Self::flags_2).
    ///
    /// Not saved from stream version 132 on. Those files save [`crc_flags_2`](Self::crc_flags_2)
    /// instead.
    pub fn raw_flags_2(&self) -> u32 {
        ffi::shader_flags_2(&self.0)
    }

    pub fn set_raw_flags_1(&mut self, flags: u32) {
        ffi::set_shader_flags_1(self.ffi_mut(), flags);
    }

    pub fn set_raw_flags_2(&mut self, flags: u32) {
        ffi::set_shader_flags_2(self.ffi_mut(), flags);
    }

    /// The first shader flags field as the flags of the file's game: [`Fallout3ShaderFlags1`],
    /// [`SkyrimShaderFlags1`] or [`Fallout4ShaderFlags1`]. Not checked against the file.
    pub fn flags_1<F: ShaderFlags1>(&self) -> F {
        F::from_bits_retain(self.raw_flags_1())
    }

    /// The second shader flags field as the flags of the file's game: [`Fallout3ShaderFlags2`],
    /// [`SkyrimShaderFlags2`] or [`Fallout4ShaderFlags2`]. Not checked against the file.
    pub fn flags_2<F: ShaderFlags2>(&self) -> F {
        F::from_bits_retain(self.raw_flags_2())
    }

    /// Sets the [first shader flags](Self::flags_1) from a game's flags.
    pub fn set_flags_1<F: ShaderFlags1>(&mut self, flags: F) {
        self.set_raw_flags_1(flags.bits());
    }

    /// Sets the [second shader flags](Self::flags_2) from a game's flags.
    pub fn set_flags_2<F: ShaderFlags2>(&mut self, flags: F) {
        self.set_raw_flags_2(flags.bits());
    }

    /// The first list of shader flags, each the CRC32 of a flag's name. Only saved by a
    /// [`BsLightingShaderProperty`] or [`BsEffectShaderProperty`], from stream version 132 on
    /// (Fallout 76, Starfield and the latest Fallout 4 files).
    pub fn crc_flags_1(&self) -> &[u32] {
        ffi::shader_crc_flags(&self.0, 0)
    }

    /// The second list of shader flags, saved like [`crc_flags_1`](Self::crc_flags_1) but from
    /// stream version 152 on.
    pub fn crc_flags_2(&self) -> &[u32] {
        ffi::shader_crc_flags(&self.0, 1)
    }

    pub fn set_crc_flags_1(&mut self, flags: &[u32]) {
        ffi::set_shader_crc_flags(self.ffi_mut(), 0, flags);
    }

    pub fn set_crc_flags_2(&mut self, flags: &[u32]) {
        ffi::set_shader_crc_flags(self.ffi_mut(), 1, flags);
    }

    fields! {
        /// Offset added to texture coordinates, as `[u, v]`. Saved from Skyrim on.
        uv_offset, set_uv_offset: [f32; 2] = shader_uv_offset, set_shader_uv_offset, uv, uv_to_ffi;
        /// Scale applied to texture coordinates, as `[u, v]`. Saved from Skyrim on.
        uv_scale, set_uv_scale: [f32; 2] = shader_uv_scale, set_shader_uv_scale, uv, uv_to_ffi;
        /// Strength of the environment map. Saved by Fallout 3 and New Vegas shaders, and by a
        /// [`BsLightingShaderProperty`] of type
        /// [`EnvironmentMap`](LightingShaderType::EnvironmentMap).
        environment_map_scale, set_environment_map_scale: f32 =
            shader_environment_map_scale, set_shader_environment_map_scale, same, same;
    }

    /// The shader type of a Fallout 3 or New Vegas shader, or `None` if the stored value isn't a
    /// [`LegacyShaderType`]. Only saved in Fallout 3 and New Vegas files; a
    /// [`BsLightingShaderProperty`] has its own [type](BsLightingShaderProperty::shader_type).
    pub fn legacy_shader_type(&self) -> Option<LegacyShaderType> {
        LegacyShaderType::try_from(ffi::shader_legacy_type(&self.0)).ok()
    }

    /// Sets the [legacy shader type](Self::legacy_shader_type).
    pub fn set_legacy_shader_type(&mut self, shader_type: LegacyShaderType) {
        ffi::set_shader_legacy_type(self.ffi_mut(), shader_type.into());
    }
}

/// # Shaders, materials and textures
///
/// A shape's shader, material, texture set and texture paths.
impl NifFile {

    /// Normalizes every texture path in the file's texture sets, effect shaders and texturing
    /// properties. Loading already does this. Each path is:
    ///
    /// - trimmed of whitespace;
    /// - given single backslashes as separators;
    /// - cut to start after a `textures\` folder inside it, and stripped of leading backslashes;
    /// - for relative paths, prefixed with `textures\` (except in Morrowind and Oblivion
    ///   files), and with `Data\` in terrain files.
    pub fn trim_texture_paths(&mut self) {
        nif_ffi::NifFile::TrimTexturePaths(self.inner.pin_mut());
    }

    /// Adds an [`NiSourceTexture`] for the external texture file `path`, attached to nothing, and
    /// returns its index. Attach it with [`Edit::set_map_source`] or
    /// [`Edit::set_source_texture`]; saving drops it if nothing refers to it.
    pub fn add_source_texture(&mut self, path: impl AsRef<[u8]>) -> Index {
        index_from_ffi(nif_ffi::add_source_texture(self.inner.pin_mut(), path.as_ref()))
            .expect("a new block has an index")
    }
}

fn rgb(c: ffi::Vec3) -> [f32; 3] {
    [c.x, c.y, c.z]
}

fn rgb_to_ffi([x, y, z]: [f32; 3]) -> ffi::Vec3 {
    ffi::Vec3 { x, y, z }
}

/// A position and width in bits within packed flags.
pub(crate) type BitField = (u16, u16);

pub(crate) fn get_bits(bits: u16, (pos, width): BitField) -> u32 {
    u32::from((bits >> pos) & ((1 << width) - 1))
}

pub(crate) fn set_bits(bits: u16, (pos, width): BitField, value: u32) -> u16 {
    let mask = ((1 << width) - 1) << pos;
    // Every enum value fits its field; anything wider is masked off.
    (bits & !mask) | (((value as u16) << pos) & mask)
}

/// Getter and setter pairs for texture paths, over the bridge's `$ffi_get` and `$ffi_set`.
macro_rules! texture_paths {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> &BStr {
            ffi::$ffi_get(&self.0).as_bstr()
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, path: impl AsRef<[u8]>) {
            ffi::$ffi_set(self.ffi_mut(), path.as_ref());
        }
    )*};
}

mod flags;
mod handles;
mod material;
mod texturing;
mod lighting;
mod effect;
mod textures;
mod properties;
mod legacy;

pub use self::flags::*;
pub use self::material::*;
pub use self::texturing::*;
pub use self::lighting::*;
pub use self::textures::*;
pub use self::properties::*;
pub use self::legacy::*;
