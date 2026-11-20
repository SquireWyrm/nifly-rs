//! Tests for pixel data (NiPixelData, NiPersistentSrcTextureRendererData) and NiPalette.
//!
//! The new blocks aren't attached to a shape, so files are saved without optimizing, which
//! would drop them.

use std::io::Cursor;

use nifly::{
    file::{Index, NifFile, NifVersion, SaveOptions},
    handle::View,
    shaders::{
        MipMap, NiPersistentSrcTextureRendererData, NiPixelData, NiSourceTexture, PixelChannel, PixelComponent,
        PixelFormat, PixelTiling, Platform, TextureRenderData,
    },
};

fn reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, Some(SaveOptions { optimize: false, ..Default::default() })).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

/// A file with an embedded texture, as `(file, texture, pixel data)`.
fn embedded(version: NifVersion) -> (NifFile, Index, Index) {
    let mut nif = NifFile::create(version);
    let texture = nif.add_source_texture("embedded.dds");
    nif.block_of_mut::<NiSourceTexture>(texture).unwrap().set_use_external(false);
    let data = nif.edit::<NiSourceTexture>(texture).unwrap().assign_pixel_data::<NiPixelData>();
    (nif, texture, data)
}

fn texture_data(nif: &NifFile) -> View<'_, TextureRenderData> {
    let texture = nif.blocks_of::<NiSourceTexture>().views().next().unwrap();
    texture.pixel_data().unwrap()
}

#[test]
fn embedded_texture_keeps_its_data() {
    for version in [NifVersion::MORROWIND, NifVersion::OBLIVION, NifVersion::SKYRIM] {
        let (mut nif, _, data) = embedded(version);
        assert!(nif.block_of_mut::<NiPixelData>(data).unwrap().set_faces(&[&[1, 2, 3, 4]]));
        let saved = reload(&mut nif);
        let pixels = texture_data(&saved).cast::<NiPixelData>().unwrap();
        assert_eq!(pixels.faces(), [[1, 2, 3, 4]], "{version:?}");
    }
}

#[test]
fn saving_keeps_use_internal() {
    let (mut nif, texture, _) = embedded(NifVersion::OBLIVION);
    assert!(nif.block_of::<NiSourceTexture>(texture).unwrap().use_internal());
    reload(&mut nif);
    assert!(nif.block_of::<NiSourceTexture>(texture).unwrap().use_internal());
}

#[test]
fn render_data_fields() {
    let mipmaps = [MipMap { width: 4, height: 4, offset: 0 }, MipMap { width: 2, height: 2, offset: 16 }];
    let channel = PixelChannel { component: PixelComponent::Red.into(), representation: 0, bits: 8, signed: false };

    let (mut nif, _, data) = embedded(NifVersion::OBLIVION);
    let d = nif.block_of_mut::<NiPixelData>(data).unwrap();
    d.set_pixel_format(PixelFormat::Dxt5);
    d.set_bits_per_pixel(32);
    d.set_bytes_per_pixel(4);
    d.set_renderer_hint(7);
    d.set_format_extra_data(8);
    d.set_raw_flags(1);
    d.set_tiling(PixelTiling::Xenon);
    d.set_channels([channel; 4]);
    d.set_color_masks([1, 2, 3, 4]);
    d.set_mipmaps(&mipmaps);

    let saved = reload(&mut nif);
    let d = texture_data(&saved);
    assert_eq!(d.pixel_format(), Some(PixelFormat::Dxt5));
    assert_eq!((d.bits_per_pixel(), d.bytes_per_pixel()), (32, 4));
    assert_eq!((d.renderer_hint(), d.format_extra_data(), d.raw_flags()), (7, 8, 1));
    assert_eq!(d.tiling(), Some(PixelTiling::Xenon));
    assert_eq!(d.channels(), [channel; 4]);
    assert_eq!(d.channels()[0].component(), Some(PixelComponent::Red));
    assert_eq!(d.mipmaps(), mipmaps);
    // Color masks are only saved up to Morrowind.
    assert_eq!(d.color_masks(), [0; 4]);

    let (mut nif, _, data) = embedded(NifVersion::MORROWIND);
    let d = nif.block_of_mut::<NiPixelData>(data).unwrap();
    d.set_color_masks([1, 2, 3, 4]);
    d.set_old_fast_compare([1, 2, 3, 4, 5, 6, 7, 8]);
    let saved = reload(&mut nif);
    assert_eq!(texture_data(&saved).color_masks(), [1, 2, 3, 4]);
    assert_eq!(texture_data(&saved).old_fast_compare(), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn faces() {
    let (mut nif, _, data) = embedded(NifVersion::OBLIVION);
    let d = nif.block_of_mut::<NiPixelData>(data).unwrap();
    assert!(!d.set_faces(&[&[1, 2], &[3]]));
    assert!(d.set_faces(&[&[1, 2], &[3, 4]]));
    let saved = reload(&mut nif);
    assert_eq!(texture_data(&saved).cast::<NiPixelData>().unwrap().faces(), [[1, 2], [3, 4]]);

    // Morrowind stores one face.
    let (mut nif, _, data) = embedded(NifVersion::MORROWIND);
    assert!(nif.block_of_mut::<NiPixelData>(data).unwrap().set_faces(&[&[1, 2], &[3, 4]]));
    let saved = reload(&mut nif);
    assert_eq!(texture_data(&saved).cast::<NiPixelData>().unwrap().faces(), [[1, 2]]);
}

#[test]
fn persistent_data() {
    let mut nif = NifFile::create(NifVersion::SKYRIM);
    let texture = nif.add_source_texture("cube.dds");
    let data = nif.edit::<NiSourceTexture>(texture).unwrap().assign_pixel_data::<NiPersistentSrcTextureRendererData>();
    let faces: Vec<Vec<u8>> = (0..6).map(|f| vec![f; 3]).collect();
    let faces: Vec<&[u8]> = faces.iter().map(Vec::as_slice).collect();
    let d = nif.block_of_mut::<NiPersistentSrcTextureRendererData>(data).unwrap();
    assert!(d.set_faces(&faces));
    d.set_platform(Platform::Dx9);
    d.set_padded_pixel_count(4);

    let saved = reload(&mut nif);
    let d = texture_data(&saved).cast::<NiPersistentSrcTextureRendererData>().unwrap();
    assert_eq!(d.faces(), faces);
    assert_eq!((d.platform(), d.padded_pixel_count()), (Some(Platform::Dx9), 4));
}

#[test]
fn palette() {
    let (mut nif, texture, data) = embedded(NifVersion::OBLIVION);
    let colors: Vec<[u8; 4]> = (0..=255).map(|i| [i, 255 - i, 0, 255]).collect();
    let mut edit = nif.edit::<TextureRenderData>(data).unwrap();
    assert_eq!(edit.assign_palette(&colors[..17], false), None);
    let palette = edit.assign_palette(&colors, true).unwrap();
    assert!(edit.palette().is_some());
    // Only pixel data has a palette.
    assert!(nif.edit::<TextureRenderData>(texture).is_none());

    let saved = reload(&mut nif);
    let p = texture_data(&saved).palette().unwrap();
    assert!(p.has_alpha());
    assert_eq!(p.colors(), colors);

    let p = nif.block_of_mut::<nifly::shaders::NiPalette>(palette).unwrap();
    assert!(!p.set_colors(&colors[..100]));
    assert!(p.set_colors(&colors[..16]));
    p.set_has_alpha(false);
    let saved = reload(&mut nif);
    let p = texture_data(&saved).palette().unwrap();
    assert_eq!((p.colors(), p.has_alpha()), (colors[..16].to_vec(), false));
}

#[test]
fn rejects_other_blocks() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let root = nif.root_node().unwrap().index();
    assert!(nif.edit::<NiSourceTexture>(root).is_none());
    assert!(nif.edit::<NiSourceTexture>(usize::MAX).is_none());
    assert!(nif.view::<TextureRenderData>(root).is_none());
}
