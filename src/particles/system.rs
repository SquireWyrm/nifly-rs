//! Particle systems: fields, refs and adding systems and modifiers.

use super::*;

impl NiParticleSystem {
    fields! {
        /// Whether new particles are in world space rather than the system's space.
        world_space, set_world_space: bool = particle_system_world_space, set_particle_system_world_space, same, same;
        /// Saved from Skyrim on.
        far_begin, set_far_begin: u16 = particle_system_far_begin, set_particle_system_far_begin, same, same;
        /// Saved from Skyrim on.
        far_end, set_far_end: u16 = particle_system_far_end, set_particle_system_far_end, same, same;
        /// Saved from Skyrim on.
        near_begin, set_near_begin: u16 = particle_system_near_begin, set_particle_system_near_begin, same, same;
        /// Saved from Skyrim on.
        near_end, set_near_end: u16 = particle_system_near_end, set_particle_system_near_end, same, same;
        /// The index of the active material. Saved from 20.2.0.5 on.
        active_material, set_active_material: i32 = particle_system_active_material, set_particle_system_active_material, same, same;
        /// Whether the default material needs updating. Saved from 20.2.0.7 on.
        default_material_update_flag, set_default_material_update_flag: u8 = particle_system_default_material_update_flag, set_particle_system_default_material_update_flag, same, same;
        /// The shader's implementation, saved only with a shader.
        shader_implementation, set_shader_implementation: u32 = particle_system_shader_implementation, set_particle_system_shader_implementation, same, same;
    }
}

/// Bethesda fields, saved from Skyrim SE on: the vertex flags and bounds.
impl NiParticleSystem {
    /// The eight vertex flag bytes.
    pub fn vertex_raw_flags(&self) -> [u8; 8] {
        ffi::particle_system_vertex_raw_flags(&self.0)
    }

    /// Sets [`vertex_raw_flags`](Self::vertex_raw_flags).
    pub fn set_vertex_raw_flags(&mut self, flags: [u8; 8]) {
        ffi::set_particle_system_vertex_raw_flags(self.ffi_mut(), flags);
    }

    /// The bounding sphere.
    pub fn bounds(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::particle_system_bounds(&self.0))
    }

    /// Sets [`bounds`](Self::bounds).
    pub fn set_bounds(&mut self, bounds: BoundingSphere) {
        ffi::set_particle_system_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// The bounding box's minimum and maximum. Saved only from Fallout 76 on.
    pub fn bound_min_max(&self) -> [f32; 6] {
        ffi::particle_system_bound_min_max(&self.0)
    }

    /// Sets [`bound_min_max`](Self::bound_min_max).
    pub fn set_bound_min_max(&mut self, values: [f32; 6]) {
        ffi::set_particle_system_bound_min_max(self.ffi_mut(), values);
    }
}

/// Material and shader fields, saved before Skyrim SE: files from version 20.2.0.5 save the
/// materials, files from 10.0.1.0 to 20.1.0.3 the shader.
impl NiParticleSystem {
    /// The materials: each name with its extra data.
    pub fn materials(&self) -> Vec<(&BStr, u32)> {
        let extra = ffi::particle_system_material_extra_data(&self.0);
        (0..ffi::particle_system_material_count(&self.0))
            .map(|i| (ffi::particle_system_material_name(&self.0, i).as_bstr(), extra.get(i).copied().unwrap_or(0)))
            .collect()
    }

    /// Replaces [`materials`](Self::materials).
    pub fn set_materials<N: AsRef<[u8]>>(&mut self, materials: &[(N, u32)]) {
        let (bytes, lengths) = crate::extra_data::joined(materials.iter().map(|(name, _)| name));
        let extra: Vec<u32> = materials.iter().map(|&(_, e)| e).collect();
        ffi::set_particle_system_materials(self.ffi_mut(), &bytes, &lengths, &extra);
    }

    /// The shader's name, or `None` if there's no shader.
    pub fn shader_name(&self) -> Option<&BStr> {
        ffi::particle_system_has_shader(&self.0).then(|| ffi::particle_system_shader_name(&self.0).as_bstr())
    }

    /// Sets the shader's name, or removes the shader with `None`.
    pub fn set_shader_name(&mut self, name: Option<impl AsRef<[u8]>>) {
        match name {
            Some(name) => ffi::set_particle_system_shader(self.ffi_mut(), true, name.as_ref()),
            None => ffi::set_particle_system_shader(self.ffi_mut(), false, &[]),
        }
    }
}

/// The refs of a particle system. These are on the exact type: for a subtype, `upcast` first.
impl<'a> View<'a, NiParticleSystem> {
    /// The particle data, or `None` if it has none.
    pub fn data(&self) -> Option<View<'a, NiPSysData>> {
        self.follow(RefKind::ParticleData)
    }

    /// The skin instance, or `None` if it has none.
    pub fn skin_instance(&self) -> Option<View<'a, NiBoneContainer>> {
        self.follow(RefKind::ParticleSkinInstance)
    }

    /// The shader, or `None` if it has none. Saved from Skyrim on.
    pub fn shader(&self) -> Option<View<'a, BsShaderProperty>> {
        self.follow(RefKind::ParticleShader)
    }

    /// The alpha property, or `None` if it has none. Saved from Skyrim on.
    pub fn alpha_property(&self) -> Option<View<'a, NiAlphaProperty>> {
        self.follow(RefKind::ParticleAlpha)
    }

    /// The system's modifiers, in order. Skips empty refs.
    pub fn modifiers(&self) -> BlockIter<'a, NiPSysModifier, RefIndices> {
        self.follow_list(RefList::Modifiers)
    }
}

impl Edit<'_, NiParticleSystem> {
    /// Like [`View::data`], for editing.
    pub fn data(&mut self) -> Option<Edit<'_, NiPSysData>> {
        self.follow(RefKind::ParticleData)
    }

    /// Sets the [data](View::data), or clears it with `None`. Returns `false`, changing nothing,
    /// unless `data` is an [`NiPSysData`].
    pub fn set_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleData, data)
    }

    /// Like [`View::skin_instance`], for editing.
    pub fn skin_instance(&mut self) -> Option<Edit<'_, NiBoneContainer>> {
        self.follow(RefKind::ParticleSkinInstance)
    }

    /// Sets the [skin instance](View::skin_instance), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `skin` is an [`NiBoneContainer`].
    pub fn set_skin_instance(&mut self, skin: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleSkinInstance, skin)
    }

    /// Like [`View::shader`], for editing.
    pub fn shader(&mut self) -> Option<Edit<'_, BsShaderProperty>> {
        self.follow(RefKind::ParticleShader)
    }

    /// Sets the [shader](View::shader), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `shader` is a [`BsShaderProperty`].
    pub fn set_shader(&mut self, shader: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleShader, shader)
    }

    /// Like [`View::alpha_property`], for editing.
    pub fn alpha_property(&mut self) -> Option<Edit<'_, NiAlphaProperty>> {
        self.follow(RefKind::ParticleAlpha)
    }

    /// Sets the [alpha property](View::alpha_property), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `alpha` is an [`NiAlphaProperty`].
    pub fn set_alpha_property(&mut self, alpha: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleAlpha, alpha)
    }

    /// Replaces the [modifiers](View::modifiers). Returns `false`, changing nothing, unless each
    /// is an [`NiPSysModifier`].
    pub fn set_modifiers(&mut self, modifiers: &[Index]) -> bool {
        self.set_ref_list(RefList::Modifiers, modifiers)
    }

    /// Adds a new, active `M` named `name` to the end of the [modifiers](View::modifiers),
    /// targeting the system, and returns its index. Its [order](NiPSysModifier::order) is 0.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, object3d::Transform, particles::*};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let mut root = nif.root_node_mut().unwrap();
    /// let system = root.add_particle_system::<NiParticleSystem, _>("Sparks", Transform::default());
    /// let mut system = nif.edit::<NiParticleSystem>(system).unwrap();
    /// let emitter = system.add_modifier::<NiPSysBoxEmitter>("Emitter");
    /// system.add_modifier::<NiPSysPositionModifier>("Position");
    /// let emitter = nif.block_of_mut::<NiPSysEmitter>(emitter).unwrap();
    /// emitter.set_life_span(2.0);
    /// ```
    pub fn add_modifier<M: NewModifier>(&mut self, name: impl AsRef<[u8]>) -> Index {
        let system = self.index() as u32;
        index_from_ffi(M::add(self.file_mut().inner.pin_mut(), system, name.as_ref()))
            .expect("the block is an NiParticleSystem")
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Adds a new `P` named `name`, with `transform` relative to this node, as the node's last
    /// child, and returns its index. The system gets new data of its type ([`NiPSysData`],
    /// [`NiAutoNormalParticlesData`], ...). An [`NiParticleSystem`] is in
    /// [world space](NiParticleSystem::world_space). Doesn't check that the file's game uses `P`.
    pub fn add_particle_system<P: NewParticleSystem, I>(&mut self, name: impl AsRef<[u8]>, transform: Transform) -> Index
    where
        T: IsA<NiNode, I>,
    {
        let parent = self.index() as u32;
        index_from_ffi(P::add(self.file_mut().inner.pin_mut(), parent, name.as_ref(), transform.to_ffi()))
            .expect("the block is a node")
    }
}

/// A particle system [`Edit::add_particle_system`] can add.
pub trait NewParticleSystem: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, parent: u32, name: &[u8], transform: ffi::Transform) -> u32;
}

macro_rules! new_particle_system {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewParticleSystem for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, parent: u32, name: &[u8], transform: ffi::Transform) -> u32 {
                ffi::$add(file, parent, name, transform)
            }
        }
    )*};
}

new_particle_system! {
    NiParticleSystem => add_ni_particle_system;
    NiMeshParticleSystem => add_ni_mesh_particle_system;
    BsStripParticleSystem => add_bs_strip_particle_system;
    NiParticles => add_ni_particles;
    NiAutoNormalParticles => add_ni_auto_normal_particles;
    NiRotatingParticles => add_ni_rotating_particles;
    NiParticleMeshes => add_ni_particle_meshes;
}

/// A modifier [`Edit::add_modifier`] can add.
pub trait NewModifier: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, system: u32, name: &[u8]) -> u32;
}

macro_rules! new_modifier {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewModifier for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, system: u32, name: &[u8]) -> u32 {
                ffi::$add(file, system, name)
            }
        }
    )*};
}

new_modifier! {
    BsPSysStripUpdateModifier => add_bs_psys_strip_update_modifier;
    NiPSysSpawnModifier => add_ni_psys_spawn_modifier;
    NiPSysAgeDeathModifier => add_ni_psys_age_death_modifier;
    BsPSysLodModifier => add_bs_psys_lod_modifier;
    BsPSysSimpleColorModifier => add_bs_psys_simple_color_modifier;
    NiPSysRotationModifier => add_ni_psys_rotation_modifier;
    BsPSysScaleModifier => add_bs_psys_scale_modifier;
    NiPSysGravityModifier => add_ni_psys_gravity_modifier;
    NiPSysPositionModifier => add_ni_psys_position_modifier;
    NiPSysBoundUpdateModifier => add_ni_psys_bound_update_modifier;
    NiPSysDragModifier => add_ni_psys_drag_modifier;
    BsPSysInheritVelocityModifier => add_bs_psys_inherit_velocity_modifier;
    BsPSysSubTexModifier => add_bs_psys_sub_tex_modifier;
    NiPSysBombModifier => add_ni_psys_bomb_modifier;
    NiPSysColorModifier => add_ni_psys_color_modifier;
    NiPSysGrowFadeModifier => add_ni_psys_grow_fade_modifier;
    NiPSysMeshUpdateModifier => add_ni_psys_mesh_update_modifier;
    NiPSysVortexFieldModifier => add_ni_psys_vortex_field_modifier;
    NiPSysGravityFieldModifier => add_ni_psys_gravity_field_modifier;
    NiPSysDragFieldModifier => add_ni_psys_drag_field_modifier;
    NiPSysTurbulenceFieldModifier => add_ni_psys_turbulence_field_modifier;
    NiPSysAirFieldModifier => add_ni_psys_air_field_modifier;
    NiPSysRadialFieldModifier => add_ni_psys_radial_field_modifier;
    BsWindModifier => add_bs_wind_modifier;
    BsPSysRecycleBoundModifier => add_bs_psys_recycle_bound_modifier;
    BsPSysHavokUpdateModifier => add_bs_psys_havok_update_modifier;
    BsParentVelocityModifier => add_bs_parent_velocity_modifier;
    NiPSysColliderManager => add_ni_psys_collider_manager;
    NiPSysSphereEmitter => add_ni_psys_sphere_emitter;
    NiPSysCylinderEmitter => add_ni_psys_cylinder_emitter;
    NiPSysBoxEmitter => add_ni_psys_box_emitter;
    BsPSysArrayEmitter => add_bs_psys_array_emitter;
    NiPSysMeshEmitter => add_ni_psys_mesh_emitter;
}
