//! Shader, property and texture methods on the block handles.

use super::*;

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
