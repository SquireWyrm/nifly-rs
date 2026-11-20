//! Old (Morrowind and Oblivion) particles.

use super::*;

impl NiGravity {
    fields! {
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = gravity_decay, set_gravity_decay, same, same;
        /// The force's strength.
        force, set_force: f32 = gravity_force, set_gravity_force, same, same;
        /// The point particles are pulled toward, in the system's space.
        position, set_position: Vec3 = gravity_position, set_gravity_position, vec3, vec3_to_ffi;
        /// The direction particles are pulled in.
        direction, set_direction: Vec3 = gravity_direction, set_gravity_direction, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// The kind of force, or `None` for a value it doesn't list.
        field_type, set_field_type: FieldType = gravity_field_type, set_gravity_field_type
    }
}

impl NiParticleGrowFade {
    fields! {
        /// The time from a particle's birth during which it grows.
        grow, set_grow: f32 = grow_fade_grow, set_grow_fade_grow, same, same;
        /// The time before a particle's death during which it shrinks.
        fade, set_fade: f32 = grow_fade_fade, set_grow_fade_fade, same, same;
    }
}

impl NiParticleRotation {
    fields! {
        /// Whether each particle starts with a random axis instead of [`initial_axis`](Self::initial_axis).
        random_initial_axis, set_random_initial_axis: bool = particle_rotation_random_initial_axis, set_particle_rotation_random_initial_axis, same, same;
        initial_axis, set_initial_axis: Vec3 = particle_rotation_initial_axis, set_particle_rotation_initial_axis, vec3, vec3_to_ffi;
        /// The speed of rotation, in radians per second.
        rotation_speed, set_rotation_speed: f32 = particle_rotation_rotation_speed, set_particle_rotation_rotation_speed, same, same;
    }
}

impl NiParticleBomb {
    fields! {
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = particle_bomb_decay, set_particle_bomb_decay, same, same;
        /// How long the bomb pushes.
        duration, set_duration: f32 = particle_bomb_duration, set_particle_bomb_duration, same, same;
        /// The change in speed the bomb gives.
        delta_v, set_delta_v: f32 = particle_bomb_delta_v, set_particle_bomb_delta_v, same, same;
        /// When the bomb goes off.
        start, set_start: f32 = particle_bomb_start, set_particle_bomb_start, same, same;
        position, set_position: Vec3 = particle_bomb_position, set_particle_bomb_position, vec3, vec3_to_ffi;
        direction, set_direction: Vec3 = particle_bomb_direction, set_particle_bomb_direction, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// How the force falls off, or `None` for a value it doesn't list.
        decay_type, set_decay_type: DecayType = particle_bomb_decay_type, set_particle_bomb_decay_type
    }

    enum_field! {
        /// The force's shape, or `None` for a value it doesn't list. Saved from 4.1.0.12 on.
        symmetry_type, set_symmetry_type: SymmetryType = particle_bomb_symmetry_type, set_particle_bomb_symmetry_type
    }
}

impl NiParticleCollider {
    fields! {
        /// How much speed particles keep when they bounce off, from 0 to 1.
        bounce, set_bounce: f32 = particle_collider_bounce, set_particle_collider_bounce, same, same;
        /// Whether particles spawn new ones when they hit. Saved from 4.2.0.2 on.
        spawn_on_collide, set_spawn_on_collide: bool = particle_collider_spawn_on_collide, set_particle_collider_spawn_on_collide, same, same;
        /// Whether particles die when they hit. Saved from 4.2.0.2 on.
        die_on_collide, set_die_on_collide: bool = particle_collider_die_on_collide, set_particle_collider_die_on_collide, same, same;
    }
}

impl NiPlanarCollider {
    fields! {
        /// The plane's size along [`x_axis`](Self::x_axis).
        width, set_width: f32 = old_planar_collider_width, set_old_planar_collider_width, same, same;
        /// The plane's size along [`y_axis`](Self::y_axis).
        height, set_height: f32 = old_planar_collider_height, set_old_planar_collider_height, same, same;
        /// The plane's center.
        position, set_position: Vec3 = old_planar_collider_position, set_old_planar_collider_position, vec3, vec3_to_ffi;
        x_axis, set_x_axis: Vec3 = old_planar_collider_x_axis, set_old_planar_collider_x_axis, vec3, vec3_to_ffi;
        y_axis, set_y_axis: Vec3 = old_planar_collider_y_axis, set_old_planar_collider_y_axis, vec3, vec3_to_ffi;
    }
}

impl NiSphericalCollider {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = old_spherical_collider_radius, set_old_spherical_collider_radius, same, same;
        /// The sphere's center.
        position, set_position: Vec3 = old_spherical_collider_position, set_old_spherical_collider_position, vec3, vec3_to_ffi;
    }
}

impl NiParticleSystemController {
    fields! {
        /// The speed of new particles.
        speed, set_speed: f32 = particle_system_ctlr_speed, set_particle_system_ctlr_speed, same, same;
        speed_variation, set_speed_variation: f32 = particle_system_ctlr_speed_variation, set_particle_system_ctlr_speed_variation, same, same;
        /// The angle of new particles' direction from the emitter's up axis, in radians.
        declination, set_declination: f32 = particle_system_ctlr_declination, set_particle_system_ctlr_declination, same, same;
        declination_variation, set_declination_variation: f32 = particle_system_ctlr_declination_variation, set_particle_system_ctlr_declination_variation, same, same;
        /// The angle of new particles' direction around the emitter's up axis, in radians.
        planar_angle, set_planar_angle: f32 = particle_system_ctlr_planar_angle, set_particle_system_ctlr_planar_angle, same, same;
        planar_angle_variation, set_planar_angle_variation: f32 = particle_system_ctlr_planar_angle_variation, set_particle_system_ctlr_planar_angle_variation, same, same;
        /// The normal of new particles.
        initial_normal, set_initial_normal: Vec3 = particle_system_ctlr_initial_normal, set_particle_system_ctlr_initial_normal, vec3, vec3_to_ffi;
        /// The color of new particles, RGBA.
        initial_color, set_initial_color: [f32; 4] = particle_system_ctlr_initial_color, set_particle_system_ctlr_initial_color, same, same;
        /// The size of new particles.
        initial_size, set_initial_size: f32 = particle_system_ctlr_initial_size, set_particle_system_ctlr_initial_size, same, same;
        /// When emitting starts.
        emit_start_time, set_emit_start_time: f32 = particle_system_ctlr_emit_start_time, set_particle_system_ctlr_emit_start_time, same, same;
        /// When emitting stops.
        emit_stop_time, set_emit_stop_time: f32 = particle_system_ctlr_emit_stop_time, set_particle_system_ctlr_emit_stop_time, same, same;
        /// Whether the system resets when the animation loops.
        reset_particle_system, set_reset_particle_system: bool = particle_system_ctlr_reset_particle_system, set_particle_system_ctlr_reset_particle_system, same, same;
        /// New particles per second, with [`use_birth_rate`](Self::use_birth_rate).
        birth_rate, set_birth_rate: f32 = particle_system_ctlr_birth_rate, set_particle_system_ctlr_birth_rate, same, same;
        /// How long particles live.
        lifetime, set_lifetime: f32 = particle_system_ctlr_lifetime, set_particle_system_ctlr_lifetime, same, same;
        lifetime_variation, set_lifetime_variation: f32 = particle_system_ctlr_lifetime_variation, set_particle_system_ctlr_lifetime_variation, same, same;
        /// Whether to emit at [`birth_rate`](Self::birth_rate) rather than to keep every particle alive.
        use_birth_rate, set_use_birth_rate: bool = particle_system_ctlr_use_birth_rate, set_particle_system_ctlr_use_birth_rate, same, same;
        /// Whether particles spawn new ones when they die.
        spawn_on_death, set_spawn_on_death: bool = particle_system_ctlr_spawn_on_death, set_particle_system_ctlr_spawn_on_death, same, same;
        /// The size of the box particles start in, around the [emitter](crate::handle::View::emitter).
        emitter_dimensions, set_emitter_dimensions: Vec3 = particle_system_ctlr_emitter_dimensions, set_particle_system_ctlr_emitter_dimensions, vec3, vec3_to_ffi;
        /// How many generations of particles can spawn.
        spawn_generations, set_spawn_generations: u16 = particle_system_ctlr_spawn_generations, set_particle_system_ctlr_spawn_generations, same, same;
        /// The fraction of particles that spawn, from 0 to 1.
        percent_spawned, set_percent_spawned: f32 = particle_system_ctlr_percent_spawned, set_particle_system_ctlr_percent_spawned, same, same;
        /// How many particles each one spawns.
        spawn_multiplier, set_spawn_multiplier: u16 = particle_system_ctlr_spawn_multiplier, set_particle_system_ctlr_spawn_multiplier, same, same;
        /// The speed variation of spawned particles.
        spawn_speed_chaos, set_spawn_speed_chaos: f32 = particle_system_ctlr_spawn_speed_chaos, set_particle_system_ctlr_spawn_speed_chaos, same, same;
        /// The direction variation of spawned particles.
        spawn_direction_chaos, set_spawn_direction_chaos: f32 = particle_system_ctlr_spawn_direction_chaos, set_particle_system_ctlr_spawn_direction_chaos, same, same;
        /// The number of [particles](Self::particles) alive when the file was saved.
        active_count, set_active_count: u16 = particle_system_ctlr_active_count, set_particle_system_ctlr_active_count, same, same;
        static_target_bound, set_static_target_bound: bool = particle_system_ctlr_static_target_bound, set_particle_system_ctlr_static_target_bound, same, same;
    }
}

impl NiPlanarCollider {
    /// The plane particles bounce off.
    pub fn plane(&self) -> Plane {
        Plane {
            normal: Vec3::from_ffi(ffi::old_planar_collider_plane_normal(&self.0)),
            constant: ffi::old_planar_collider_plane_constant(&self.0),
        }
    }

    /// Sets [`plane`](Self::plane).
    pub fn set_plane(mut self: Pin<&mut Self>, plane: Plane) {
        ffi::set_old_planar_collider_plane_normal(self.as_mut().ffi_mut(), plane.normal.to_ffi());
        ffi::set_old_planar_collider_plane_constant(self.ffi_mut(), plane.constant);
    }
}

impl NiParticleSystemController {
    /// The particles' records: the most particles alive at once, and each one's motion.
    pub fn particles(&self) -> Vec<ParticleInfo> {
        ffi::particle_system_ctlr_particles(&self.0).into_iter().map(ParticleInfo::from_ffi).collect()
    }

    /// Replaces [`particles`](Self::particles). Returns `false`, changing nothing, if there are
    /// more than 65535.
    pub fn set_particles(self: Pin<&mut Self>, particles: &[ParticleInfo]) -> bool {
        let particles: Vec<_> = particles.iter().copied().map(ParticleInfo::to_ffi).collect();
        ffi::set_particle_system_ctlr_particles(self.ffi_mut(), &particles)
    }
}

/// Old particle, modifier and controller refs.
impl<'a, T: NiType> View<'a, T> {
    /// The modifier after this one in its chain, or `None` if it's the last.
    pub fn next_modifier(&self) -> Option<View<'a, T::Next>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::NextParticleModifier)
    }

    /// The controller the modifier belongs to, or `None` if it has none.
    pub fn particle_controller(&self) -> Option<View<'a, NiParticleSystemController>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::ParticleController)
    }

    /// The particles' data, or `None` if they have none.
    pub fn particles_data<I>(&self) -> Option<View<'a, NiParticlesData>>
    where
        T: IsA<NiParticles, I>,
    {
        self.follow(RefKind::ParticlesData)
    }

    /// The object particles start around, or `None` if it has none.
    pub fn emitter<I>(&self) -> Option<View<'a, NiAvObject>>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow(RefKind::ParticleEmitter)
    }

    /// The controller's particle modifiers: the first, then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn particle_modifiers<I>(&self) -> BlockIter<'a, NiParticleModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::ParticleModifiers)
    }

    /// The controller's colliders: the first, which is an [`NiParticleCollider`], then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn particle_colliders<I>(&self) -> BlockIter<'a, NiParticleModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::ParticleColliders)
    }

    /// The controller's emitter modifiers: the first, then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn emitter_modifiers<I>(&self) -> BlockIter<'a, NiEmitterModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::EmitterModifiers)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::next_modifier`], for editing.
    pub fn next_modifier(&mut self) -> Option<Edit<'_, T::Next>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::NextParticleModifier)
    }

    /// Sets the [next modifier](View::next_modifier), or ends the chain with `None`. Returns
    /// `false`, changing nothing, unless `next` is an [`NiParticleModifier`] (an
    /// [`NiEmitterModifier`] for one) whose chain doesn't lead back to this one.
    pub fn set_next_modifier(&mut self, next: Option<Index>) -> bool
    where
        T: ChainedModifier,
    {
        self.set_ref(RefKind::NextParticleModifier, next)
    }

    /// Like [`View::particle_controller`], for editing.
    pub fn particle_controller(&mut self) -> Option<Edit<'_, NiParticleSystemController>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::ParticleController)
    }

    /// Sets the [particle controller](View::particle_controller), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `controller` is an
    /// [`NiParticleSystemController`].
    pub fn set_particle_controller(&mut self, controller: Option<Index>) -> bool
    where
        T: ChainedModifier,
    {
        self.set_ref(RefKind::ParticleController, controller)
    }

    /// Like [`View::particles_data`], for editing.
    pub fn particles_data<I>(&mut self) -> Option<Edit<'_, NiParticlesData>>
    where
        T: IsA<NiParticles, I>,
    {
        self.follow(RefKind::ParticlesData)
    }

    /// Sets the [particles' data](View::particles_data), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `data` is an [`NiParticlesData`]. Use the data type that
    /// matches the particles' type, such as [`NiAutoNormalParticlesData`] for
    /// [`NiAutoNormalParticles`].
    pub fn set_particles_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiParticles, I>,
    {
        self.set_ref(RefKind::ParticlesData, data)
    }

    /// Like [`View::emitter`], for editing.
    pub fn emitter<I>(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow(RefKind::ParticleEmitter)
    }

    /// Sets the [emitter](View::emitter), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `emitter` is an [`NiAvObject`].
    pub fn set_emitter<I>(&mut self, emitter: Option<Index>) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref(RefKind::ParticleEmitter, emitter)
    }

    /// Replaces the [particle modifiers](View::particle_modifiers), linking them in order.
    /// Returns `false`, changing nothing, unless they're distinct [`NiParticleModifier`]s.
    pub fn set_particle_modifiers<I>(&mut self, modifiers: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::ParticleModifiers, modifiers)
    }

    /// Replaces the [particle colliders](View::particle_colliders), linking them in order.
    /// Returns `false`, changing nothing, unless they're distinct [`NiParticleModifier`]s and the
    /// first is an [`NiParticleCollider`].
    pub fn set_particle_colliders<I>(&mut self, colliders: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::ParticleColliders, colliders)
    }

    /// Replaces the [emitter modifiers](View::emitter_modifiers), linking them in order. Returns
    /// `false`, changing nothing, unless they're distinct [`NiEmitterModifier`]s.
    pub fn set_emitter_modifiers<I>(&mut self, modifiers: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::EmitterModifiers, modifiers)
    }

    /// Adds a new `M` belonging to this controller to the end of a chain and returns its index:
    /// colliders to the [particle colliders](View::particle_colliders), an [`NiEmitterModifier`]
    /// to the [emitter modifiers](View::emitter_modifiers), and others to the
    /// [particle modifiers](View::particle_modifiers). A ref in the chain that isn't of the
    /// chain's type, or that repeats one, ends the chain and is replaced.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, object3d::Transform, particles::*, geometry::NiAutoNormalParticles};
    /// let mut nif = NifFile::create(NifVersion::MORROWIND);
    /// let mut root = nif.root_node_mut().unwrap();
    /// let particles = root.add_particle_system::<NiAutoNormalParticles, _>("Smoke", Transform::default());
    /// let mut particles = nif.edit::<NiAutoNormalParticles>(particles).unwrap();
    /// let controller = particles.add_controller::<NiParticleSystemController, _>();
    /// let mut controller = nif.edit::<NiParticleSystemController>(controller).unwrap();
    /// controller.set_emitter(Some(0));
    /// let gravity = controller.add_particle_modifier::<NiGravity, _>();
    /// controller.add_particle_modifier::<NiParticleGrowFade, _>();
    /// nif.block_of_mut::<NiGravity>(gravity).unwrap().set_force(-9.8);
    /// ```
    pub fn add_particle_modifier<M: NewParticleModifier, I>(&mut self) -> Index
    where
        T: IsA<NiParticleSystemController, I>,
    {
        let controller = self.index() as u32;
        index_from_ffi(M::add(self.file_mut().inner.pin_mut(), controller))
            .expect("the block is an NiParticleSystemController")
    }
}

/// The container node of an [`NiParticleMeshesData`].
impl<'a> View<'a, NiParticleMeshesData> {
    /// The node holding the particle meshes, or `None` if it has none.
    pub fn container_node(&self) -> Option<View<'a, NiNode>> {
        self.follow(RefKind::ContainerNode)
    }
}

impl Edit<'_, NiParticleMeshesData> {
    /// Like [`View::container_node`], for editing.
    pub fn container_node(&mut self) -> Option<Edit<'_, NiNode>> {
        self.follow(RefKind::ContainerNode)
    }

    /// Sets the [container node](View::container_node), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `node` is an [`NiNode`].
    pub fn set_container_node(&mut self, node: Option<Index>) -> bool {
        self.set_ref(RefKind::ContainerNode, node)
    }
}

/// An old particle modifier or an [`NiEmitterModifier`]: a block in a chain of an
/// [`NiParticleSystemController`].
///
/// This trait is sealed.
pub trait ChainedModifier: NiType {
    /// The type of the next block in the chain.
    type Next: NiType;
}

macro_rules! chained_modifier {
    ($next:ident: $($t:ident)*) => {$(impl ChainedModifier for $t { type Next = $next; })*};
}

chained_modifier! { NiParticleModifier:
    NiParticleModifier NiGravity NiParticleGrowFade NiParticleColorModifier NiParticleRotation
    NiParticleBomb NiParticleMeshModifier NiParticleCollider NiPlanarCollider NiSphericalCollider
}

chained_modifier! { NiEmitterModifier: NiEmitterModifier }

/// An old particle modifier [`Edit::add_particle_modifier`] can add.
pub trait NewParticleModifier: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, controller: u32) -> u32;
}

macro_rules! new_particle_modifier {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewParticleModifier for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, controller: u32) -> u32 {
                ffi::$add(file, controller)
            }
        }
    )*};
}

new_particle_modifier! {
    NiGravity => add_ni_gravity;
    NiParticleGrowFade => add_ni_particle_grow_fade;
    NiParticleColorModifier => add_ni_particle_color_modifier;
    NiParticleRotation => add_ni_particle_rotation;
    NiParticleBomb => add_ni_particle_bomb;
    NiParticleMeshModifier => add_ni_particle_mesh_modifier;
    NiPlanarCollider => add_ni_planar_collider;
    NiSphericalCollider => add_ni_spherical_collider;
    NiEmitterModifier => add_ni_emitter_modifier;
}
