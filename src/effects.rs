//! Dynamic effects: lights and projected textures.
//!
//! A node lists the effects that apply to it ([`View::effects`]), and an effect lists the nodes
//! it affects ([`View::affected_nodes`]). Effects are also scene objects with a transform, usually
//! a node's children. Add one with [`Edit::add_effect`].
//!
//! ```text
//! NiSpotLight -> NiPointLight -> NiLight -> NiDynamicEffect -> NiAvObject
//! ```

use std::pin::Pin;

use int_enum::IntEnum;
use nifly_sys::{effects::ffi, nif_file::ffi as nif_ffi};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, Index},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    nodes::NiNode,
    object3d::{Matrix3, Vec3},
    objects::{IsA, NiAvObject, NiType},
    shaders::{ClampMode, FilterMode, NiSourceTexture},
};

ni_types! { ffi;
    /// A light or texture effect. Its affected nodes are on [`View::affected_nodes`].
    NiDynamicEffect(NiDynamicEffect): NiAvObject => upcast_ni_dynamic_effect, downcast_ni_dynamic_effect;
    /// A texture projected onto the nodes it affects, such as an environment map. Its texture is
    /// on [`View::source_texture`].
    NiTextureEffect(NiTextureEffect): NiDynamicEffect => upcast_ni_texture_effect, downcast_ni_texture_effect;
    NiLight(NiLight): NiDynamicEffect => upcast_ni_light, downcast_ni_light;
    /// A light lighting everything equally.
    NiAmbientLight(NiAmbientLight): NiLight => upcast_ni_ambient_light, downcast_ni_ambient_light;
    /// A light from far away, shining along its X axis.
    NiDirectionalLight(NiDirectionalLight): NiLight => upcast_ni_directional_light, downcast_ni_directional_light;
    /// A light shining in every direction from its position, fading with distance.
    NiPointLight(NiPointLight): NiLight => upcast_ni_point_light, downcast_ni_point_light;
    /// A point light shining in a cone along its X axis.
    NiSpotLight(NiSpotLight): NiPointLight => upcast_ni_spot_light, downcast_ni_spot_light;
}

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

fn rgb(c: ffi::Vec3) -> [f32; 3] {
    [c.x, c.y, c.z]
}

fn rgb_to_ffi([x, y, z]: [f32; 3]) -> ffi::Vec3 {
    ffi::Vec3 { x, y, z }
}

impl NiDynamicEffect {
    fields! {
        /// Whether the effect is on. Saved from Oblivion until Fallout 4, which doesn't save it.
        switch_state, set_switch_state: bool = effect_switch_state, set_effect_switch_state, same, same;
    }

    /// The affected nodes as Morrowind files store them: memory addresses from the program that
    /// saved the file, which don't identify blocks. Only saved in Morrowind files; later files
    /// save [`View::affected_nodes`] instead.
    pub fn affected_node_pointers(&self) -> Vec<u32> {
        ffi::effect_affected_node_pointers(&self.0)
    }

    /// Sets [`affected_node_pointers`](Self::affected_node_pointers).
    pub fn set_affected_node_pointers(&mut self, pointers: &[u32]) {
        ffi::set_effect_affected_node_pointers(self.ffi_mut(), pointers);
    }
}

/// What an [`NiTextureEffect`]'s texture is used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum EffectType {
    /// Projected as light, brightening what it falls on.
    ProjectedLight = 0,
    /// Projected as a shadow, darkening what it falls on.
    ProjectedShadow = 1,
    /// An environment map, reflected by what it affects.
    EnvironmentMap = 2,
    /// A fog map.
    FogMap = 3,
}

/// How an [`NiTextureEffect`] generates texture coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum CoordinateGeneration {
    /// A parallel projection.
    WorldParallel = 0,
    /// A perspective projection.
    WorldPerspective = 1,
    /// A sphere map.
    SphereMap = 2,
    /// A specular cube map, for an `NiSourceCubeMap` texture.
    SpecularCubeMap = 3,
    /// A diffuse cube map, for an `NiSourceCubeMap` texture.
    DiffuseCubeMap = 4,
}

/// A plane: the points `p` where `normal.dot(p) == constant`.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Plane {
    pub normal: Vec3,
    pub constant: f32,
}

impl NiTextureEffect {
    /// The projection's rotation and scale, usually the identity.
    pub fn projection_matrix(&self) -> Matrix3 {
        let mut rows = [Vec3::ZERO.to_ffi(); 3];
        ffi::texture_effect_matrix(&self.0, &mut rows);
        Matrix3 { rows: rows.map(Vec3::from_ffi) }
    }

    /// Sets the [projection matrix](Self::projection_matrix).
    pub fn set_projection_matrix(&mut self, matrix: Matrix3) {
        ffi::set_texture_effect_matrix(self.ffi_mut(), &matrix.rows.map(Vec3::to_ffi));
    }

    fields! {
        /// The projection's translation, usually zero.
        projection_translation, set_projection_translation: Vec3 =
            texture_effect_translation, set_texture_effect_translation, Vec3::from_ffi, Vec3::to_ffi;
        /// Whether the [clipping plane](Self::clipping_plane) is used.
        clipping, set_clipping: bool = texture_effect_clipping, set_texture_effect_clipping, |v| v != 0, u8::from;
        /// A PlayStation 2 setting. Only saved in Morrowind files.
        ps2_l, set_ps2_l: i16 = texture_effect_ps2_l, set_texture_effect_ps2_l, same, same;
        /// A PlayStation 2 setting. Only saved in Morrowind files.
        ps2_k, set_ps2_k: i16 = texture_effect_ps2_k, set_texture_effect_ps2_k, same, same;
        /// The stored byte behind [`clipping`](Self::clipping); any value but 0 turns it on.
        clipping_value, set_clipping_value: u8 = texture_effect_clipping, set_texture_effect_clipping, same, same;
        /// A value of unknown meaning, saved in files up to version 4.1.0.12.
        unknown, set_unknown: u16 = texture_effect_unknown, set_texture_effect_unknown, same, same;
    }

    /// How the texture is filtered, or `None` if the stored value isn't a [`FilterMode`].
    pub fn filter_mode(&self) -> Option<FilterMode> {
        FilterMode::try_from(ffi::texture_effect_filtering(&self.0)).ok()
    }

    /// Sets the [filter mode](Self::filter_mode).
    pub fn set_filter_mode(&mut self, mode: FilterMode) {
        ffi::set_texture_effect_filtering(self.ffi_mut(), mode.into());
    }

    /// How the texture repeats, or `None` if the stored value isn't a [`ClampMode`].
    pub fn clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(ffi::texture_effect_clamping(&self.0)).ok()
    }

    /// Sets the [clamp mode](Self::clamp_mode).
    pub fn set_clamp_mode(&mut self, mode: ClampMode) {
        ffi::set_texture_effect_clamping(self.ffi_mut(), mode.into());
    }

    /// What the texture is used for, or `None` if the stored value isn't an [`EffectType`].
    pub fn effect_type(&self) -> Option<EffectType> {
        EffectType::try_from(ffi::texture_effect_type(&self.0)).ok()
    }

    /// Sets the [effect type](Self::effect_type).
    pub fn set_effect_type(&mut self, effect_type: EffectType) {
        ffi::set_texture_effect_type(self.ffi_mut(), effect_type.into());
    }

    /// How texture coordinates are generated, or `None` if the stored value isn't a
    /// [`CoordinateGeneration`].
    pub fn coordinate_generation(&self) -> Option<CoordinateGeneration> {
        CoordinateGeneration::try_from(ffi::texture_effect_coordinates(&self.0)).ok()
    }

    /// Sets the [coordinate generation](Self::coordinate_generation).
    pub fn set_coordinate_generation(&mut self, generation: CoordinateGeneration) {
        ffi::set_texture_effect_coordinates(self.ffi_mut(), generation.into());
    }

    /// The plane the effect is clipped by, if [clipping](Self::clipping) is on.
    pub fn clipping_plane(&self) -> Plane {
        Plane {
            normal: Vec3::from_ffi(ffi::texture_effect_plane_normal(&self.0)),
            constant: ffi::texture_effect_plane_constant(&self.0),
        }
    }

    /// Sets the [clipping plane](Self::clipping_plane).
    pub fn set_clipping_plane(&mut self, plane: Plane) {
        ffi::set_texture_effect_plane_normal(self.ffi_mut(), plane.normal.to_ffi());
        ffi::set_texture_effect_plane_constant(self.ffi_mut(), plane.constant);
    }
}

impl NiLight {
    fields! {
        /// Scales the light's brightness.
        dimmer, set_dimmer: f32 = light_dimmer, set_light_dimmer, same, same;
        /// The ambient color, as `[r, g, b]`.
        ambient_color, set_ambient_color: [f32; 3] = light_ambient, set_light_ambient, rgb, rgb_to_ffi;
        /// The diffuse color, as `[r, g, b]`.
        diffuse_color, set_diffuse_color: [f32; 3] = light_diffuse, set_light_diffuse, rgb, rgb_to_ffi;
        /// The specular color, as `[r, g, b]`.
        specular_color, set_specular_color: [f32; 3] = light_specular, set_light_specular, rgb, rgb_to_ffi;
    }
}

impl NiPointLight {
    fields! {
        /// The constant term of the light's falloff with distance `d`:
        /// `1 / (constant + linear * d + quadratic * d²)`.
        constant_attenuation, set_constant_attenuation: f32 =
            point_light_constant, set_point_light_constant, same, same;
        /// The linear term of the [falloff](Self::constant_attenuation).
        linear_attenuation, set_linear_attenuation: f32 = point_light_linear, set_point_light_linear, same, same;
        /// The quadratic term of the [falloff](Self::constant_attenuation).
        quadratic_attenuation, set_quadratic_attenuation: f32 =
            point_light_quadratic, set_point_light_quadratic, same, same;
    }
}

impl NiSpotLight {
    fields! {
        /// The angle from the axis to the edge of the cone.
        outer_angle, set_outer_angle: f32 = spot_light_outer, set_spot_light_outer, same, same;
        /// The angle from the axis within which the light is at full strength. Saved from
        /// Fallout 3 on.
        inner_angle, set_inner_angle: f32 = spot_light_inner, set_spot_light_inner, same, same;
        /// How the light is distributed across the cone.
        exponent, set_exponent: f32 = spot_light_exponent, set_spot_light_exponent, same, same;
    }
}

/// The effects on nodes, and the nodes affected by effects.
impl<'a, T: NiType> View<'a, T> {
    /// The effects applied to the node. Skips empty refs and refs that don't resolve to an
    /// [`NiDynamicEffect`].
    pub fn effects<I>(&self) -> BlockIter<'a, NiDynamicEffect, RefIndices>
    where
        T: IsA<NiNode, I>,
    {
        let node: &NiNode = self.get().upcast_ref();
        BlockIter::new(self.file(), RefIndices::new(ffi::node_effects(node.as_ffi())))
    }

    /// The nodes the effect applies to, with their subtrees. Saved from Oblivion until Fallout 4,
    /// which doesn't save them; Morrowind saves
    /// [`affected_node_pointers`](NiDynamicEffect::affected_node_pointers) instead.
    pub fn affected_nodes<I>(&self) -> BlockIter<'a, NiNode, RefIndices>
    where
        T: IsA<NiDynamicEffect, I>,
    {
        let effect: &NiDynamicEffect = self.get().upcast_ref();
        BlockIter::new(self.file(), RefIndices::new(ffi::effect_affected_nodes(&effect.0)))
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Replaces the node's [effects](View::effects). Returns `false`, changing nothing, if one
    /// isn't an [`NiDynamicEffect`].
    ///
    /// Fallout 4 and later files don't save effect lists.
    pub fn set_effects<I>(&mut self, effects: &[Index]) -> bool
    where
        T: IsA<NiNode, I>,
    {
        let Some(effects) = effects.iter().map(|&e| block_index_to_ffi(e)).collect::<Option<Vec<_>>>() else {
            return false;
        };
        let node = self.index() as u32;
        ffi::set_node_effects(self.file_mut().inner.pin_mut(), node, &effects)
    }

    /// Adds a new `E` to the end of the node's [effects](View::effects) and returns its index.
    /// It's added at the end of the block list. New lights have a [dimmer](NiLight::dimmer) of 1
    /// and are black; new point and spot lights fade linearly.
    ///
    /// Fallout 4 and later files don't save effect lists. To give the effect a place in the
    /// scene, also make it a child with [`Edit::set_parent`].
    ///
    /// ```
    /// # use nifly::{effects::NiPointLight, file::NifFile, nodes::NiNode};
    /// # let mut nif = NifFile::load(std::fs::File::open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif").unwrap(), None).unwrap();
    /// let light = nif.edit::<NiNode>(0).unwrap().add_effect::<NiPointLight, _>();
    /// nif.edit::<NiPointLight>(light).unwrap().set_parent(Some(0));
    /// let mut edit = nif.edit::<NiPointLight>(light).unwrap();
    /// edit.set_diffuse_color([1.0, 0.8, 0.6]);
    /// ```
    pub fn add_effect<E: NewEffect, I>(&mut self) -> Index
    where
        T: IsA<NiNode, I>,
    {
        let node = self.index() as u32;
        index_from_ffi(E::add(self.file_mut().inner.pin_mut(), node)).expect("the block is an NiNode")
    }

    /// Replaces the effect's [affected nodes](View::affected_nodes). Returns `false`, changing
    /// nothing, if one isn't an [`NiNode`].
    pub fn set_affected_nodes<I>(&mut self, nodes: &[Index]) -> bool
    where
        T: IsA<NiDynamicEffect, I>,
    {
        let Some(nodes) = nodes.iter().map(|&n| block_index_to_ffi(n)).collect::<Option<Vec<_>>>() else {
            return false;
        };
        let effect = self.index() as u32;
        ffi::set_effect_affected_nodes(self.file_mut().inner.pin_mut(), effect, &nodes)
    }
}

/// The texture of an [`NiTextureEffect`].
impl<'a> View<'a, NiTextureEffect> {
    /// The projected texture, or `None` if it has none.
    pub fn source_texture(&self) -> Option<View<'a, NiSourceTexture>> {
        self.resolve(index_from_ffi(ffi::texture_effect_source(&self.get().0)))
    }
}

impl Edit<'_, NiTextureEffect> {
    /// Like [`View::source_texture`], for editing.
    pub fn source_texture(&mut self) -> Option<Edit<'_, NiSourceTexture>> {
        let index = self.view().source_texture()?.index();
        self.resolve(Some(index))
    }

    /// Projects the [`NiSourceTexture`] at block index `texture`, or none if `None`. Returns
    /// `false`, changing nothing, if that block isn't an `NiSourceTexture`. Create one with
    /// [`NifFile::add_source_texture`](crate::file::NifFile::add_source_texture).
    pub fn set_source_texture(&mut self, texture: Option<Index>) -> bool {
        let texture = match texture {
            Some(t) => match block_index_to_ffi(t) {
                Some(t) => t,
                None => return false,
            },
            None => u32::MAX,
        };
        let effect = self.index() as u32;
        ffi::set_texture_effect_source(self.file_mut().inner.pin_mut(), effect, texture)
    }
}

/// An effect type that [`Edit::add_effect`] can create.
///
/// This trait is sealed.
pub trait NewEffect: NiType {
    /// Adds a default effect to the node at `node` and returns its index (`u32::MAX` on
    /// failure).
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32;
}

macro_rules! new_effect {
    ($($ty:ty => $add:ident;)*) => {$(
        impl NewEffect for $ty {
            fn add(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32 {
                ffi::$add(file, node)
            }
        }
    )*};
}

new_effect! {
    NiTextureEffect => add_texture_effect;
    NiAmbientLight => add_ambient_light;
    NiDirectionalLight => add_directional_light;
    NiPointLight => add_point_light;
    NiSpotLight => add_spot_light;
}
