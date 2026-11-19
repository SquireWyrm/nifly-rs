//! Properties and shaders: `NiProperty`, `NiShader` and its subclasses.
//!
//! ```text
//! BsLightingShaderProperty -> BsShaderProperty -> NiShadeProperty -> NiShader -> NiProperty -> NiObjectNet
//! ```

use std::pin::Pin;

use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    nif_file::ffi::{self as nif_ffi, RefKind, RefList},
    shaders::ffi,
};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, Index, NifFile},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    geometry::{NiShape, TextureSource},
    object3d::Vec3,
    objects::{IsA, NiAvObject, NiObject, NiObjectNet, NiType},
};

/// Getter and setter pairs over the bridge's field shims, converting with `$to` and `$from`.
macro_rules! fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident, $to:expr, $from:expr;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ($to)(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, value: $ty) {
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
    pub fn set_path(self: Pin<&mut Self>, index: usize, path: impl AsRef<[u8]>) -> bool {
        ffi::set_texture_set_path(self.ffi_mut(), index, path.as_ref())
    }

    /// Replaces every slot, changing the number of slots to the number of paths.
    pub fn set_paths<P: AsRef<[u8]>>(mut self: Pin<&mut Self>, paths: impl IntoIterator<Item = P>) {
        let paths: Vec<P> = paths.into_iter().collect();
        ffi::set_texture_set_len(self.as_mut().ffi_mut(), paths.len());
        for (i, path) in paths.iter().enumerate() {
            ffi::set_texture_set_path(self.as_mut().ffi_mut(), i, path.as_ref());
        }
    }
}

impl NiShadeProperty {
    fields! {
        /// The shading flags, as raw bits: bit 0 is smooth shading. Only saved before Skyrim.
        shading_flags, set_shading_flags: u16 = shade_flags, set_shade_flags, same, same;
    }
}

impl BsShaderProperty {
    /// The first shader flags field, as raw bits.
    /// For example, bit 7 is environment mapping and bit 29 external emittance.
    ///
    /// Not saved from stream version 132 on (Fallout 76, Starfield and the latest Fallout 4
    /// files), which save [`crc_flags_1`](Self::crc_flags_1) instead. The two aren't converted
    /// into each other.
    pub fn flags_1(&self) -> u32 {
        ffi::shader_flags_1(&self.0)
    }

    /// The second shader flags field, as raw bits.
    /// For example, bit 4 is double-sided and bit 5 vertex colors.
    ///
    /// Not saved from stream version 132 on. Those files save [`crc_flags_2`](Self::crc_flags_2)
    /// instead.
    pub fn flags_2(&self) -> u32 {
        ffi::shader_flags_2(&self.0)
    }

    pub fn set_flags_1(self: Pin<&mut Self>, flags: u32) {
        ffi::set_shader_flags_1(self.ffi_mut(), flags);
    }

    pub fn set_flags_2(self: Pin<&mut Self>, flags: u32) {
        ffi::set_shader_flags_2(self.ffi_mut(), flags);
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

    pub fn set_crc_flags_1(self: Pin<&mut Self>, flags: &[u32]) {
        ffi::set_shader_crc_flags(self.ffi_mut(), 0, flags);
    }

    pub fn set_crc_flags_2(self: Pin<&mut Self>, flags: &[u32]) {
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
    pub fn set_legacy_shader_type(self: Pin<&mut Self>, shader_type: LegacyShaderType) {
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

/// Reading a shape.
impl<'a> View<'a, NiShape> {
    /// The shape's shader, or `None` if it has none.
    ///
    /// Checks the shape's shader property ref first (`BSTriShape`, and `NiTriShape` from Skyrim
    /// on), then its property list (older games). In the property list, an
    /// [`NiMaterialProperty`] is only returned if there is no other shader.
    pub fn shader(&self) -> Option<View<'a, NiShader>> {
        let ptr = nif_ffi::shader(&self.file().inner, self.get().as_ffi());
        // SAFETY: the shader is owned by this file, which outlives `'a`.
        self.file().view_of(unsafe { ptr.as_ref() }.map(NiShader::from_ffi)?)
    }

    /// The shape's alpha property, or `None` if it has none.
    ///
    /// Checks the shape's alpha property ref first (Skyrim and later), then its property list
    /// (older games).
    pub fn alpha_property(&self) -> Option<View<'a, NiAlphaProperty>> {
        let ptr = nif_ffi::alpha_property(&self.file().inner, self.get().as_ffi());
        // SAFETY: as in `shader`.
        self.file().view_of(unsafe { ptr.as_ref() }.map(NiAlphaProperty::from_ffi)?)
    }

    /// The first [`NiStencilProperty`] in the shape's property list, or `None` if it has none.
    /// Found in files before Skyrim.
    pub fn stencil_property(&self) -> Option<View<'a, NiStencilProperty>> {
        let ptr = nif_ffi::stencil_property(&self.file().inner, self.get().as_ffi());
        // SAFETY: as in `shader`.
        self.file().view_of(unsafe { ptr.as_ref() }.map(NiStencilProperty::from_ffi)?)
    }

    /// The first [`NiTexturingProperty`] in the shape's property list, or `None` if it has
    /// none. Found in files before Skyrim.
    pub fn texturing_property(&self) -> Option<View<'a, NiTexturingProperty>> {
        let ptr = nif_ffi::texturing_property(&self.file().inner, self.get().as_ffi());
        // SAFETY: as in `shader`.
        self.file().view_of(unsafe { ptr.as_ref() }.map(NiTexturingProperty::from_ffi)?)
    }

    /// The first [`NiMaterialProperty`] in the shape's property list, or `None` if it has none.
    /// Found in files before Skyrim. Unlike [`shader`](Self::shader), this finds it even when
    /// the shape has another shader too.
    pub fn material_property(&self) -> Option<View<'a, NiMaterialProperty>> {
        let ptr = nif_ffi::material_property(&self.file().inner, self.get().as_ffi());
        // SAFETY: as in `shader`.
        self.file().view_of(unsafe { ptr.as_ref() }.map(NiMaterialProperty::from_ffi)?)
    }


    /// Every texture path the shape uses, in this order:
    /// 1. the slots of the shader's `BSShaderTextureSet` (empty slots included, as `b""`),
    /// 2. for a `BSEffectShaderProperty`: its source, normal, greyscale, environment map and
    ///    environment mask textures,
    /// 3. for older files' `NiTexturingProperty`: the file name of each texture it has (base,
    ///    dark, detail, gloss, glow, bump, decals).
    ///
    /// Empty if the shape has no shader or texturing property. Paths are as stored.
    pub fn texture_paths(&self) -> Vec<&'a BStr> {
        let (file, shape) = (self.file(), self.get());
        let count = nif_ffi::texture_path_count(&file.inner, shape.as_ffi());
        (0..count)
            .map(|i| nif_ffi::texture_path(&file.inner, shape.as_ffi(), i).as_bstr())
            .collect()
    }

    /// The texture path in slot `index` of the shape, and where it was found. `None` if the shape
    /// has no such slot. Looks in this order:
    ///
    /// 1. **Texture set:** if the shader has one, `index` is a slot in it (0 = diffuse,
    ///    1 = normal, ...). Out-of-range indices fall through to step 3.
    /// 2. **Effect shader:** if the shader is a `BSEffectShaderProperty` without a texture set,
    ///    0 = source, 1 = normal, 3 = greyscale, 4 = environment map, 5 = environment mask. Any
    ///    other index returns `Some` with an empty path.
    /// 3. **Texturing property** (older files): 0 = base, 1 = dark, 2 = detail, 3 = gloss,
    ///    4 = glow, 5 = bump, 6..=9 = decals. Empty paths return `None`.
    ///
    /// Empty texture set slots return `Some` with an empty path.
    pub fn texture_slot(&self, index: usize) -> Option<(TextureSource, BString)> {
        let (file, shape) = (self.file(), self.get());
        let index = u32::try_from(index).ok()?;
        let slot = nif_ffi::texture_slot(&file.inner, shape.as_ffi(), index);
        let source = match slot.source {
            1 => TextureSource::TextureSet,
            2 => TextureSource::EffectShader,
            3 => TextureSource::TexturingProperty,
            _ => return None,
        };
        Some((source, BString::from(slot.path)))
    }
}

/// Changing a shape.
impl Edit<'_, NiShape> {
    /// Like [`View::shader`], for editing.
    pub fn shader(&mut self) -> Option<Edit<'_, NiShader>> {
        let index = self.view().shader()?.index();
        self.resolve(Some(index))
    }

    /// Like [`View::alpha_property`], for editing.
    pub fn alpha_property(&mut self) -> Option<Edit<'_, NiAlphaProperty>> {
        let index = self.view().alpha_property()?.index();
        self.resolve(Some(index))
    }

    /// Like [`View::stencil_property`], for editing.
    pub fn stencil_property(&mut self) -> Option<Edit<'_, NiStencilProperty>> {
        let index = self.view().stencil_property()?.index();
        self.resolve(Some(index))
    }

    /// Like [`View::texturing_property`], for editing.
    pub fn texturing_property(&mut self) -> Option<Edit<'_, NiTexturingProperty>> {
        let index = self.view().texturing_property()?.index();
        self.resolve(Some(index))
    }

    /// Like [`View::material_property`], for editing.
    pub fn material_property(&mut self) -> Option<Edit<'_, NiMaterialProperty>> {
        let index = self.view().material_property()?.index();
        self.resolve(Some(index))
    }


    /// Gives the shape a new [`NiAlphaProperty`] with these
    /// [flags](NiAlphaProperty::flags) and threshold, replacing any it had (as
    /// [`remove_alpha_property`](Self::remove_alpha_property) does). Returns the new block's
    /// index.
    ///
    /// The property goes in the shape's alpha property ref for Skyrim and later (and every
    /// `BSTriShape`), and in its property list for older files.
    ///
    /// ```
    /// # use nifly::{file::NifFile, geometry::NiShape, shaders::{AlphaFlags, NiAlphaProperty}};
    /// # let mut nif = NifFile::load(std::fs::File::open("assets/Cube.nif").unwrap(), None).unwrap();
    /// let shape = nif.shapes().indices().next().unwrap();
    /// let flags = AlphaFlags::DEFAULT.with_blending(true);
    /// nif.edit::<NiShape>(shape).unwrap().assign_alpha_property(flags, NiAlphaProperty::DEFAULT_THRESHOLD);
    /// ```
    pub fn assign_alpha_property(&mut self, flags: AlphaFlags, threshold: u8) -> Index {
        let shape = self.index() as u32;
        let index = nif_ffi::assign_alpha_property(self.file_mut().inner.pin_mut(), shape, flags.bits(), threshold);
        index_from_ffi(index).expect("the block is a shape")
    }

    /// Removes the alpha property of the shape. A property no other block
    /// uses is deleted, which moves every later block down one index; one shared with other
    /// shapes stays for them. Returns `false`, changing nothing, if the shape has no alpha
    /// property.
    pub fn remove_alpha_property(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::remove_alpha_property(file.inner.pin_mut(), shape))
    }

    /// Removes the shader of the shape: its shader, the shaders and
    /// materials in its property list, and its alpha property. Each is deleted, with its texture
    /// set and controllers, unless another block still uses it; other properties, such as
    /// texturing and stencil properties, stay.
    ///
    /// Deleting blocks moves later blocks down, so block indices change. Returns `false`,
    /// changing nothing, if the shape has none of these.
    pub fn delete_shader(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::delete_shader(file.inner.pin_mut(), shape))
    }

    /// Sets the texture path in slot `index` of the shape, using the same
    /// slot numbering as [`View::texture_slot`]. Does nothing if the shape has no such slot.
    ///
    /// For an [`NiTexturingProperty`] (older files), the map is added if the property doesn't
    /// have it, and its [texture count](NiTexturingProperty::texture_count) is raised if needed
    /// so the map is saved. A map without an [`NiSourceTexture`] gets a new one, added at the end
    /// of the block list, so existing block indices don't change.
    /// Points the shape's shader ref at the [`NiShader`] at `shader`, or clears it with `None`.
    /// Files before Skyrim save `NiGeometry` shaders from the
    /// [property list](Edit::set_properties) instead. Returns `false`, changing nothing, if
    /// that block isn't a shader.
    pub fn set_shader(&mut self, shader: Option<Index>) -> bool {
        self.set_ref(RefKind::Shader, shader)
    }

    /// Points the shape's alpha property ref at the [`NiAlphaProperty`] at `alpha`, or clears it
    /// with `None`. Files before Skyrim save `NiGeometry` alpha properties from the
    /// [property list](Edit::set_properties) instead. Returns `false`, changing nothing, if that
    /// block isn't one.
    pub fn set_alpha_property(&mut self, alpha: Option<Index>) -> bool {
        self.set_ref(RefKind::AlphaProperty, alpha)
    }

    pub fn set_texture_slot(&mut self, index: usize, path: impl AsRef<[u8]>) {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(index)) = (block_index_to_ffi(shape), u32::try_from(index)) else {
            return;
        };
        nif_ffi::set_texture_slot(file.inner.pin_mut(), shape, path.as_ref(), index);
    }
}

/// Properties of shapes and nodes.
impl<'a, T: NiType> View<'a, T> {
    /// The object's property list. Skips empty refs. Skyrim and later files don't save it.
    pub fn properties<I>(&self) -> BlockIter<'a, NiProperty, RefIndices>
    where
        T: IsA<NiAvObject, I>,
    {
        self.follow_list(RefList::Properties)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Replaces the object's property list. Returns `false`, changing nothing, unless each block
    /// is an [`NiProperty`].
    pub fn set_properties<I>(&mut self, properties: &[Index]) -> bool
    where
        T: IsA<NiAvObject, I>,
    {
        self.set_ref_list(RefList::Properties, properties)
    }

    /// Adds a new `P` with default settings to the object's property list, after any properties
    /// it has, and returns the new block's index. It's added at the end.
    ///
    /// Skyrim and later files don't save property lists, so there the property isn't attached
    /// when saved.
    ///
    /// ```
    /// # use nifly::{file::NifFile, geometry::NiShape, shaders::{DepthTest, NiZBufferProperty}};
    /// # let mut nif = NifFile::load(std::fs::File::open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif").unwrap(), None).unwrap();
    /// let shape = nif.shapes().indices().next().unwrap();
    /// let index = nif.edit::<NiShape>(shape).unwrap().add_property::<NiZBufferProperty, _>();
    /// nif.block_of_mut::<NiZBufferProperty>(index).unwrap().set_function(DepthTest::Less);
    /// ```
    pub fn add_property<P: NewProperty, I>(&mut self) -> Index
    where
        T: IsA<NiAvObject, I>,
    {
        let target = self.index() as u32;
        index_from_ffi(P::add(self.file_mut().inner.pin_mut(), target)).expect("the block is an NiAVObject")
    }
}

/// The texture set of a shader.
impl<'a> View<'a, NiShader> {
    /// The shader's texture set, or `None` if it has none. Only [`BsLightingShaderProperty`] and
    /// [`BsShaderPpLightingProperty`] have one; a [`BsEffectShaderProperty`] stores its textures
    /// itself. Use [`View::texture_paths`] to read a shape's paths wherever they're stored.
    pub fn texture_set(&self) -> Option<View<'a, BsShaderTextureSet>> {
        self.resolve(index_from_ffi(ffi::shader_texture_set(&self.get().0)))
    }
}

impl Edit<'_, NiShader> {
    /// Like [`View::texture_set`], for editing.
    pub fn texture_set(&mut self) -> Option<Edit<'_, BsShaderTextureSet>> {
        let index = self.view().texture_set()?.index();
        self.resolve(Some(index))
    }

    /// Points the shader at the [`BsShaderTextureSet`] at `set`, or clears it with `None`.
    /// Returns `false`, changing nothing, if the shader has no texture set ref or that block
    /// isn't one.
    pub fn set_texture_set(&mut self, set: Option<Index>) -> bool {
        self.set_ref(RefKind::TextureSet, set)
    }
}

/// The source textures of an [`NiTexturingProperty`]'s maps.
impl<'a> View<'a, NiTexturingProperty> {
    /// `map`'s source texture, or `None` if the property doesn't have `map` or it has no texture.
    pub fn map_source(&self, map: impl Into<MapSlot>) -> Option<View<'a, NiSourceTexture>> {
        self.resolve(self.get().map_source_index(map))
    }
}

impl Edit<'_, NiTexturingProperty> {
    /// Like [`View::map_source`], for editing.
    pub fn map_source(&mut self, map: impl Into<MapSlot>) -> Option<Edit<'_, NiSourceTexture>> {
        let index = self.view().map_source(map)?.index();
        self.resolve(Some(index))
    }

    /// Gives `map` the [`NiSourceTexture`] at block index `source`, adding the map if the
    /// property doesn't have it, or removes the map if `source` is `None`. Adding raises the
    /// property's [texture count](NiTexturingProperty::texture_count) if needed so the map is
    /// saved. Files before Fallout 3 can't save [`Normal`](TextureMap::Normal) or
    /// [`Parallax`](TextureMap::Parallax) maps, so there those are lost on save.
    ///
    /// A [shader map](MapSlot::Shader) must be in range of
    /// [`shader_map_count`](NiTexturingProperty::shader_map_count); adding one marks it used.
    ///
    /// Returns `false`, changing nothing, if `source` isn't an `NiSourceTexture` or the shader
    /// map is out of range.
    ///
    /// ```
    /// # use nifly::{file::NifFile, shaders::{NiTexturingProperty, TextureMap}};
    /// # let mut nif = NifFile::load(std::fs::File::open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif").unwrap(), None).unwrap();
    /// let property = nif.blocks_of::<NiTexturingProperty>().indices().next().unwrap();
    /// let glow = nif.add_source_texture(r"textures\glow.dds");
    /// assert!(nif.edit::<NiTexturingProperty>(property).unwrap().set_map_source(TextureMap::Glow, Some(glow)));
    /// ```
    pub fn set_map_source(&mut self, map: impl Into<MapSlot>, source: Option<Index>) -> bool {
        let source = match source {
            Some(source) => match block_index_to_ffi(source) {
                Some(source) => source,
                None => return false,
            },
            None => u32::MAX,
        };
        let property = self.index() as u32;
        nif_ffi::set_map_source(self.file_mut().inner.pin_mut(), property, map.into().to_ffi(), source)
    }
}

/// The pixel data of an [`NiSourceTexture`].
impl<'a> View<'a, NiSourceTexture> {
    /// The texture's pixel data, or `None` if it has none. Cast it to an [`NiPixelData`] or
    /// [`NiPersistentSrcTextureRendererData`].
    pub fn pixel_data(&self) -> Option<View<'a, TextureRenderData>> {
        self.resolve(self.get().pixel_data_index())
    }
}

impl Edit<'_, NiSourceTexture> {
    /// Like [`View::pixel_data`], for editing.
    pub fn pixel_data(&mut self) -> Option<Edit<'_, TextureRenderData>> {
        let index = self.view().pixel_data()?.index();
        self.resolve(Some(index))
    }

    /// Gives the texture a new `T` with default settings as its pixel data, replacing any it
    /// had, and returns the new block's index. It's added at the end. Replaced pixel data stays
    /// in the file until saving drops it, if nothing else uses it.
    pub fn assign_pixel_data<T: NewPixelData>(&mut self) -> Index {
        let texture = self.index() as u32;
        index_from_ffi(T::assign(self.file_mut().inner.pin_mut(), texture)).expect("the block is an NiSourceTexture")
    }

    /// Points the texture at the [`TextureRenderData`] at `data`, or clears it with `None`. It's
    /// saved only for an internal texture. Returns `false`, changing nothing, if that block
    /// isn't one.
    pub fn set_pixel_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::PixelData, data)
    }
}

/// The palette of a [`TextureRenderData`].
impl<'a> View<'a, TextureRenderData> {
    /// The pixel data's palette, or `None` if it has none.
    pub fn palette(&self) -> Option<View<'a, NiPalette>> {
        self.resolve(index_from_ffi(ffi::render_data_palette(&self.get().0)))
    }
}

impl Edit<'_, TextureRenderData> {
    /// Like [`View::palette`], for editing.
    pub fn palette(&mut self) -> Option<Edit<'_, NiPalette>> {
        let index = self.view().palette()?.index();
        self.resolve(Some(index))
    }

    /// Gives the pixel data a new [`NiPalette`] with these colors, as `[r, g, b, a]`, replacing
    /// any it had, and returns the palette's index. Returns `None`, changing nothing, unless
    /// there are 16 or 256 colors.
    pub fn assign_palette(&mut self, colors: &[[u8; 4]], has_alpha: bool) -> Option<Index> {
        let data = self.index() as u32;
        index_from_ffi(nif_ffi::assign_palette(self.file_mut().inner.pin_mut(), data, colors.as_flattened(), has_alpha))
    }

    /// Points the pixel data at the [`NiPalette`] at `palette`, or clears it with `None`. Returns
    /// `false`, changing nothing, if that block isn't one.
    pub fn set_palette(&mut self, palette: Option<Index>) -> bool {
        self.set_ref(RefKind::Palette, palette)
    }
}

fn rgb(c: ffi::Vec3) -> [f32; 3] {
    [c.x, c.y, c.z]
}

fn rgb_to_ffi([x, y, z]: [f32; 3]) -> ffi::Vec3 {
    ffi::Vec3 { x, y, z }
}

impl NiMaterialProperty {
    fields! {
        /// Flags of unknown meaning. Only saved in files from version 3.0 up to 10.0.1.2, which
        /// include Morrowind's.
        legacy_flags, set_legacy_flags: u16 = material_legacy_flags, set_material_legacy_flags, same, same;
    }

    /// Ambient color. Only saved in Oblivion and older files.
    pub fn ambient(&self) -> [f32; 3] {
        rgb(ffi::material_ambient(&self.0))
    }

    /// Diffuse color. Only saved in Oblivion and older files.
    pub fn diffuse(&self) -> [f32; 3] {
        rgb(ffi::material_diffuse(&self.0))
    }

    pub fn specular(&self) -> [f32; 3] {
        rgb(ffi::material_specular(&self.0))
    }

    pub fn emissive(&self) -> [f32; 3] {
        rgb(ffi::material_emissive(&self.0))
    }

    pub fn glossiness(&self) -> f32 {
        ffi::material_glossiness(&self.0)
    }

    pub fn alpha(&self) -> f32 {
        ffi::material_alpha(&self.0)
    }

    /// Multiplier for the emissive color. Only saved in Fallout 3 and later files; 1.0 otherwise.
    pub fn emissive_multiple(&self) -> f32 {
        ffi::material_emissive_multiple(&self.0)
    }

    /// Whether the emissive color isn't black (exactly zero).
    pub fn is_emissive(&self) -> bool {
        ffi::material_is_emissive(&self.0)
    }

    /// Whether the specular color isn't black (exactly zero).
    pub fn has_specular(&self) -> bool {
        ffi::material_has_specular(&self.0)
    }

    pub fn set_ambient(self: Pin<&mut Self>, color: [f32; 3]) {
        ffi::set_material_ambient(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_diffuse(self: Pin<&mut Self>, color: [f32; 3]) {
        ffi::set_material_diffuse(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_specular(self: Pin<&mut Self>, color: [f32; 3]) {
        ffi::set_material_specular(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_emissive(self: Pin<&mut Self>, color: [f32; 3]) {
        ffi::set_material_emissive(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_glossiness(self: Pin<&mut Self>, glossiness: f32) {
        ffi::set_material_glossiness(self.ffi_mut(), glossiness);
    }

    pub fn set_alpha(self: Pin<&mut Self>, alpha: f32) {
        ffi::set_material_alpha(self.ffi_mut(), alpha);
    }

    pub fn set_emissive_multiple(self: Pin<&mut Self>, multiple: f32) {
        ffi::set_material_emissive_multiple(self.ffi_mut(), multiple);
    }
}

impl NiAlphaProperty {
    /// The threshold of a new alpha property.
    pub const DEFAULT_THRESHOLD: u8 = 128;

    /// The blending and testing settings.
    pub fn flags(&self) -> AlphaFlags {
        AlphaFlags::from_bits(ffi::alpha_flags(&self.0))
    }

    /// The alpha test threshold. Pixels are tested against it with the
    /// [test function](AlphaFlags::test_function).
    pub fn threshold(&self) -> u8 {
        ffi::alpha_threshold(&self.0)
    }

    pub fn set_flags(self: Pin<&mut Self>, flags: AlphaFlags) {
        ffi::set_alpha_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_threshold(self: Pin<&mut Self>, threshold: u8) {
        ffi::set_alpha_threshold(self.ffi_mut(), threshold);
    }
}

impl NiStencilProperty {
    fields! {
        /// Flags of unknown meaning. Only saved in files from version 3.0 up to 10.0.1.2, which
        /// include Morrowind's.
        legacy_flags, set_legacy_flags: u16 = stencil_legacy_flags, set_stencil_legacy_flags, same, same;
    }

    /// The packed settings (Fallout 3 and later).
    pub fn flags(&self) -> StencilFlags {
        StencilFlags::from_bits(ffi::stencil_flags(&self.0))
    }

    /// Whether stencil testing is on (Oblivion and older).
    pub fn enabled(&self) -> bool {
        ffi::stencil_enabled(&self.0)
    }

    /// The test function (Oblivion and older). `None` if the stored value isn't a test function.
    pub fn function(&self) -> Option<StencilTest> {
        StencilTest::try_from(ffi::stencil_function(&self.0)).ok()
    }

    /// The value tested against.
    pub fn reference(&self) -> u32 {
        ffi::stencil_reference(&self.0)
    }

    /// The mask applied before testing.
    pub fn mask(&self) -> u32 {
        ffi::stencil_mask(&self.0)
    }

    /// The action when the stencil test fails (Oblivion and older). `None` if the stored value
    /// isn't an action.
    pub fn fail_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_fail_action(&self.0)).ok()
    }

    /// The action when the stencil test passes but the depth test fails (Oblivion and older).
    /// `None` if the stored value isn't an action.
    pub fn z_fail_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_z_fail_action(&self.0)).ok()
    }

    /// The action when both tests pass (Oblivion and older). `None` if the stored value isn't an
    /// action.
    pub fn pass_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_pass_action(&self.0)).ok()
    }

    /// Which faces are drawn (Oblivion and older). `None` if the stored value isn't a draw mode.
    pub fn draw_mode(&self) -> Option<DrawMode> {
        DrawMode::try_from(ffi::stencil_draw_mode(&self.0)).ok()
    }

    pub fn set_flags(self: Pin<&mut Self>, flags: StencilFlags) {
        ffi::set_stencil_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_enabled(self: Pin<&mut Self>, enabled: bool) {
        ffi::set_stencil_enabled(self.ffi_mut(), enabled);
    }

    pub fn set_function(self: Pin<&mut Self>, function: StencilTest) {
        ffi::set_stencil_function(self.ffi_mut(), function.into());
    }

    pub fn set_reference(self: Pin<&mut Self>, reference: u32) {
        ffi::set_stencil_reference(self.ffi_mut(), reference);
    }

    pub fn set_mask(self: Pin<&mut Self>, mask: u32) {
        ffi::set_stencil_mask(self.ffi_mut(), mask);
    }

    pub fn set_fail_action(self: Pin<&mut Self>, action: StencilAction) {
        ffi::set_stencil_fail_action(self.ffi_mut(), action.into());
    }

    pub fn set_z_fail_action(self: Pin<&mut Self>, action: StencilAction) {
        ffi::set_stencil_z_fail_action(self.ffi_mut(), action.into());
    }

    pub fn set_pass_action(self: Pin<&mut Self>, action: StencilAction) {
        ffi::set_stencil_pass_action(self.ffi_mut(), action.into());
    }

    pub fn set_draw_mode(self: Pin<&mut Self>, mode: DrawMode) {
        ffi::set_stencil_draw_mode(self.ffi_mut(), mode.into());
    }
}

/// A position and width in bits within packed flags.
type BitField = (u16, u16);

fn get_bits(bits: u16, (pos, width): BitField) -> u32 {
    u32::from((bits >> pos) & ((1 << width) - 1))
}

fn set_bits(bits: u16, (pos, width): BitField, value: u32) -> u16 {
    let mask = ((1 << width) - 1) << pos;
    // Every enum value fits its field; anything wider is masked off.
    (bits & !mask) | (((value as u16) << pos) & mask)
}

/// How a color and the color already drawn are weighted when blending.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum BlendFactor {
    /// 1.
    One = 0,
    /// 0.
    Zero = 1,
    /// The source color.
    SourceColor = 2,
    /// 1 minus the source color.
    InverseSourceColor = 3,
    /// The destination color.
    DestinationColor = 4,
    /// 1 minus the destination color.
    InverseDestinationColor = 5,
    /// The source alpha.
    SourceAlpha = 6,
    /// 1 minus the source alpha.
    InverseSourceAlpha = 7,
    /// The destination alpha.
    DestinationAlpha = 8,
    /// 1 minus the destination alpha.
    InverseDestinationAlpha = 9,
    /// The smaller of the source alpha and 1 minus the destination alpha.
    SourceAlphaSaturate = 10,
}

/// An alpha test function: how a pixel's alpha compares with the
/// [threshold](NiAlphaProperty::threshold) for the pixel to be drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum AlphaTest {
    /// Always drawn.
    Always = 0,
    /// Drawn if the alpha is less than the threshold.
    Less = 1,
    /// Drawn if they're equal.
    Equal = 2,
    /// Drawn if the alpha is less than or equal to the threshold.
    LessOrEqual = 3,
    /// Drawn if the alpha is greater than the threshold.
    Greater = 4,
    /// Drawn if they differ.
    NotEqual = 5,
    /// Drawn if the alpha is greater than or equal to the threshold.
    GreaterOrEqual = 6,
    /// Never drawn.
    Never = 7,
}

/// The packed blending and testing settings of an [`NiAlphaProperty`].
///
/// Bits:
/// - 0: alpha blending
/// - 1–4: source blend factor
/// - 5–8: destination blend factor
/// - 9: alpha testing
/// - 10–12: test function
/// - 13: no sorting
///
/// The `with_*` methods replace one setting and keep the others.
///
/// ```
/// # use nifly::shaders::{AlphaFlags, AlphaTest};
/// let flags = AlphaFlags::DEFAULT.with_blending(true).with_test_function(AlphaTest::GreaterOrEqual);
/// assert!(flags.blending() && flags.testing());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AlphaFlags(u16);

impl Default for AlphaFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl AlphaFlags {
    /// The flags of a new alpha property: blending off, factors
    /// [`SourceAlpha`](BlendFactor::SourceAlpha) and
    /// [`InverseSourceAlpha`](BlendFactor::InverseSourceAlpha), testing on with
    /// [`Greater`](AlphaTest::Greater), sorting on.
    pub const DEFAULT: Self = Self(4844);

    const BLENDING: BitField = (0, 1);
    const SOURCE: BitField = (1, 4);
    const DESTINATION: BitField = (5, 4);
    const TESTING: BitField = (9, 1);
    const TEST: BitField = (10, 3);
    const NO_SORTING: BitField = (13, 1);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    fn field(self, field: BitField) -> u32 {
        get_bits(self.0, field)
    }

    fn with_field(self, field: BitField, value: u32) -> Self {
        Self(set_bits(self.0, field, value))
    }

    /// Whether alpha blending is on.
    pub fn blending(self) -> bool {
        self.field(Self::BLENDING) != 0
    }

    /// The factor for the color being drawn. `None` if the bits hold 11 or more.
    pub fn source_blend(self) -> Option<BlendFactor> {
        BlendFactor::try_from(self.field(Self::SOURCE)).ok()
    }

    /// The factor for the color already drawn. `None` if the bits hold 11 or more.
    pub fn destination_blend(self) -> Option<BlendFactor> {
        BlendFactor::try_from(self.field(Self::DESTINATION)).ok()
    }

    /// Whether alpha testing is on.
    pub fn testing(self) -> bool {
        self.field(Self::TESTING) != 0
    }

    pub fn test_function(self) -> AlphaTest {
        AlphaTest::try_from(self.field(Self::TEST)).expect("3 bits are always a test function")
    }

    /// Whether the shape is drawn without being sorted by depth.
    pub fn no_sorting(self) -> bool {
        self.field(Self::NO_SORTING) != 0
    }

    #[must_use]
    pub fn with_blending(self, on: bool) -> Self {
        self.with_field(Self::BLENDING, on.into())
    }

    #[must_use]
    pub fn with_source_blend(self, factor: BlendFactor) -> Self {
        self.with_field(Self::SOURCE, factor.into())
    }

    #[must_use]
    pub fn with_destination_blend(self, factor: BlendFactor) -> Self {
        self.with_field(Self::DESTINATION, factor.into())
    }

    #[must_use]
    pub fn with_testing(self, on: bool) -> Self {
        self.with_field(Self::TESTING, on.into())
    }

    #[must_use]
    pub fn with_test_function(self, function: AlphaTest) -> Self {
        self.with_field(Self::TEST, function.into())
    }

    #[must_use]
    pub fn with_no_sorting(self, on: bool) -> Self {
        self.with_field(Self::NO_SORTING, on.into())
    }
}

/// A stencil test function: how the reference compares with the stored stencil value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum StencilTest {
    /// Never passes.
    Never = 0,
    /// Passes if the reference is less than the stored value.
    Less = 1,
    /// Passes if they're equal.
    Equal = 2,
    /// Passes if the reference is less than or equal to the stored value.
    LessOrEqual = 3,
    /// Passes if the reference is greater than the stored value.
    Greater = 4,
    /// Passes if they differ.
    NotEqual = 5,
    /// Passes if the reference is greater than or equal to the stored value.
    GreaterOrEqual = 6,
    /// Always passes.
    Always = 7,
}

/// What happens to the stored stencil value after a test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum StencilAction {
    /// Leaves it unchanged.
    Keep = 0,
    /// Sets it to 0.
    Zero = 1,
    /// Sets it to the reference.
    Replace = 2,
    /// Adds 1.
    Increment = 3,
    /// Subtracts 1.
    Decrement = 4,
    /// Inverts its bits.
    Invert = 5,
}

/// Which faces of a shape are drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum DrawMode {
    /// The application's default: counterclockwise faces, or both.
    Default = 0,
    /// Counterclockwise (front) faces.
    CounterClockwise = 1,
    /// Clockwise (back) faces.
    Clockwise = 2,
    /// Both sides.
    Both = 3,
}

/// The packed stencil settings of Fallout 3 and later files, from
/// [`NiStencilProperty::flags`].
///
/// Bits:
/// - 0: stencil testing
/// - 1–3: fail action
/// - 4–6: z-fail action
/// - 7–9: pass action
/// - 10–11: draw mode
/// - 12–14: test function
///
/// The `with_*` methods replace one setting and keep the others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StencilFlags(u16);

impl Default for StencilFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl StencilFlags {
    /// The flags of a new stencil property: testing off, [`Greater`](StencilTest::Greater),
    /// pass action [`Increment`](StencilAction::Increment), draw mode [`Both`](DrawMode::Both).
    pub const DEFAULT: Self = Self(19840);

    const FAIL: BitField = (1, 3);
    const Z_FAIL: BitField = (4, 3);
    const PASS: BitField = (7, 3);
    const DRAW: BitField = (10, 2);
    const TEST: BitField = (12, 3);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    fn field(self, field: BitField) -> u32 {
        get_bits(self.0, field)
    }

    fn with_field(self, field: BitField, value: u32) -> Self {
        Self(set_bits(self.0, field, value))
    }

    /// Whether stencil testing is on.
    pub fn enabled(self) -> bool {
        self.0 & 1 != 0
    }

    pub fn function(self) -> StencilTest {
        StencilTest::try_from(self.field(Self::TEST)).expect("3 bits are always a test function")
    }

    /// `None` if the bits hold 6 or 7.
    pub fn fail_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::FAIL)).ok()
    }

    /// `None` if the bits hold 6 or 7.
    pub fn z_fail_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::Z_FAIL)).ok()
    }

    /// `None` if the bits hold 6 or 7.
    pub fn pass_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::PASS)).ok()
    }

    pub fn draw_mode(self) -> DrawMode {
        DrawMode::try_from(self.field(Self::DRAW)).expect("2 bits are always a draw mode")
    }

    #[must_use]
    pub fn with_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !1) | u16::from(enabled))
    }

    #[must_use]
    pub fn with_function(self, function: StencilTest) -> Self {
        self.with_field(Self::TEST, function.into())
    }

    #[must_use]
    pub fn with_fail_action(self, action: StencilAction) -> Self {
        self.with_field(Self::FAIL, action.into())
    }

    #[must_use]
    pub fn with_z_fail_action(self, action: StencilAction) -> Self {
        self.with_field(Self::Z_FAIL, action.into())
    }

    #[must_use]
    pub fn with_pass_action(self, action: StencilAction) -> Self {
        self.with_field(Self::PASS, action.into())
    }

    #[must_use]
    pub fn with_draw_mode(self, mode: DrawMode) -> Self {
        self.with_field(Self::DRAW, mode.into())
    }
}

/// # Maps
///
/// A map is one of the fixed [`TextureMap`]s or a numbered [shader map](MapSlot::Shader). Its
/// fields are read and set through the `map_*` methods, which return `None` or `false`,
/// changing nothing, if the property doesn't have the map.
///
/// # Fields by version
///
/// Morrowind files are version 4.0.0.2, Oblivion's 20.0.0.4 or 20.0.0.5, and Fallout 3's and
/// New Vegas' 20.2.0.7. Fields a file's version doesn't store can still be read and set, but
/// aren't written when it's saved.
impl NiTexturingProperty {
    /// How the base map combines with the vertex and material colors, or `None` if the stored
    /// value isn't an apply mode. Saved only before Fallout 3; later files keep it in
    /// [`flags`](Self::flags).
    pub fn apply_mode(&self) -> Option<ApplyMode> {
        ApplyMode::try_from(ffi::texturing_apply_mode(&self.0)).ok()
    }

    pub fn set_apply_mode(self: Pin<&mut Self>, mode: ApplyMode) {
        ffi::set_texturing_apply_mode(self.ffi_mut(), mode.into());
    }

    fields! {
        /// The raw flags. From Fallout 3 on, bits 1–3 hold the apply mode. Saved in files up to
        /// version 10.0.1.2 and from 20.1.0.2 on.
        flags, set_flags: u16 = texturing_flags, set_texturing_flags, same, same;
        /// The number of map slots saved. The first five maps (base to glow) are always saved;
        /// later maps past the count aren't, and are lost on save. Files before Fallout 3 have no
        /// normal or parallax maps, so there the decals are slots 6 to 9.
        texture_count, set_texture_count: u32 = texturing_texture_count, set_texturing_texture_count, same, same;
        /// Scale of the bump map's luminance. Saved with the [bump map](TextureMap::Bump).
        bump_luma_scale, set_bump_luma_scale: f32 = texturing_luma_scale, set_texturing_luma_scale, same, same;
        /// Offset of the bump map's luminance. Saved with the [bump map](TextureMap::Bump).
        bump_luma_offset, set_bump_luma_offset: f32 =
            texturing_luma_offset, set_texturing_luma_offset, same, same;
        /// The bump map's 2×2 matrix, by rows. Saved with the [bump map](TextureMap::Bump).
        bump_matrix, set_bump_matrix: [f32; 4] = texturing_bump_matrix, set_texturing_bump_matrix, same, same;
        /// Offset of the parallax map. Saved with the [parallax map](TextureMap::Parallax).
        parallax_offset, set_parallax_offset: f32 =
            texturing_parallax_offset, set_texturing_parallax_offset, same, same;
    }

    /// Whether the property has `map`. A shader map counts while it's used.
    pub fn has_map(&self, map: impl Into<MapSlot>) -> bool {
        ffi::texturing_has_map(&self.0, map.into().to_ffi())
    }

    /// The block index of `map`'s source texture, or `None` if the property doesn't have `map`
    /// or its ref is empty. [`View::map_source`] gives the texture itself.
    pub fn map_source_index(&self, map: impl Into<MapSlot>) -> Option<Index> {
        index_from_ffi(ffi::texturing_map_source(&self.0, map.into().to_ffi()))
    }

    /// How `map` is repeated outside its UV range, or `None` if the stored value isn't a clamp
    /// mode. Only saved up to version 20.0.0.5 (Oblivion); later files keep it in
    /// [`map_flags`](Self::map_flags).
    pub fn map_clamp_mode(&self, map: impl Into<MapSlot>) -> Option<ClampMode> {
        ClampMode::try_from(self.map_field(map, ffi::texturing_map_clamp_mode)?).ok()
    }

    /// How `map` is filtered, or `None` if the stored value isn't a filter mode. Only saved up
    /// to version 20.0.0.5 (Oblivion); later files keep it in [`map_flags`](Self::map_flags).
    pub fn map_filter_mode(&self, map: impl Into<MapSlot>) -> Option<FilterMode> {
        FilterMode::try_from(self.map_field(map, ffi::texturing_map_filter_mode)?).ok()
    }

    /// The UV set `map` uses. Only saved up to version 20.0.0.5 (Oblivion); later files keep it
    /// in [`map_flags`](Self::map_flags).
    pub fn map_uv_set(&self, map: impl Into<MapSlot>) -> Option<u32> {
        self.map_field(map, ffi::texturing_map_uv_set)
    }

    /// `map`'s raw flags: bits 0–7 are the UV set, 8–11 the [filter mode](FilterMode) and 12–15
    /// the [clamp mode](ClampMode). Only saved from version 20.1.0.3 (Fallout 3) on; the
    /// separate fields aren't converted into them.
    pub fn map_flags(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_flags).map(|v| v as u16)
    }

    /// `map`'s maximum anisotropy. Only saved from version 20.5.0.4 on.
    pub fn map_max_anisotropy(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_max_anisotropy).map(|v| v as u16)
    }

    /// `map`'s PlayStation 2 L and K values, as `[l, k]`. Only saved before version 10.4.0.1.
    pub fn map_ps2(&self, map: impl Into<MapSlot>) -> Option<[i16; 2]> {
        let map = map.into();
        let l = self.map_field(map, ffi::texturing_map_ps2_l)?;
        let k = self.map_field(map, ffi::texturing_map_ps2_k)?;
        Some([l as i16, k as i16])
    }

    /// A value of unknown meaning stored with `map`. Only saved up to version 4.1.0.12.
    pub fn map_unknown(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_unknown).map(|v| v as u16)
    }

    /// `map`'s texture coordinate transform, or `None` if the property doesn't have `map` or
    /// `map` has no transform. Only saved from version 10.1.0.0 on.
    pub fn map_transform(&self, map: impl Into<MapSlot>) -> Option<TextureTransform> {
        let mut out = TextureTransform::default().to_ffi();
        ffi::texturing_map_transform(&self.0, map.into().to_ffi(), &mut out)
            .then(|| TextureTransform::from_ffi(out))
    }

    pub fn set_map_clamp_mode(self: Pin<&mut Self>, map: impl Into<MapSlot>, mode: ClampMode) -> bool {
        ffi::set_texturing_map_clamp_mode(self.ffi_mut(), map.into().to_ffi(), mode.into())
    }

    pub fn set_map_filter_mode(self: Pin<&mut Self>, map: impl Into<MapSlot>, mode: FilterMode) -> bool {
        ffi::set_texturing_map_filter_mode(self.ffi_mut(), map.into().to_ffi(), mode.into())
    }

    pub fn set_map_uv_set(self: Pin<&mut Self>, map: impl Into<MapSlot>, uv_set: u32) -> bool {
        ffi::set_texturing_map_uv_set(self.ffi_mut(), map.into().to_ffi(), uv_set)
    }

    pub fn set_map_flags(self: Pin<&mut Self>, map: impl Into<MapSlot>, flags: u16) -> bool {
        ffi::set_texturing_map_flags(self.ffi_mut(), map.into().to_ffi(), flags.into())
    }

    pub fn set_map_max_anisotropy(self: Pin<&mut Self>, map: impl Into<MapSlot>, value: u16) -> bool {
        ffi::set_texturing_map_max_anisotropy(self.ffi_mut(), map.into().to_ffi(), value.into())
    }

    pub fn set_map_ps2(mut self: Pin<&mut Self>, map: impl Into<MapSlot>, [l, k]: [i16; 2]) -> bool {
        let map = map.into().to_ffi();
        ffi::set_texturing_map_ps2_l(self.as_mut().ffi_mut(), map, l as u32)
            && ffi::set_texturing_map_ps2_k(self.ffi_mut(), map, k as u32)
    }

    pub fn set_map_unknown(self: Pin<&mut Self>, map: impl Into<MapSlot>, value: u16) -> bool {
        ffi::set_texturing_map_unknown(self.ffi_mut(), map.into().to_ffi(), value.into())
    }

    /// Gives `map` a texture coordinate transform, or removes it with `None`.
    pub fn set_map_transform(
        self: Pin<&mut Self>,
        map: impl Into<MapSlot>,
        transform: Option<TextureTransform>,
    ) -> bool {
        let value = transform.unwrap_or_default().to_ffi();
        ffi::set_texturing_map_transform(self.ffi_mut(), map.into().to_ffi(), transform.is_some(), &value)
    }

    /// The number of shader maps, used or not. Only saved from version 10.0.1.0 on.
    pub fn shader_map_count(&self) -> usize {
        ffi::texturing_shader_map_count(&self.0)
    }

    /// Changes the number of shader maps. New ones are unused; give them a texture with
    /// [`Edit::set_map_source`].
    pub fn set_shader_map_count(self: Pin<&mut Self>, count: usize) {
        ffi::set_texturing_shader_map_count(self.ffi_mut(), count);
    }

    /// The ID the shader uses for used shader map `index`, or `None` if it's unused or out of
    /// range.
    pub fn shader_map_id(&self, index: usize) -> Option<u32> {
        let mut out = 0;
        ffi::texturing_shader_map_id(&self.0, index, &mut out).then_some(out)
    }

    /// Returns `false`, changing nothing, if shader map `index` is unused or out of range.
    pub fn set_shader_map_id(self: Pin<&mut Self>, index: usize, id: u32) -> bool {
        ffi::set_texturing_shader_map_id(self.ffi_mut(), index, id)
    }

    fn map_field(
        &self,
        map: impl Into<MapSlot>,
        get: fn(&ffi::NiTexturingProperty, u32, &mut u32) -> bool,
    ) -> Option<u32> {
        let mut out = 0;
        get(&self.0, map.into().to_ffi(), &mut out).then_some(out)
    }
}

/// A map of an [`NiTexturingProperty`]: one of the fixed maps or a shader map. A
/// [`TextureMap`] converts into it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapSlot {
    Map(TextureMap),
    /// The shader map at this position, below
    /// [`shader_map_count`](NiTexturingProperty::shader_map_count).
    Shader(usize),
}

impl From<TextureMap> for MapSlot {
    fn from(map: TextureMap) -> Self {
        MapSlot::Map(map)
    }
}

impl MapSlot {
    /// The bridge's map number: the fixed maps, then shader maps from 12. Out of range for a
    /// shader map that can't exist.
    fn to_ffi(self) -> u32 {
        match self {
            MapSlot::Map(map) => map.into(),
            MapSlot::Shader(i) => u32::try_from(i).ok().and_then(|i| i.checked_add(12)).unwrap_or(u32::MAX),
        }
    }
}

/// A texture coordinate transform of an [`NiTexturingProperty`] map.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextureTransform {
    pub translation: [f32; 2],
    pub scale: [f32; 2],
    /// Rotation in radians.
    pub rotation: f32,
    /// How the parts combine: 0 and 2 for Maya's order, 1 for 3ds Max's.
    pub transform_type: u32,
    /// The center of rotation and scaling.
    pub center: [f32; 2],
}

impl TextureTransform {
    fn from_ffi(t: ffi::TextureTransform) -> Self {
        Self {
            translation: uv(t.translation),
            scale: uv(t.scale),
            rotation: t.rotation,
            transform_type: t.transform_type,
            center: uv(t.center),
        }
    }

    fn to_ffi(self) -> ffi::TextureTransform {
        ffi::TextureTransform {
            translation: uv_to_ffi(self.translation),
            scale: uv_to_ffi(self.scale),
            rotation: self.rotation,
            transform_type: self.transform_type,
            center: uv_to_ffi(self.center),
        }
    }
}

/// A map of an [`NiTexturingProperty`], in the order they're saved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum TextureMap {
    /// The main color texture.
    Base = 0,
    /// Darkens the base, e.g. a light map.
    Dark = 1,
    /// Fine detail, usually tiled.
    Detail = 2,
    /// Where the surface is shiny.
    Gloss = 3,
    /// Light the surface emits.
    Glow = 4,
    /// A bump map.
    Bump = 5,
    /// A normal map (Fallout 3 and later).
    Normal = 6,
    /// A parallax map (Fallout 3 and later).
    Parallax = 7,
    Decal0 = 8,
    Decal1 = 9,
    Decal2 = 10,
    Decal3 = 11,
}

/// How an [`NiTexturingProperty`]'s base map combines with the vertex and material colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum ApplyMode {
    /// The texture replaces the color.
    Replace = 0,
    /// The texture is blended over the color by its alpha.
    Decal = 1,
    /// The texture and color are multiplied.
    Modulate = 2,
    /// Highlighting (PlayStation 2 only).
    Hilight = 3,
    /// A second highlighting mode (PlayStation 2 only).
    Hilight2 = 4,
}

/// How a texture repeats outside its UV range, along U (S) and V (T).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum ClampMode {
    /// Clamped along both.
    ClampBoth = 0,
    /// Clamped along U, repeated along V.
    ClampUWrapV = 1,
    /// Repeated along U, clamped along V.
    WrapUClampV = 2,
    /// Repeated along both.
    WrapBoth = 3,
}

/// How a texture is filtered when scaled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum FilterMode {
    /// Nearest texel, no mipmaps.
    Nearest = 0,
    /// Bilinear, no mipmaps.
    Bilinear = 1,
    /// Bilinear between the two nearest mipmaps.
    Trilinear = 2,
    /// Nearest texel of the nearest mipmap.
    NearestMipNearest = 3,
    /// Nearest texel, blended between the two nearest mipmaps.
    NearestMipLinear = 4,
    /// Bilinear within the nearest mipmap.
    BilinearMipNearest = 5,
}

/// # Fields by game
///
/// Not every game stores every field. Each field's doc says which games save it ("Skyrim" means
/// both LE and SE). Elsewhere you can still read and set the field, but the value isn't written
/// when the file is saved.
///
/// Some fields are also only saved for one [shader type](Self::shader_type).
///
/// In Fallout 76 and Starfield, a shader with a name saves none of these fields. Its name is the
/// path of a material file, which holds them instead.
impl BsLightingShaderProperty {
    /// The shader type, which picks the shader program and which type-specific fields are saved,
    /// or `None` if the stored value isn't a [`LightingShaderType`].
    pub fn shader_type(&self) -> Option<LightingShaderType> {
        LightingShaderType::try_from(ffi::lighting_shader_type(&self.0)).ok()
    }

    /// Sets the [shader type](Self::shader_type).
    ///
    /// # Fallout 76 and Starfield
    ///
    /// These games dropped the [`Parallax`](LightingShaderType::Parallax) type and store every
    /// later type one lower (FaceTint as 3, SkinTint as 4, ...). Loading reads this numbering
    /// correctly, but saving doesn't write it. For a shader without a name, a type from
    /// `Parallax` on is saved wrong:
    ///
    /// - The stored type is the next one: `Parallax` reloads as `FaceTint`, `FaceTint` as
    ///   `SkinTint`, `Eye` as `Cloud`.
    /// - The type-specific fields written are those of the type before the one set, while
    ///   readers expect those of the stored type. Where the two have different fields, the
    ///   rest of the block is misread, and some types (such as `HairTint`) give a file that
    ///   can't be loaded again.
    ///
    /// Types before `Parallax` save correctly. A shader with a name (a material file path)
    /// isn't affected: it stores neither its type nor its other fields, which come from the
    /// material file.
    pub fn set_shader_type(self: Pin<&mut Self>, shader_type: LightingShaderType) {
        ffi::set_lighting_shader_type(self.ffi_mut(), shader_type.into());
    }

    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`].
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(ffi::lighting_texture_clamp_mode(&self.0)).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(self: Pin<&mut Self>, mode: ClampMode) {
        ffi::set_lighting_texture_clamp_mode(self.ffi_mut(), mode.into());
    }

    fields! {
        /// Emissive (glow) color as `[r, g, b]`.
        emissive_color, set_emissive_color: [f32; 3] =
            lighting_emissive_color, set_lighting_emissive_color, rgb, rgb_to_ffi;
        /// Multiplier for the emissive color.
        emissive_multiple, set_emissive_multiple: f32 =
            lighting_emissive_multiple, set_lighting_emissive_multiple, same, same;
        /// Opacity, from 0 (transparent) to 1.
        alpha, set_alpha: f32 = lighting_alpha, set_lighting_alpha, same, same;
        refraction_strength, set_refraction_strength: f32 =
            lighting_refraction_strength, set_lighting_refraction_strength, same, same;
        /// Specular glossiness (smoothness from Fallout 4 on).
        glossiness, set_glossiness: f32 = lighting_glossiness, set_lighting_glossiness, same, same;
        /// Specular color as `[r, g, b]`.
        specular_color, set_specular_color: [f32; 3] =
            lighting_specular_color, set_lighting_specular_color, rgb, rgb_to_ffi;
        specular_strength, set_specular_strength: f32 =
            lighting_specular_strength, set_lighting_specular_strength, same, same;

        /// Soft lighting (light wrap). Only saved in Skyrim.
        soft_lighting, set_soft_lighting: f32 =
            lighting_soft_lighting, set_lighting_soft_lighting, same, same;
        /// Rim lighting power. Only saved in Skyrim; Fallout 4 uses
        /// [`rimlight_power_2`](Self::rimlight_power_2).
        rimlight_power, set_rimlight_power: f32 =
            lighting_rimlight_power, set_lighting_rimlight_power, same, same;

        /// Subsurface scattering rolloff. Only saved in Fallout 4.
        subsurface_rolloff, set_subsurface_rolloff: f32 =
            lighting_subsurface_rolloff, set_lighting_subsurface_rolloff, same, same;
        /// Fallout 4's rim lighting power, kept apart from Skyrim's
        /// [`rimlight_power`](Self::rimlight_power). Only saved in Fallout 4. Defaults to
        /// `f32::MAX`, the only value that also saves [`backlight_power`](Self::backlight_power).
        rimlight_power_2, set_rimlight_power_2: f32 =
            lighting_rimlight_power_2, set_lighting_rimlight_power_2, same, same;
        /// Back lighting power. Only saved in Fallout 4, while
        /// [`rimlight_power_2`](Self::rimlight_power_2) is `f32::MAX`.
        backlight_power, set_backlight_power: f32 =
            lighting_backlight_power, set_lighting_backlight_power, same, same;

        /// Only saved from Fallout 4 on.
        grayscale_to_palette_scale, set_grayscale_to_palette_scale: f32 =
            lighting_grayscale_to_palette_scale, set_lighting_grayscale_to_palette_scale, same, same;
        /// Only saved from Fallout 4 on.
        fresnel_power, set_fresnel_power: f32 =
            lighting_fresnel_power, set_lighting_fresnel_power, same, same;
        /// Specular scale when wet. Only saved from Fallout 4 on.
        wetness_specular_scale, set_wetness_specular_scale: f32 =
            lighting_wetness_specular_scale, set_lighting_wetness_specular_scale, same, same;
        /// Specular power when wet. Only saved from Fallout 4 on.
        wetness_specular_power, set_wetness_specular_power: f32 =
            lighting_wetness_specular_power, set_lighting_wetness_specular_power, same, same;
        /// Minimum wetness. Only saved from Fallout 4 on.
        wetness_min_var, set_wetness_min_var: f32 =
            lighting_wetness_min_var, set_lighting_wetness_min_var, same, same;
        /// Environment map scale when wet. Only saved in Fallout 4 files with stream version 130.
        wetness_envmap_scale, set_wetness_envmap_scale: f32 =
            lighting_wetness_envmap_scale, set_lighting_wetness_envmap_scale, same, same;
        /// Fresnel power when wet. Only saved from Fallout 4 on.
        wetness_fresnel_power, set_wetness_fresnel_power: f32 =
            lighting_wetness_fresnel_power, set_lighting_wetness_fresnel_power, same, same;
        /// Metalness when wet. Only saved from Fallout 4 on.
        wetness_metalness, set_wetness_metalness: f32 =
            lighting_wetness_metalness, set_lighting_wetness_metalness, same, same;

        /// Luminance of the emittance. Only saved in Fallout 76 and Starfield.
        lum_emittance, set_lum_emittance: f32 =
            lighting_lum_emittance, set_lighting_lum_emittance, same, same;
        /// Only saved in Fallout 76 and Starfield.
        exposure_offset, set_exposure_offset: f32 =
            lighting_exposure_offset, set_lighting_exposure_offset, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_min, set_final_exposure_min: f32 =
            lighting_final_exposure_min, set_lighting_final_exposure_min, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_max, set_final_exposure_max: f32 =
            lighting_final_exposure_max, set_lighting_final_exposure_max, same, same;

        /// Uses screen-space reflections. Only saved in Fallout 4, for type
        /// [`EnvironmentMap`](LightingShaderType::EnvironmentMap).
        use_ssr, set_use_ssr: bool = lighting_use_ssr, set_lighting_use_ssr, same, same;
        /// Uses screen-space reflections when wet. Only saved in Fallout 4, for type
        /// [`EnvironmentMap`](LightingShaderType::EnvironmentMap).
        wetness_use_ssr, set_wetness_use_ssr: bool =
            lighting_wetness_use_ssr, set_lighting_wetness_use_ssr, same, same;
        /// Skin tint as `[r, g, b]`. Only saved for type
        /// [`SkinTint`](LightingShaderType::SkinTint).
        skin_tint_color, set_skin_tint_color: [f32; 3] =
            lighting_skin_tint_color, set_lighting_skin_tint_color, rgb, rgb_to_ffi;
        /// Skin tint opacity. Only saved from Fallout 4 on, for type
        /// [`SkinTint`](LightingShaderType::SkinTint).
        skin_tint_alpha, set_skin_tint_alpha: f32 =
            lighting_skin_tint_alpha, set_lighting_skin_tint_alpha, same, same;
        /// Hair tint as `[r, g, b]`. Only saved for type
        /// [`HairTint`](LightingShaderType::HairTint).
        hair_tint_color, set_hair_tint_color: [f32; 3] =
            lighting_hair_tint_color, set_lighting_hair_tint_color, rgb, rgb_to_ffi;
        /// Parallax occlusion passes. Only saved for type
        /// [`ParallaxOcclusion`](LightingShaderType::ParallaxOcclusion).
        parallax_max_passes, set_parallax_max_passes: f32 =
            lighting_max_passes, set_lighting_max_passes, same, same;
        /// Parallax occlusion height scale. Only saved for type
        /// [`ParallaxOcclusion`](LightingShaderType::ParallaxOcclusion).
        parallax_scale, set_parallax_scale: f32 = lighting_scale, set_lighting_scale, same, same;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_inner_layer_thickness, set_parallax_inner_layer_thickness: f32 =
            lighting_parallax_inner_layer_thickness, set_lighting_parallax_inner_layer_thickness, same, same;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_refraction_scale, set_parallax_refraction_scale: f32 =
            lighting_parallax_refraction_scale, set_lighting_parallax_refraction_scale, same, same;
        /// Inner layer texture scale as `[u, v]`. Only saved for type
        /// [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_inner_layer_texture_scale, set_parallax_inner_layer_texture_scale: [f32; 2] =
            lighting_parallax_inner_layer_texture_scale, set_lighting_parallax_inner_layer_texture_scale, uv, uv_to_ffi;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_envmap_strength, set_parallax_envmap_strength: f32 =
            lighting_parallax_envmap_strength, set_lighting_parallax_envmap_strength, same, same;
        /// Snow sparkle parameters as four values. Only saved for type
        /// [`MultiIndexSnow`](LightingShaderType::MultiIndexSnow).
        sparkle_parameters, set_sparkle_parameters: [f32; 4] =
            lighting_sparkle_parameters, set_lighting_sparkle_parameters, rgba, rgba_to_ffi;
        /// Only saved for type [`Eye`](LightingShaderType::Eye).
        eye_cubemap_scale, set_eye_cubemap_scale: f32 =
            lighting_eye_cubemap_scale, set_lighting_eye_cubemap_scale, same, same;
        /// Center of the left eye's reflection. Only saved for type
        /// [`Eye`](LightingShaderType::Eye).
        eye_left_reflection_center, set_eye_left_reflection_center: Vec3 =
            lighting_eye_left_reflection_center, set_lighting_eye_left_reflection_center, Vec3::from_ffi, Vec3::to_ffi;
        /// Center of the right eye's reflection. Only saved for type
        /// [`Eye`](LightingShaderType::Eye).
        eye_right_reflection_center, set_eye_right_reflection_center: Vec3 =
            lighting_eye_right_reflection_center, set_lighting_eye_right_reflection_center, Vec3::from_ffi, Vec3::to_ffi;

        /// A value of unknown meaning stored with the wetness fields. Only saved in Fallout 4
        /// files with a stream version above 130, Fallout 76 and Starfield.
        wetness_unknown_1, set_wetness_unknown_1: f32 =
            lighting_wetness_unknown_1, set_lighting_wetness_unknown_1, same, same;
        /// A value of unknown meaning stored with the wetness fields. Only saved in Fallout 76
        /// and Starfield.
        wetness_unknown_2, set_wetness_unknown_2: f32 =
            lighting_wetness_unknown_2, set_lighting_wetness_unknown_2, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float, set_unknown_float: f32 =
            lighting_unknown_float, set_lighting_unknown_float, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float_1, set_unknown_float_1: f32 =
            lighting_unknown_float_1, set_lighting_unknown_float_1, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float_2, set_unknown_float_2: f32 =
            lighting_unknown_float_2, set_lighting_unknown_float_2, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_short, set_unknown_short: u16 =
            lighting_unknown_short, set_lighting_unknown_short, same, same;
    }

    /// The wet material's path. Only saved from Fallout 4 on.
    pub fn wet_material(&self) -> &BStr {
        ffi::lighting_root_material(&self.0).as_bstr()
    }

    /// Sets the [wet material](Self::wet_material).
    pub fn set_wet_material(self: Pin<&mut Self>, path: impl AsRef<[u8]>) {
        ffi::set_lighting_root_material(self.ffi_mut(), path.as_ref());
    }

    /// The translucency settings, or `None` if translucency is off. Only saved in Fallout 76.
    pub fn translucency(&self) -> Option<Translucency> {
        ffi::lighting_translucency(&self.0).then(|| Translucency {
            subsurface_color: rgb(ffi::lighting_subsurface_color(&self.0)),
            transmissive_scale: ffi::lighting_transmissive_scale(&self.0),
            turbulence: ffi::lighting_turbulence(&self.0),
            thick_object: ffi::lighting_thick_object(&self.0),
            mix_albedo: ffi::lighting_mix_albedo(&self.0),
        })
    }

    /// Turns translucency on with these settings, or off with `None`.
    pub fn set_translucency(mut self: Pin<&mut Self>, translucency: Option<Translucency>) {
        ffi::set_lighting_translucency(self.as_mut().ffi_mut(), translucency.is_some());
        if let Some(t) = translucency {
            ffi::set_lighting_subsurface_color(self.as_mut().ffi_mut(), rgb_to_ffi(t.subsurface_color));
            ffi::set_lighting_transmissive_scale(self.as_mut().ffi_mut(), t.transmissive_scale);
            ffi::set_lighting_turbulence(self.as_mut().ffi_mut(), t.turbulence);
            ffi::set_lighting_thick_object(self.as_mut().ffi_mut(), t.thick_object);
            ffi::set_lighting_mix_albedo(self.ffi_mut(), t.mix_albedo);
        }
    }

    /// The texture arrays, each a list of texture paths. Only saved in Fallout 76.
    pub fn texture_arrays(&self) -> Vec<Vec<&BStr>> {
        (0..ffi::lighting_texture_array_count(&self.0))
            .map(|a| {
                (0..ffi::lighting_texture_array_len(&self.0, a))
                    .map(|i| ffi::lighting_texture_array_path(&self.0, a, i).as_bstr())
                    .collect()
            })
            .collect()
    }

    /// Replaces the [texture arrays](Self::texture_arrays).
    pub fn set_texture_arrays<A, P>(mut self: Pin<&mut Self>, arrays: impl IntoIterator<Item = A>)
    where
        A: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        ffi::clear_lighting_texture_arrays(self.as_mut().ffi_mut());
        for array in arrays {
            ffi::add_lighting_texture_array(self.as_mut().ffi_mut());
            for path in array {
                ffi::push_lighting_texture_array_path(self.as_mut().ffi_mut(), path.as_ref());
            }
        }
    }
}

/// Getter and setter pairs for texture paths, over the bridge's `$ffi_get` and `$ffi_set`.
macro_rules! texture_paths {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> &BStr {
            ffi::$ffi_get(&self.0).as_bstr()
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, path: impl AsRef<[u8]>) {
            ffi::$ffi_set(self.ffi_mut(), path.as_ref());
        }
    )*};
}

/// # Fields by game
///
/// Not every game stores every field. Each field's doc says which games save it. Elsewhere you
/// can still read and set the field, but the value isn't written when the file is saved.
///
/// In Fallout 76 and Starfield, and in Fallout 4 files with a stream version above 130, a shader
/// with a name saves none of these fields. Its name is the path of a material file, which holds
/// them instead.
impl BsEffectShaderProperty {
    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`].
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(u32::from(self.clamp_byte(0))).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(self: Pin<&mut Self>, mode: ClampMode) {
        self.set_clamp_byte(0, u32::from(mode) as u8);
    }

    /// How much scene lighting affects the shader, from 0 (none) to 255.
    pub fn lighting_influence(&self) -> u8 {
        self.clamp_byte(1)
    }

    /// Sets the [lighting influence](Self::lighting_influence).
    pub fn set_lighting_influence(self: Pin<&mut Self>, influence: u8) {
        self.set_clamp_byte(1, influence);
    }

    /// The smallest mipmap level used for the environment map.
    pub fn env_map_min_lod(&self) -> u8 {
        self.clamp_byte(2)
    }

    /// Sets the [environment map's minimum LOD](Self::env_map_min_lod).
    pub fn set_env_map_min_lod(self: Pin<&mut Self>, lod: u8) {
        self.set_clamp_byte(2, lod);
    }

    /// The last byte of the field holding the [clamp mode](Self::texture_clamp_mode), of unknown
    /// meaning.
    pub fn clamp_unknown(&self) -> u8 {
        self.clamp_byte(3)
    }

    /// Sets [`clamp_unknown`](Self::clamp_unknown).
    pub fn set_clamp_unknown(self: Pin<&mut Self>, value: u8) {
        self.set_clamp_byte(3, value);
    }

    /// Byte `i` of the field holding the clamp mode, lighting influence, environment map
    /// minimum LOD and an unknown byte, lowest first.
    fn clamp_byte(&self, i: usize) -> u8 {
        ffi::effect_texture_clamp_bytes(&self.0).to_le_bytes()[i]
    }

    fn set_clamp_byte(mut self: Pin<&mut Self>, i: usize, value: u8) {
        let mut bytes = ffi::effect_texture_clamp_bytes(&self.0).to_le_bytes();
        bytes[i] = value;
        ffi::set_effect_texture_clamp_bytes(self.as_mut().ffi_mut(), u32::from_le_bytes(bytes));
    }

    texture_paths! {
        /// The base texture.
        source_texture, set_source_texture = effect_source_texture, set_effect_source_texture;
        /// The palette the base texture's greyscale is mapped through.
        greyscale_texture, set_greyscale_texture = effect_greyscale_texture, set_effect_greyscale_texture;
        /// Only saved from Fallout 4 on.
        env_map_texture, set_env_map_texture = effect_env_map_texture, set_effect_env_map_texture;
        /// Only saved from Fallout 4 on.
        normal_texture, set_normal_texture = effect_normal_texture, set_effect_normal_texture;
        /// Only saved from Fallout 4 on.
        env_mask_texture, set_env_mask_texture = effect_env_mask_texture, set_effect_env_mask_texture;
        /// Only saved in Fallout 76 and Starfield.
        reflectance_texture, set_reflectance_texture = effect_reflectance_texture, set_effect_reflectance_texture;
        /// Only saved in Fallout 76 and Starfield.
        lighting_texture, set_lighting_texture = effect_lighting_texture, set_effect_lighting_texture;
        /// Only saved in Fallout 76 and Starfield.
        emit_gradient_texture, set_emit_gradient_texture =
            effect_emit_gradient_texture, set_effect_emit_gradient_texture;
    }

    fields! {
        /// Angle in degrees, between the view and the surface normal, where falloff starts.
        falloff_start_angle, set_falloff_start_angle: f32 =
            effect_falloff_start_angle, set_effect_falloff_start_angle, same, same;
        /// Angle in degrees where falloff stops.
        falloff_stop_angle, set_falloff_stop_angle: f32 =
            effect_falloff_stop_angle, set_effect_falloff_stop_angle, same, same;
        /// Opacity where falloff starts.
        falloff_start_opacity, set_falloff_start_opacity: f32 =
            effect_falloff_start_opacity, set_effect_falloff_start_opacity, same, same;
        /// Opacity where falloff stops.
        falloff_stop_opacity, set_falloff_stop_opacity: f32 =
            effect_falloff_stop_opacity, set_effect_falloff_stop_opacity, same, same;
        /// Only saved in Fallout 76.
        refraction_power, set_refraction_power: f32 =
            effect_refraction_power, set_effect_refraction_power, same, same;
        /// Base (emissive) color as `[r, g, b, a]`.
        base_color, set_base_color: [f32; 4] = effect_base_color, set_effect_base_color, rgba, rgba_to_ffi;
        /// Multiplier for the base color.
        base_color_scale, set_base_color_scale: f32 =
            effect_base_color_scale, set_effect_base_color_scale, same, same;
        /// Depth over which soft particles fade where they meet geometry.
        soft_falloff_depth, set_soft_falloff_depth: f32 =
            effect_soft_falloff_depth, set_effect_soft_falloff_depth, same, same;
        /// Strength of the environment map. Only saved from Fallout 4 on. This shader doesn't
        /// save [`BsShaderProperty::environment_map_scale`].
        env_map_scale, set_env_map_scale: f32 = effect_env_map_scale, set_effect_env_map_scale, same, same;
        /// Emittance color as `[r, g, b]`. Only saved in Fallout 76 and Starfield.
        emittance_color, set_emittance_color: [f32; 3] =
            effect_emittance_color, set_effect_emittance_color, rgb, rgb_to_ffi;
        /// Luminance of the emittance. Only saved in Fallout 76 and Starfield.
        lum_emittance, set_lum_emittance: f32 =
            effect_lum_emittance, set_effect_lum_emittance, same, same;
        /// Only saved in Fallout 76 and Starfield.
        exposure_offset, set_exposure_offset: f32 =
            effect_exposure_offset, set_effect_exposure_offset, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_min, set_final_exposure_min: f32 =
            effect_final_exposure_min, set_effect_final_exposure_min, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_max, set_final_exposure_max: f32 =
            effect_final_exposure_max, set_effect_final_exposure_max, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float, set_unknown_float: f32 = effect_unknown_float, set_effect_unknown_float, same, same;
        /// Values of unknown meaning. Only saved in Starfield.
        unknown_bytes, set_unknown_bytes: [u8; 7] = effect_unknown_bytes, set_effect_unknown_bytes, same, same;
        /// Values of unknown meaning. Only saved in Starfield.
        unknown_floats, set_unknown_floats: [f32; 6] =
            effect_unknown_floats, set_effect_unknown_floats, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_byte, set_unknown_byte: u8 = effect_unknown_byte, set_effect_unknown_byte, same, same;
    }
}

/// # Fields by version
///
/// Morrowind files are version 4.0.0.2, Oblivion's 20.0.0.4 or 20.0.0.5, and later games'
/// 20.2.0.7. Fields a file's version doesn't store can still be read and set, but aren't
/// written when it's saved.
impl NiSourceTexture {
    texture_paths! {
        /// The texture's path. Saved while the texture is
        /// [external](Self::use_external), and always from version 10.1.0.0 (Oblivion) on.
        file_name, set_file_name = source_file_name, set_source_file_name;
    }

    fields! {
        /// Whether the texture is in an external file, at [`file_name`](Self::file_name), rather
        /// than in the pixel data.
        use_external, set_use_external: bool = source_use_external, set_source_use_external, same, same;
        /// Whether a texture that isn't external has its pixel data in this file. Only saved up
        /// to version 10.0.1.3 (Morrowind), while the texture isn't external; later files always
        /// have it there and load it as `false`.
        use_internal, set_use_internal: bool = source_use_internal, set_source_use_internal, same, same;
        is_static, set_static: bool = source_is_static, set_source_is_static, same, same;
        /// Only saved from version 10.1.0.103 (Oblivion) on.
        direct_render, set_direct_render: bool = source_direct_render, set_source_direct_render, same, same;
        /// Only saved from version 20.2.0.4 (Fallout 3) on.
        persistent_render_data, set_persistent_render_data: bool =
            source_persistent_render_data, set_source_persistent_render_data, same, same;
    }

    /// The block index of the texture's pixel data, or `None` if there's none.
    /// [`View::pixel_data`] gives the pixel data itself.
    pub fn pixel_data_index(&self) -> Option<Index> {
        index_from_ffi(ffi::source_pixel_data(&self.0))
    }

    /// How the pixels are stored, or `None` if the stored value isn't a [`PixelLayout`].
    pub fn pixel_layout(&self) -> Option<PixelLayout> {
        PixelLayout::try_from(ffi::source_pixel_layout(&self.0)).ok()
    }

    /// Sets the [pixel layout](Self::pixel_layout).
    pub fn set_pixel_layout(self: Pin<&mut Self>, layout: PixelLayout) {
        ffi::set_source_pixel_layout(self.ffi_mut(), layout.into());
    }

    /// Whether the texture uses mipmaps, or `None` if the stored value isn't a
    /// [`MipMapFormat`].
    pub fn mip_map_format(&self) -> Option<MipMapFormat> {
        MipMapFormat::try_from(ffi::source_mip_map_format(&self.0)).ok()
    }

    /// Sets the [mipmap format](Self::mip_map_format).
    pub fn set_mip_map_format(self: Pin<&mut Self>, format: MipMapFormat) {
        ffi::set_source_mip_map_format(self.ffi_mut(), format.into());
    }

    /// How the texture's alpha is used, or `None` if the stored value isn't an
    /// [`AlphaFormat`].
    pub fn alpha_format(&self) -> Option<AlphaFormat> {
        AlphaFormat::try_from(ffi::source_alpha_format(&self.0)).ok()
    }

    /// Sets the [alpha format](Self::alpha_format).
    pub fn set_alpha_format(self: Pin<&mut Self>, format: AlphaFormat) {
        ffi::set_source_alpha_format(self.ffi_mut(), format.into());
    }
}

/// Pixel data fields. Files from version 10.4.0.2 (Oblivion) on save the bit depth, renderer
/// hint, extra data, flags and channels; earlier ones save the color masks instead.
impl TextureRenderData {
    /// The pixel format, or `None` if the stored value isn't a [`PixelFormat`].
    pub fn pixel_format(&self) -> Option<PixelFormat> {
        PixelFormat::try_from(ffi::render_data_pixel_format(&self.0)).ok()
    }

    /// Sets the [pixel format](Self::pixel_format).
    pub fn set_pixel_format(self: Pin<&mut Self>, format: PixelFormat) {
        ffi::set_render_data_pixel_format(self.ffi_mut(), format.into());
    }

    fields! {
        /// Bits per pixel: 0 if compressed, or 8, 24 or 32.
        bits_per_pixel, set_bits_per_pixel: u8 =
            render_data_bits_per_pixel, set_render_data_bits_per_pixel, same, same;
        bytes_per_pixel, set_bytes_per_pixel: u32 =
            render_data_bytes_per_pixel, set_render_data_bytes_per_pixel, same, same;
        renderer_hint, set_renderer_hint: u32 = render_data_renderer_hint, set_render_data_renderer_hint, same, same;
        /// The format's extra data value (not an extra data block).
        format_extra_data, set_format_extra_data: u32 =
            render_data_extra_data, set_render_data_extra_data, same, same;
        flags, set_flags: u8 = render_data_flags, set_render_data_flags, same, same;
        /// Bytes of unknown meaning used to compare formats. Only saved up to version 10.4.0.1.
        old_fast_compare, set_old_fast_compare: [u8; 8] =
            render_data_old_fast_compare, set_render_data_old_fast_compare, same, same;
    }

    /// How the pixels are tiled, or `None` if the stored value isn't a [`PixelTiling`]. Saved
    /// from version 10.1.0.0 on.
    pub fn tiling(&self) -> Option<PixelTiling> {
        PixelTiling::try_from(ffi::render_data_tiling(&self.0)).ok()
    }

    /// Sets the [tiling](Self::tiling).
    pub fn set_tiling(self: Pin<&mut Self>, tiling: PixelTiling) {
        ffi::set_render_data_tiling(self.ffi_mut(), tiling.into());
    }

    /// The red, green, blue and alpha bit masks. Only saved up to version 10.4.0.1
    /// (Morrowind).
    pub fn color_masks(&self) -> [u32; 4] {
        let mut out = [0; 4];
        ffi::render_data_masks(&self.0, &mut out);
        out
    }

    /// Sets the [color masks](Self::color_masks).
    pub fn set_color_masks(self: Pin<&mut Self>, masks: [u32; 4]) {
        ffi::set_render_data_masks(self.ffi_mut(), &masks);
    }

    /// The four channels' formats.
    pub fn channels(&self) -> [PixelChannel; 4] {
        std::array::from_fn(|i| {
            let mut c = [0; 4];
            ffi::render_data_channel(&self.0, i, &mut c);
            PixelChannel { component: c[0], representation: c[1], bits: c[2] as u8, signed: c[3] != 0 }
        })
    }

    /// Sets the [channels](Self::channels).
    pub fn set_channels(mut self: Pin<&mut Self>, channels: [PixelChannel; 4]) {
        for (i, c) in channels.iter().enumerate() {
            let v = [c.component, c.representation, c.bits.into(), c.signed.into()];
            ffi::set_render_data_channel(self.as_mut().ffi_mut(), i, &v);
        }
    }

    /// The mipmaps, largest first. Each one's pixels start at its
    /// [offset](MipMap::offset) in every face.
    pub fn mipmaps(&self) -> Vec<MipMap> {
        let flat = ffi::render_data_mipmaps(&self.0);
        flat.as_chunks().0.iter().map(|&[width, height, offset]| MipMap { width, height, offset }).collect()
    }

    /// Sets the [mipmaps](Self::mipmaps). Not checked against the pixel bytes.
    pub fn set_mipmaps(self: Pin<&mut Self>, mipmaps: &[MipMap]) {
        let flat: Vec<u32> = mipmaps.iter().flat_map(|m| [m.width, m.height, m.offset]).collect();
        ffi::set_render_data_mipmaps(self.ffi_mut(), &flat);
    }
}

/// A [`TextureRenderData`] channel. `component` and `representation` are raw values; see
/// [`PixelComponent`] and [`PixelRepresentation`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct PixelChannel {
    pub component: u32,
    pub representation: u32,
    pub bits: u8,
    pub signed: bool,
}

impl PixelChannel {
    /// The [component](Self::component), or `None` if it isn't a [`PixelComponent`].
    pub fn component(&self) -> Option<PixelComponent> {
        PixelComponent::try_from(self.component).ok()
    }

    /// The [representation](Self::representation), or `None` if it isn't a
    /// [`PixelRepresentation`].
    pub fn representation(&self) -> Option<PixelRepresentation> {
        PixelRepresentation::try_from(self.representation).ok()
    }
}

/// A mipmap of a [`TextureRenderData`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MipMap {
    pub width: u32,
    pub height: u32,
    /// Where its pixels start, in bytes.
    pub offset: u32,
}

/// Faces getter and setter for pixel data types.
macro_rules! pixel_faces {
    ($count:ident, $face:ident, $clear:ident, $push:ident) => {
        /// The pixel bytes of each face: one, or six for a cube map.
        pub fn faces(&self) -> Vec<&[u8]> {
            (0..ffi::$count(&self.0)).map(|f| ffi::$face(&self.0, f)).collect()
        }

        /// Replaces the [faces](Self::faces). Returns `false`, changing nothing, unless they all
        /// have the same length, below 2³².
        pub fn set_faces(mut self: Pin<&mut Self>, faces: &[&[u8]]) -> bool {
            let len = faces.first().map_or(0, |f| f.len());
            if u32::try_from(len).is_err() || faces.iter().any(|f| f.len() != len) {
                return false;
            }
            ffi::$clear(self.as_mut().ffi_mut());
            for face in faces {
                ffi::$push(self.as_mut().ffi_mut(), face);
            }
            true
        }
    };
}

/// Files before version 10.4.0.2 (Morrowind) store one face: saving one drops the others.
impl NiPixelData {
    pixel_faces!(pixel_data_face_count, pixel_data_face, clear_pixel_data_faces, push_pixel_data_face);
}

impl NiPersistentSrcTextureRendererData {
    pixel_faces!(
        persistent_data_face_count,
        persistent_data_face,
        clear_persistent_data_faces,
        push_persistent_data_face
    );

    /// The target platform, or `None` if the stored value isn't a [`Platform`].
    pub fn platform(&self) -> Option<Platform> {
        Platform::try_from(ffi::persistent_data_platform(&self.0)).ok()
    }

    /// Sets the [platform](Self::platform).
    pub fn set_platform(self: Pin<&mut Self>, platform: Platform) {
        ffi::set_persistent_data_platform(self.ffi_mut(), platform.into());
    }

    fields! {
        /// The pixel count with padding.
        padded_pixel_count, set_padded_pixel_count: u32 =
            persistent_data_padded_pixel_count, set_persistent_data_padded_pixel_count, same, same;
    }
}

impl NiPalette {
    /// Whether the colors' alpha is used.
    pub fn has_alpha(&self) -> bool {
        ffi::palette_has_alpha(&self.0)
    }

    pub fn set_has_alpha(self: Pin<&mut Self>, value: bool) {
        ffi::set_palette_has_alpha(self.ffi_mut(), value);
    }

    /// The colors as `[r, g, b, a]`.
    pub fn colors(&self) -> Vec<[u8; 4]> {
        ffi::palette_colors(&self.0).as_chunks().0.to_vec()
    }

    /// Sets the [colors](Self::colors). Returns `false`, changing nothing, unless there are 16 or
    /// 256.
    pub fn set_colors(self: Pin<&mut Self>, colors: &[[u8; 4]]) -> bool {
        ffi::set_palette_colors(self.ffi_mut(), colors.as_flattened())
    }
}

/// The format of a [`TextureRenderData`]'s pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PixelFormat {
    /// 8 bits each of red, green and blue.
    Rgb = 0,
    /// 8 bits each of red, green, blue and alpha.
    Rgba = 1,
    /// An 8-bit [palette](View::palette) index.
    Palette = 2,
    /// An 8-bit palette index with alpha.
    PaletteAlpha = 3,
    Dxt1 = 4,
    Dxt3 = 5,
    Dxt5 = 6,
    /// A deprecated PlayStation 2 format.
    Rgb24NonInterleaved = 7,
    /// A dU/dV bump map.
    Bump = 8,
    /// A dU/dV bump map with luma.
    BumpLuma = 9,
    /// A renderer-specific format.
    RenderSpecific = 10,
    OneChannel = 11,
    TwoChannels = 12,
    ThreeChannels = 13,
    FourChannels = 14,
    DepthStencil = 15,
    Unknown = 16,
}

/// How a [`TextureRenderData`]'s pixels are tiled for a platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PixelTiling {
    None = 0,
    Xenon = 1,
    Wii = 2,
    NvSwizzled = 3,
}

/// What a [`PixelChannel`] holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PixelComponent {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3,
    Compressed = 4,
    OffsetU = 5,
    OffsetV = 6,
    OffsetW = 7,
    OffsetQ = 8,
    Luma = 9,
    Height = 10,
    VectorX = 11,
    VectorY = 12,
    VectorZ = 13,
    Padding = 14,
    Intensity = 15,
    Index = 16,
    Depth = 17,
    Stencil = 18,
    Empty = 19,
}

/// How a [`PixelChannel`]'s values are stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PixelRepresentation {
    NormalizedInt = 0,
    Half = 1,
    Float = 2,
    Index = 3,
    Compressed = 4,
    Unknown = 5,
    Int = 6,
}

/// The target platform of an [`NiPersistentSrcTextureRendererData`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum Platform {
    Any = 0,
    Xenon = 1,
    Ps3 = 2,
    Dx9 = 3,
    Wii = 4,
    D3d10 = 5,
}

/// A pixel data type that [`Edit::assign_pixel_data`] can create.
///
/// This trait is sealed.
pub trait NewPixelData: NiType {
    /// Gives `texture` a new default `Self` and returns its index (`u32::MAX` on failure).
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, texture: u32) -> u32;
}

impl NewPixelData for NiPixelData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, texture: u32) -> u32 {
        nif_ffi::assign_ni_pixel_data(file, texture)
    }
}

impl NewPixelData for NiPersistentSrcTextureRendererData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, texture: u32) -> u32 {
        nif_ffi::assign_persistent_pixel_data(file, texture)
    }
}

/// How an [`NiSourceTexture`]'s pixels are stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PixelLayout {
    Palettized8 = 0,
    HighColor16 = 1,
    TrueColor32 = 2,
    Compressed = 3,
    BumpMap = 4,
    Palettized4 = 5,
    Default = 6,
    SingleColor8 = 7,
    SingleColor16 = 8,
    SingleColor32 = 9,
    DoubleColor32 = 10,
    DoubleColor64 = 11,
    FloatColor32 = 12,
    FloatColor64 = 13,
    FloatColor128 = 14,
    SingleColor4 = 15,
    Depth24X8 = 16,
}

/// Whether an [`NiSourceTexture`] uses mipmaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum MipMapFormat {
    No = 0,
    Yes = 1,
    Default = 2,
}

/// How an [`NiSourceTexture`]'s alpha is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum AlphaFormat {
    /// Opaque.
    None = 0,
    /// Fully opaque or fully transparent.
    Binary = 1,
    /// Any opacity.
    Smooth = 2,
    Default = 3,
}

/// Translucency (subsurface scattering) settings of a [`BsLightingShaderProperty`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Translucency {
    /// Subsurface color as `[r, g, b]`.
    pub subsurface_color: [f32; 3],
    pub transmissive_scale: f32,
    pub turbulence: f32,
    pub thick_object: bool,
    /// Mixes the subsurface color with the diffuse texture.
    pub mix_albedo: bool,
}

impl Default for Translucency {
    fn default() -> Self {
        Self { subsurface_color: [0.0; 3], transmissive_scale: 1.0, turbulence: 0.0, thick_object: false, mix_albedo: false }
    }
}

/// The shader program of a [`BsLightingShaderProperty`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LightingShaderType {
    Default = 0,
    EnvironmentMap = 1,
    GlowMap = 2,
    /// Not in Fallout 76 and Starfield, which store the later types one lower. See
    /// [`set_shader_type`](BsLightingShaderProperty::set_shader_type).
    Parallax = 3,
    FaceTint = 4,
    SkinTint = 5,
    HairTint = 6,
    ParallaxOcclusion = 7,
    MultiTextureLandscape = 8,
    LodLandscape = 9,
    Snow = 10,
    MultiLayerParallax = 11,
    TreeAnim = 12,
    LodObjects = 13,
    MultiIndexSnow = 14,
    LodObjectsHd = 15,
    Eye = 16,
    Cloud = 17,
    LodLandscapeNoise = 18,
    MultiTextureLandscapeLodBlend = 19,
    Dismemberment = 20,
}

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
            #[doc = concat!("Whether ", $what, " is on: bit 0 of [`flags`](Self::flags).")]
            pub fn enabled(&self) -> bool {
                self.flags() & 1 != 0
            }

            /// The raw flags. Only bit 0 is used.
            pub fn flags(&self) -> u16 {
                ffi::$get(&self.0)
            }

            pub fn set_enabled(self: Pin<&mut Self>, enabled: bool) {
                let flags = (self.flags() & !1) | u16::from(enabled);
                self.set_flags(flags);
            }

            pub fn set_flags(self: Pin<&mut Self>, flags: u16) {
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
        flags, set_flags: u16 = fog_flags, set_fog_flags, same, same;
        /// How far the fog reaches.
        depth, set_depth: f32 = fog_depth, set_fog_depth, same, same;
        /// The fog color, as `[r, g, b]`.
        color, set_color: [f32; 3] = fog_color, set_fog_color, rgb, rgb_to_ffi;
    }

    /// Whether fog is on.
    pub fn enabled(&self) -> bool {
        self.flags() & 1 != 0
    }

    /// How fog thickens with distance. `None` if the bits hold 3.
    pub fn function(&self) -> Option<FogFunction> {
        FogFunction::try_from(get_bits(self.flags(), Self::FUNCTION)).ok()
    }

    pub fn set_enabled(self: Pin<&mut Self>, enabled: bool) {
        let flags = (self.flags() & !1) | u16::from(enabled);
        self.set_flags(flags);
    }

    pub fn set_function(self: Pin<&mut Self>, function: FogFunction) {
        let flags = set_bits(self.flags(), Self::FUNCTION, function.into());
        self.set_flags(flags);
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

    pub fn set_flags(self: Pin<&mut Self>, flags: VertexColorFlags) {
        ffi::set_vertex_color_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_vertex_mode(self: Pin<&mut Self>, mode: VertexColorMode) {
        ffi::set_vertex_color_vertex_mode(self.ffi_mut(), mode.into());
    }

    pub fn set_lighting_mode(self: Pin<&mut Self>, mode: LightingMode) {
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

    pub fn set_flags(self: Pin<&mut Self>, flags: ZBufferFlags) {
        ffi::set_z_buffer_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_function(self: Pin<&mut Self>, function: DepthTest) {
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

/// The shader program of a Fallout 3 or New Vegas shader, from
/// [`BsShaderProperty::legacy_shader_type`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LegacyShaderType {
    TallGrass = 0,
    Default = 1,
    Sky = 10,
    Skin = 14,
    Water = 17,
    Lighting30 = 29,
    Tile = 32,
    NoLighting = 33,
}

impl BsShaderLightingProperty {
    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`]. Only saved
    /// in Fallout 3 and New Vegas files.
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(ffi::shader_lighting_clamp_mode(&self.0)).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(self: Pin<&mut Self>, mode: ClampMode) {
        ffi::set_shader_lighting_clamp_mode(self.ffi_mut(), mode.into());
    }
}

/// Its texture set is [`View::texture_set`].
impl BsShaderPpLightingProperty {
    fields! {
        /// Refraction strength. Saved in Fallout 3 and New Vegas files.
        refraction_strength, set_refraction_strength: f32 =
            pp_lighting_refraction_strength, set_pp_lighting_refraction_strength, same, same;
        /// Refraction fire period. Saved in Fallout 3 and New Vegas files.
        refraction_fire_period, set_refraction_fire_period: i32 =
            pp_lighting_refraction_fire_period, set_pp_lighting_refraction_fire_period, same, same;
        /// Maximum number of parallax passes. Saved in Fallout 3 and New Vegas files.
        parallax_max_passes, set_parallax_max_passes: f32 =
            pp_lighting_parallax_max_passes, set_pp_lighting_parallax_max_passes, same, same;
        /// Parallax scale. Saved in Fallout 3 and New Vegas files.
        parallax_scale, set_parallax_scale: f32 =
            pp_lighting_parallax_scale, set_pp_lighting_parallax_scale, same, same;
        /// Emissive color as `[r, g, b, a]`. Only saved in Skyrim and later files, not in Fallout
        /// 3 and New Vegas ones.
        emissive_color, set_emissive_color: [f32; 4] =
            pp_lighting_emissive_color, set_pp_lighting_emissive_color, rgba, rgba_to_ffi;
    }
}

impl BsShaderNoLightingProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        base_texture, set_base_texture = no_lighting_base_texture, set_no_lighting_base_texture;
    }

    fields! {
        /// The view angle's cosine where falloff starts.
        falloff_start_angle, set_falloff_start_angle: f32 =
            no_lighting_falloff_start_angle, set_no_lighting_falloff_start_angle, same, same;
        /// The view angle's cosine where falloff stops.
        falloff_stop_angle, set_falloff_stop_angle: f32 =
            no_lighting_falloff_stop_angle, set_no_lighting_falloff_stop_angle, same, same;
        /// Opacity where falloff starts.
        falloff_start_opacity, set_falloff_start_opacity: f32 =
            no_lighting_falloff_start_opacity, set_no_lighting_falloff_start_opacity, same, same;
        /// Opacity where falloff stops.
        falloff_stop_opacity, set_falloff_stop_opacity: f32 =
            no_lighting_falloff_stop_opacity, set_no_lighting_falloff_stop_opacity, same, same;
    }
}

impl SkyShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = sky_file_name, set_sky_file_name;
    }

    /// What part of the sky this is, or `None` if the stored value isn't a [`SkyObjectType`].
    pub fn sky_object_type(&self) -> Option<SkyObjectType> {
        SkyObjectType::try_from(ffi::sky_object_type(&self.0)).ok()
    }

    /// Sets the [sky object type](Self::sky_object_type).
    pub fn set_sky_object_type(self: Pin<&mut Self>, object_type: SkyObjectType) {
        ffi::set_sky_object_type(self.ffi_mut(), object_type.into());
    }
}

/// What part of the sky a [`SkyShaderProperty`] or [`BsSkyShaderProperty`] draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum SkyObjectType {
    Texture = 0,
    Sunglare = 1,
    Sky = 2,
    Clouds = 3,
    Stars = 5,
    MoonStarsMask = 7,
}

impl TileShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = tile_file_name, set_tile_file_name;
    }
}

impl TallGrassShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = tall_grass_file_name, set_tall_grass_file_name;
    }
}

/// In Fallout 76 and Starfield, a shader with a name saves none of its fields.
impl BsWaterShaderProperty {
    fields! {
        /// The water flags, as raw bits.
        flags, set_flags: u32 = bs_water_flags, set_bs_water_flags, same, same;
    }
}

/// In Fallout 76 and Starfield, a shader with a name saves none of its fields.
impl BsSkyShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        base_texture, set_base_texture = bs_sky_base_texture, set_bs_sky_base_texture;
    }

    /// What part of the sky this is, or `None` if the stored value isn't a [`SkyObjectType`].
    pub fn sky_object_type(&self) -> Option<SkyObjectType> {
        SkyObjectType::try_from(ffi::bs_sky_object_type(&self.0)).ok()
    }

    /// Sets the [sky object type](Self::sky_object_type).
    pub fn set_sky_object_type(self: Pin<&mut Self>, object_type: SkyObjectType) {
        ffi::set_bs_sky_object_type(self.ffi_mut(), object_type.into());
    }
}
