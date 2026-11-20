//! Source textures, pixel data and palettes.

use super::*;

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
    pub fn set_pixel_layout(&mut self, layout: PixelLayout) {
        ffi::set_source_pixel_layout(self.ffi_mut(), layout.into());
    }

    /// Whether the texture uses mipmaps, or `None` if the stored value isn't a
    /// [`MipMapFormat`].
    pub fn mip_map_format(&self) -> Option<MipMapFormat> {
        MipMapFormat::try_from(ffi::source_mip_map_format(&self.0)).ok()
    }

    /// Sets the [mipmap format](Self::mip_map_format).
    pub fn set_mip_map_format(&mut self, format: MipMapFormat) {
        ffi::set_source_mip_map_format(self.ffi_mut(), format.into());
    }

    /// How the texture's alpha is used, or `None` if the stored value isn't an
    /// [`AlphaFormat`].
    pub fn alpha_format(&self) -> Option<AlphaFormat> {
        AlphaFormat::try_from(ffi::source_alpha_format(&self.0)).ok()
    }

    /// Sets the [alpha format](Self::alpha_format).
    pub fn set_alpha_format(&mut self, format: AlphaFormat) {
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
    pub fn set_pixel_format(&mut self, format: PixelFormat) {
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
        raw_flags, set_raw_flags: u8 = render_data_flags, set_render_data_flags, same, same;
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
    pub fn set_tiling(&mut self, tiling: PixelTiling) {
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
    pub fn set_color_masks(&mut self, masks: [u32; 4]) {
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
    pub fn set_channels(&mut self, channels: [PixelChannel; 4]) {
        for (i, c) in channels.iter().enumerate() {
            let v = [c.component, c.representation, c.bits.into(), c.signed.into()];
            ffi::set_render_data_channel(self.ffi_mut(), i, &v);
        }
    }

    /// The mipmaps, largest first. Each one's pixels start at its
    /// [offset](MipMap::offset) in every face.
    pub fn mipmaps(&self) -> Vec<MipMap> {
        let flat = ffi::render_data_mipmaps(&self.0);
        flat.as_chunks().0.iter().map(|&[width, height, offset]| MipMap { width, height, offset }).collect()
    }

    /// Sets the [mipmaps](Self::mipmaps). Not checked against the pixel bytes.
    pub fn set_mipmaps(&mut self, mipmaps: &[MipMap]) {
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
        pub fn set_faces(&mut self, faces: &[&[u8]]) -> bool {
            let len = faces.first().map_or(0, |f| f.len());
            if u32::try_from(len).is_err() || faces.iter().any(|f| f.len() != len) {
                return false;
            }
            ffi::$clear(self.ffi_mut());
            for face in faces {
                ffi::$push(self.ffi_mut(), face);
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
    pub fn set_platform(&mut self, platform: Platform) {
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

    pub fn set_has_alpha(&mut self, value: bool) {
        ffi::set_palette_has_alpha(self.ffi_mut(), value);
    }

    /// The colors as `[r, g, b, a]`.
    pub fn colors(&self) -> Vec<[u8; 4]> {
        ffi::palette_colors(&self.0).as_chunks().0.to_vec()
    }

    /// Sets the [colors](Self::colors). Returns `false`, changing nothing, unless there are 16 or
    /// 256.
    pub fn set_colors(&mut self, colors: &[[u8; 4]]) -> bool {
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
