//! Modifier, collider and controller refs, and adding colliders and controllers.

use super::*;

/// Modifier, collider and emitter controller refs.
impl<'a, T: NiType> View<'a, T> {
    /// The particle system the modifier belongs to, or `None` if it has none.
    pub fn particle_system<I>(&self) -> Option<View<'a, NiParticleSystem>>
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.follow(RefKind::ParticleSystem)
    }

    /// The modifier that spawns new particles, or `None` if it has none.
    pub fn spawn_modifier(&self) -> Option<View<'a, NiPSysSpawnModifier>>
    where
        T: HasSpawnModifier,
    {
        self.follow(RefKind::SpawnModifier)
    }

    /// The object whose position and orientation the modifier or collider works around, or
    /// `None` if it has none: the emitter's node for volume emitters, the field's object for
    /// field modifiers.
    pub fn modifier_object(&self) -> Option<View<'a, NiAvObject>>
    where
        T: HasModifierObject,
    {
        self.follow(RefKind::ModifierObject)
    }

    /// The colors over the particles' life, or `None` if there are none.
    pub fn color_data(&self) -> Option<View<'a, NiColorData>>
    where
        T: HasColorData,
    {
        self.follow(RefKind::ColorData)
    }

    /// The meshes the modifier updates or emits from. Skips empty refs.
    pub fn particle_meshes(&self) -> BlockIter<'a, NiAvObject, RefIndices>
    where
        T: HasParticleMeshes,
    {
        self.follow_list(RefList::ParticleMeshes)
    }

    /// The colliders of a collider manager: the first, then each one's
    /// [next collider](View::next_collider). Stops at an empty ref or a collider already listed.
    pub fn colliders<I>(&self) -> BlockIter<'a, NiPSysCollider, RefIndices>
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        self.follow_list(RefList::Colliders)
    }

    /// The collider after this one, or `None` if it's the last.
    pub fn next_collider<I>(&self) -> Option<View<'a, NiPSysCollider>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::NextCollider)
    }

    /// The manager the collider belongs to, or `None` if it has none.
    pub fn collider_manager<I>(&self) -> Option<View<'a, NiPSysColliderManager>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::ColliderManager)
    }

    /// The emitter controller's keys, or `None` if it has none. Saved up to 10.1.0.103; later
    /// controllers use the [interpolator](View::interpolator) and
    /// [visibility interpolator](View::visibility_interpolator).
    pub fn emitter_data<I>(&self) -> Option<View<'a, NiPSysEmitterCtlrData>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::EmitterCtlrData)
    }

    /// The interpolator that switches the emitter on and off, or `None` if it has none. Saved
    /// from 10.1.0.104 on.
    pub fn visibility_interpolator<I>(&self) -> Option<View<'a, NiInterpolator>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::VisibilityInterpolator)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::particle_system`], for editing.
    pub fn particle_system<I>(&mut self) -> Option<Edit<'_, NiParticleSystem>>
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.follow(RefKind::ParticleSystem)
    }

    /// Sets the [particle system](View::particle_system), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `system` is an [`NiParticleSystem`].
    pub fn set_particle_system<I>(&mut self, system: Option<Index>) -> bool
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.set_ref(RefKind::ParticleSystem, system)
    }

    /// Like [`View::spawn_modifier`], for editing.
    pub fn spawn_modifier(&mut self) -> Option<Edit<'_, NiPSysSpawnModifier>>
    where
        T: HasSpawnModifier,
    {
        self.follow(RefKind::SpawnModifier)
    }

    /// Sets the [spawn modifier](View::spawn_modifier), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `modifier` is an [`NiPSysSpawnModifier`].
    pub fn set_spawn_modifier(&mut self, modifier: Option<Index>) -> bool
    where
        T: HasSpawnModifier,
    {
        self.set_ref(RefKind::SpawnModifier, modifier)
    }

    /// Like [`View::modifier_object`], for editing.
    pub fn modifier_object(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: HasModifierObject,
    {
        self.follow(RefKind::ModifierObject)
    }

    /// Sets the [modifier object](View::modifier_object), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `object` is an [`NiNode`] for an [`NiPSysBombModifier`]
    /// or [`NiPSysVolumeEmitter`], or an [`NiAvObject`] for the others.
    pub fn set_modifier_object(&mut self, object: Option<Index>) -> bool
    where
        T: HasModifierObject,
    {
        self.set_ref(RefKind::ModifierObject, object)
    }

    /// Like [`View::color_data`], for editing.
    pub fn color_data(&mut self) -> Option<Edit<'_, NiColorData>>
    where
        T: HasColorData,
    {
        self.follow(RefKind::ColorData)
    }

    /// Sets the [color data](View::color_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiColorData`].
    pub fn set_color_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasColorData,
    {
        self.set_ref(RefKind::ColorData, data)
    }

    /// Replaces the [particle meshes](View::particle_meshes). Returns `false`, changing nothing,
    /// unless each is an [`NiAvObject`].
    pub fn set_particle_meshes(&mut self, meshes: &[Index]) -> bool
    where
        T: HasParticleMeshes,
    {
        self.set_ref_list(RefList::ParticleMeshes, meshes)
    }

    /// Replaces the [colliders](View::colliders), linking each to the next. Returns `false`,
    /// changing nothing, unless they're distinct [`NiPSysCollider`]s. Doesn't change their
    /// [manager](View::collider_manager).
    pub fn set_colliders<I>(&mut self, colliders: &[Index]) -> bool
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        self.set_ref_list(RefList::Colliders, colliders)
    }

    /// Adds a new `C` to the end of the [colliders](View::colliders), managed by this manager,
    /// and returns its index. A ref in the chain that isn't a collider, or that repeats one, ends
    /// the chain and is replaced.
    pub fn add_collider<C: NewCollider, I>(&mut self) -> Index
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        let manager = self.index() as u32;
        index_from_ffi(C::add(self.file_mut().inner.pin_mut(), manager))
            .expect("the block is an NiPSysColliderManager")
    }

    /// Like [`View::next_collider`], for editing.
    pub fn next_collider<I>(&mut self) -> Option<Edit<'_, NiPSysCollider>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::NextCollider)
    }

    /// Sets the [next collider](View::next_collider), or ends the chain with `None`. Returns
    /// `false`, changing nothing, unless `next` is an [`NiPSysCollider`] whose chain doesn't lead
    /// back to this one.
    pub fn set_next_collider<I>(&mut self, next: Option<Index>) -> bool
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.set_ref(RefKind::NextCollider, next)
    }

    /// Like [`View::collider_manager`], for editing.
    pub fn collider_manager<I>(&mut self) -> Option<Edit<'_, NiPSysColliderManager>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::ColliderManager)
    }

    /// Sets the [collider manager](View::collider_manager), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `manager` is an [`NiPSysColliderManager`].
    pub fn set_collider_manager<I>(&mut self, manager: Option<Index>) -> bool
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.set_ref(RefKind::ColliderManager, manager)
    }

    /// Like [`View::emitter_data`], for editing.
    pub fn emitter_data<I>(&mut self) -> Option<Edit<'_, NiPSysEmitterCtlrData>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::EmitterCtlrData)
    }

    /// Sets the [emitter data](View::emitter_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiPSysEmitterCtlrData`].
    pub fn set_emitter_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.set_ref(RefKind::EmitterCtlrData, data)
    }

    /// Like [`View::visibility_interpolator`], for editing.
    pub fn visibility_interpolator<I>(&mut self) -> Option<Edit<'_, NiInterpolator>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::VisibilityInterpolator)
    }

    /// Sets the [visibility interpolator](View::visibility_interpolator), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `interpolator` is an
    /// [`NiInterpolator`].
    pub fn set_visibility_interpolator<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.set_ref(RefKind::VisibilityInterpolator, interpolator)
    }
}

/// The refs of a [`BsPSysHavokUpdateModifier`].
impl<'a> View<'a, BsPSysHavokUpdateModifier> {
    /// The nodes whose Havok bodies move the particles. Skips empty refs.
    pub fn havok_nodes(&self) -> BlockIter<'a, NiNode, RefIndices> {
        self.follow_list(RefList::HavokUpdateNodes)
    }

    /// The modifier it works with, or `None` if it has none.
    pub fn havok_modifier(&self) -> Option<View<'a, NiPSysModifier>> {
        self.follow(RefKind::HavokUpdateModifier)
    }
}

impl Edit<'_, BsPSysHavokUpdateModifier> {
    /// Replaces the [Havok nodes](View::havok_nodes). Returns `false`, changing nothing, unless
    /// each is an [`NiNode`].
    pub fn set_havok_nodes(&mut self, nodes: &[Index]) -> bool {
        self.set_ref_list(RefList::HavokUpdateNodes, nodes)
    }

    /// Like [`View::havok_modifier`], for editing.
    pub fn havok_modifier(&mut self) -> Option<Edit<'_, NiPSysModifier>> {
        self.follow(RefKind::HavokUpdateModifier)
    }

    /// Sets the [Havok modifier](View::havok_modifier), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `modifier` is an [`NiPSysModifier`].
    pub fn set_havok_modifier(&mut self, modifier: Option<Index>) -> bool {
        self.set_ref(RefKind::HavokUpdateModifier, modifier)
    }
}

/// The node of an [`NiMeshPSysData`].
impl<'a> View<'a, NiMeshPSysData> {
    /// The node holding the particle meshes, or `None` if it has none.
    pub fn mesh_node(&self) -> Option<View<'a, NiNode>> {
        self.follow(RefKind::MeshPSysNode)
    }
}

impl Edit<'_, NiMeshPSysData> {
    /// Like [`View::mesh_node`], for editing.
    pub fn mesh_node(&mut self) -> Option<Edit<'_, NiNode>> {
        self.follow(RefKind::MeshPSysNode)
    }

    /// Sets the [mesh node](View::mesh_node), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `node` is an [`NiNode`].
    pub fn set_mesh_node(&mut self, node: Option<Index>) -> bool {
        self.set_ref(RefKind::MeshPSysNode, node)
    }
}

/// The master system of a [`BsPSysMultiTargetEmitterCtlr`].
impl<'a> View<'a, BsPSysMultiTargetEmitterCtlr> {
    /// The master particle system, or `None` if it has none.
    pub fn master_particle_system(&self) -> Option<View<'a, BsMasterParticleSystem>> {
        self.follow(RefKind::MasterParticleSystem)
    }
}

impl Edit<'_, BsPSysMultiTargetEmitterCtlr> {
    /// Like [`View::master_particle_system`], for editing.
    pub fn master_particle_system(&mut self) -> Option<Edit<'_, BsMasterParticleSystem>> {
        self.follow(RefKind::MasterParticleSystem)
    }

    /// Sets the [master particle system](View::master_particle_system), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `system` is a
    /// [`BsMasterParticleSystem`].
    pub fn set_master_particle_system(&mut self, system: Option<Index>) -> bool {
        self.set_ref(RefKind::MasterParticleSystem, system)
    }
}

/// An [`NiPSysAgeDeathModifier`] or [`NiPSysCollider`]: a block with a spawn modifier.
///
/// This trait is sealed.
pub trait HasSpawnModifier: NiType {}

/// A particle modifier or collider that works around an object: gravity, drag, inherit
/// velocity, bomb, field and recycle bound modifiers, volume emitters and colliders.
///
/// This trait is sealed.
pub trait HasModifierObject: NiType {}

/// An [`NiPSysColorModifier`] or [`NiParticleColorModifier`]: a block with color data.
///
/// This trait is sealed.
pub trait HasColorData: NiType {}

/// An [`NiPSysMeshUpdateModifier`], [`NiPSysMeshEmitter`] or [`NiParticleMeshModifier`]: a block
/// with a list of meshes.
///
/// This trait is sealed.
pub trait HasParticleMeshes: NiType {}

macro_rules! impl_marker {
    ($tr:ident: $($t:ident)*) => {$(impl $tr for $t {})*};
}

impl_marker! { HasSpawnModifier:
    NiPSysAgeDeathModifier NiPSysCollider NiPSysSphericalCollider NiPSysPlanarCollider
}

impl_marker! { HasModifierObject:
    NiPSysGravityModifier NiPSysDragModifier BsPSysInheritVelocityModifier NiPSysBombModifier
    NiPSysFieldModifier NiPSysVortexFieldModifier NiPSysGravityFieldModifier NiPSysDragFieldModifier
    NiPSysTurbulenceFieldModifier NiPSysAirFieldModifier NiPSysRadialFieldModifier
    BsPSysRecycleBoundModifier NiPSysVolumeEmitter NiPSysSphereEmitter NiPSysCylinderEmitter
    NiPSysBoxEmitter BsPSysArrayEmitter NiPSysCollider NiPSysSphericalCollider NiPSysPlanarCollider
}

impl_marker! { HasColorData: NiPSysColorModifier NiParticleColorModifier }

impl_marker! { HasParticleMeshes: NiPSysMeshUpdateModifier NiPSysMeshEmitter NiParticleMeshModifier }

impl_marker! { HasInterpolator: NiPSysEmitterCtlr BsPSysMultiTargetEmitterCtlr }

/// A collider [`Edit::add_collider`] can add.
pub trait NewCollider: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32;
}

impl NewCollider for NiPSysSphericalCollider {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32 {
        ffi::add_ni_psys_spherical_collider(file, manager)
    }
}

impl NewCollider for NiPSysPlanarCollider {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32 {
        ffi::add_ni_psys_planar_collider(file, manager)
    }
}

macro_rules! new_controller {
    ($($t:ty => $add:ident;)*) => {$(
        impl NewController for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, owner: u32) -> u32 {
                ffi::$add(file, owner)
            }
        }
    )*};
}

impl NewAnimationData for NiPSysEmitterCtlrData {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32 {
        ffi::add_ni_psys_emitter_ctlr_data(file)
    }
}

new_controller! {
    NiParticleSystemController => add_ni_particle_system_controller;
    NiBspArrayController => add_ni_bsp_array_controller;
    NiPSysEmitterCtlr => add_ni_psys_emitter_ctlr;
    BsPSysMultiTargetEmitterCtlr => add_bs_psys_multi_target_emitter_ctlr;
    crate::animation::NiPSysUpdateCtlr => add_ni_psys_update_ctlr;
    crate::animation::NiPSysResetOnLoopCtlr => add_ni_psys_reset_on_loop_ctlr;
    crate::animation::NiPSysModifierActiveCtlr => add_ni_psys_modifier_active_ctlr;
    crate::animation::NiPSysEmitterLifeSpanCtlr => add_ni_psys_emitter_life_span_ctlr;
    crate::animation::NiPSysEmitterSpeedCtlr => add_ni_psys_emitter_speed_ctlr;
    crate::animation::NiPSysEmitterInitialRadiusCtlr => add_ni_psys_emitter_initial_radius_ctlr;
    crate::animation::NiPSysEmitterDeclinationCtlr => add_ni_psys_emitter_declination_ctlr;
    crate::animation::NiPSysGravityStrengthCtlr => add_ni_psys_gravity_strength_ctlr;
    crate::animation::NiPSysEmitterDeclinationVarCtlr => add_ni_psys_emitter_declination_var_ctlr;
    crate::animation::NiPSysFieldMagnitudeCtlr => add_ni_psys_field_magnitude_ctlr;
    crate::animation::NiPSysFieldAttenuationCtlr => add_ni_psys_field_attenuation_ctlr;
    crate::animation::NiPSysFieldMaxDistanceCtlr => add_ni_psys_field_max_distance_ctlr;
    crate::animation::NiPSysAirFieldAirFrictionCtlr => add_ni_psys_air_field_air_friction_ctlr;
    crate::animation::NiPSysAirFieldInheritVelocityCtlr => add_ni_psys_air_field_inherit_velocity_ctlr;
    crate::animation::NiPSysAirFieldSpreadCtlr => add_ni_psys_air_field_spread_ctlr;
    crate::animation::NiPSysInitialRotSpeedCtlr => add_ni_psys_initial_rot_speed_ctlr;
    crate::animation::NiPSysInitialRotSpeedVarCtlr => add_ni_psys_initial_rot_speed_var_ctlr;
    crate::animation::NiPSysInitialRotAngleCtlr => add_ni_psys_initial_rot_angle_ctlr;
    crate::animation::NiPSysInitialRotAngleVarCtlr => add_ni_psys_initial_rot_angle_var_ctlr;
    crate::animation::NiPSysEmitterPlanarAngleCtlr => add_ni_psys_emitter_planar_angle_ctlr;
    crate::animation::NiPSysEmitterPlanarAngleVarCtlr => add_ni_psys_emitter_planar_angle_var_ctlr;
    crate::animation::NiPSysRotDampeningCtlr => add_ni_psys_rot_dampening_ctlr;
}
