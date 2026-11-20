//! Modifier, emitter, collider and emitter controller fields.

use super::*;

impl NiPSysModifier {
    fields! {
        /// When the modifier runs relative to the system's others: lower first.
        order, set_order: u32 = psys_modifier_order, set_psys_modifier_order, same, same;
        /// Whether the modifier runs.
        active, set_active: bool = psys_modifier_active, set_psys_modifier_active, same, same;
    }
}

impl BsPSysStripUpdateModifier {
    fields! {
        /// The time between strip updates.
        update_delta_time, set_update_delta_time: f32 = strip_update_modifier_update_delta_time, set_strip_update_modifier_update_delta_time, same, same;
    }
}

impl NiPSysSpawnModifier {
    fields! {
        /// How many generations of particles can spawn.
        spawn_generations, set_spawn_generations: u16 = spawn_modifier_spawn_generations, set_spawn_modifier_spawn_generations, same, same;
        /// The fraction of particles that spawn, from 0 to 1.
        percent_spawned, set_percent_spawned: f32 = spawn_modifier_percent_spawned, set_spawn_modifier_percent_spawned, same, same;
        /// The fewest particles each one spawns.
        min_spawned, set_min_spawned: u16 = spawn_modifier_min_spawned, set_spawn_modifier_min_spawned, same, same;
        /// The most particles each one spawns.
        max_spawned, set_max_spawned: u16 = spawn_modifier_max_spawned, set_spawn_modifier_max_spawned, same, same;
        speed_variation, set_speed_variation: f32 = spawn_modifier_speed_variation, set_spawn_modifier_speed_variation, same, same;
        direction_variation, set_direction_variation: f32 = spawn_modifier_direction_variation, set_spawn_modifier_direction_variation, same, same;
        life_span, set_life_span: f32 = spawn_modifier_life_span, set_spawn_modifier_life_span, same, same;
        life_span_variation, set_life_span_variation: f32 = spawn_modifier_life_span_variation, set_spawn_modifier_life_span_variation, same, same;
    }
}

impl NiPSysAgeDeathModifier {
    fields! {
        /// Whether particles spawn new ones when they die, through the [spawn modifier](crate::handle::View::spawn_modifier).
        spawn_on_death, set_spawn_on_death: bool = age_death_modifier_spawn_on_death, set_age_death_modifier_spawn_on_death, same, same;
    }
}

impl BsPSysLodModifier {
    fields! {
        lod_begin_distance, set_lod_begin_distance: f32 = lod_modifier_lod_begin_distance, set_lod_modifier_lod_begin_distance, same, same;
        lod_end_distance, set_lod_end_distance: f32 = lod_modifier_lod_end_distance, set_lod_modifier_lod_end_distance, same, same;
        end_emit_scale, set_end_emit_scale: f32 = lod_modifier_end_emit_scale, set_lod_modifier_end_emit_scale, same, same;
        end_size, set_end_size: f32 = lod_modifier_end_size, set_lod_modifier_end_size, same, same;
    }
}

impl BsPSysSimpleColorModifier {
    fields! {
        /// The fraction of the life span spent fading in.
        fade_in_percent, set_fade_in_percent: f32 = simple_color_modifier_fade_in_percent, set_simple_color_modifier_fade_in_percent, same, same;
        /// The fraction of the life span spent fading out.
        fade_out_percent, set_fade_out_percent: f32 = simple_color_modifier_fade_out_percent, set_simple_color_modifier_fade_out_percent, same, same;
        color_1_end_percent, set_color_1_end_percent: f32 = simple_color_modifier_color_1_end_percent, set_simple_color_modifier_color_1_end_percent, same, same;
        color_2_start_percent, set_color_2_start_percent: f32 = simple_color_modifier_color_2_start_percent, set_simple_color_modifier_color_2_start_percent, same, same;
        color_2_end_percent, set_color_2_end_percent: f32 = simple_color_modifier_color_2_end_percent, set_simple_color_modifier_color_2_end_percent, same, same;
        color_3_start_percent, set_color_3_start_percent: f32 = simple_color_modifier_color_3_start_percent, set_simple_color_modifier_color_3_start_percent, same, same;
        /// RGBA.
        color_1, set_color_1: [f32; 4] = simple_color_modifier_color_1, set_simple_color_modifier_color_1, same, same;
        /// RGBA.
        color_2, set_color_2: [f32; 4] = simple_color_modifier_color_2, set_simple_color_modifier_color_2, same, same;
        /// RGBA.
        color_3, set_color_3: [f32; 4] = simple_color_modifier_color_3, set_simple_color_modifier_color_3, same, same;
    }
}

impl NiPSysRotationModifier {
    fields! {
        /// The rotation speed of new particles, in radians per second.
        initial_speed, set_initial_speed: f32 = rotation_modifier_initial_speed, set_rotation_modifier_initial_speed, same, same;
        initial_speed_variation, set_initial_speed_variation: f32 = rotation_modifier_initial_speed_variation, set_rotation_modifier_initial_speed_variation, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_vector, set_unknown_vector: [f32; 4] = rotation_modifier_unknown_vector, set_rotation_modifier_unknown_vector, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_byte, set_unknown_byte: u8 = rotation_modifier_unknown_byte, set_rotation_modifier_unknown_byte, same, same;
        /// The rotation angle of new particles, in radians.
        initial_angle, set_initial_angle: f32 = rotation_modifier_initial_angle, set_rotation_modifier_initial_angle, same, same;
        initial_angle_variation, set_initial_angle_variation: f32 = rotation_modifier_initial_angle_variation, set_rotation_modifier_initial_angle_variation, same, same;
        /// Whether to rotate new particles either way at random.
        random_speed_sign, set_random_speed_sign: bool = rotation_modifier_random_speed_sign, set_rotation_modifier_random_speed_sign, same, same;
        /// Whether to pick each new particle's axis at random instead of using [`initial_axis`](Self::initial_axis).
        random_initial_axis, set_random_initial_axis: bool = rotation_modifier_random_initial_axis, set_rotation_modifier_random_initial_axis, same, same;
        initial_axis, set_initial_axis: Vec3 = rotation_modifier_initial_axis, set_rotation_modifier_initial_axis, vec3, vec3_to_ffi;
    }
}

impl NiPSysGravityModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        gravity_axis, set_gravity_axis: Vec3 = gravity_modifier_gravity_axis, set_gravity_modifier_gravity_axis, vec3, vec3_to_ffi;
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = gravity_modifier_decay, set_gravity_modifier_decay, same, same;
        strength, set_strength: f32 = gravity_modifier_strength, set_gravity_modifier_strength, same, same;
        turbulence, set_turbulence: f32 = gravity_modifier_turbulence, set_gravity_modifier_turbulence, same, same;
        turbulence_scale, set_turbulence_scale: f32 = gravity_modifier_turbulence_scale, set_gravity_modifier_turbulence_scale, same, same;
        /// Saved from Skyrim on (stream 17).
        world_aligned, set_world_aligned: bool = gravity_modifier_world_aligned, set_gravity_modifier_world_aligned, same, same;
    }

    enum_field! {
        /// The shape of the force, or `None` for a value it doesn't list.
        force_type, set_force_type: ForceType = gravity_modifier_force_type, set_gravity_modifier_force_type
    }
}

impl NiPSysBoundUpdateModifier {
    fields! {
        /// How many updates to skip between bound updates.
        update_skip, set_update_skip: u16 = bound_update_modifier_update_skip, set_bound_update_modifier_update_skip, same, same;
    }
}

impl NiPSysDragModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        drag_axis, set_drag_axis: Vec3 = drag_modifier_drag_axis, set_drag_modifier_drag_axis, vec3, vec3_to_ffi;
        /// How much to slow particles down.
        percentage, set_percentage: f32 = drag_modifier_percentage, set_drag_modifier_percentage, same, same;
        /// The distance up to which particles are fully affected.
        range, set_range: f32 = drag_modifier_range, set_drag_modifier_range, same, same;
        /// The distance at which particles are no longer affected.
        range_falloff, set_range_falloff: f32 = drag_modifier_range_falloff, set_drag_modifier_range_falloff, same, same;
    }
}

impl BsPSysInheritVelocityModifier {
    fields! {
        change_to_inherit, set_change_to_inherit: f32 = inherit_velocity_modifier_change_to_inherit, set_inherit_velocity_modifier_change_to_inherit, same, same;
        velocity_multiplier, set_velocity_multiplier: f32 = inherit_velocity_modifier_velocity_multiplier, set_inherit_velocity_modifier_velocity_multiplier, same, same;
        velocity_variation, set_velocity_variation: f32 = inherit_velocity_modifier_velocity_variation, set_inherit_velocity_modifier_velocity_variation, same, same;
    }
}

impl BsPSysSubTexModifier {
    fields! {
        start_frame, set_start_frame: f32 = sub_tex_modifier_start_frame, set_sub_tex_modifier_start_frame, same, same;
        start_frame_variation, set_start_frame_variation: f32 = sub_tex_modifier_start_frame_variation, set_sub_tex_modifier_start_frame_variation, same, same;
        end_frame, set_end_frame: f32 = sub_tex_modifier_end_frame, set_sub_tex_modifier_end_frame, same, same;
        loop_start_frame, set_loop_start_frame: f32 = sub_tex_modifier_loop_start_frame, set_sub_tex_modifier_loop_start_frame, same, same;
        loop_start_frame_variation, set_loop_start_frame_variation: f32 = sub_tex_modifier_loop_start_frame_variation, set_sub_tex_modifier_loop_start_frame_variation, same, same;
        frame_count, set_frame_count: f32 = sub_tex_modifier_frame_count, set_sub_tex_modifier_frame_count, same, same;
        frame_count_variation, set_frame_count_variation: f32 = sub_tex_modifier_frame_count_variation, set_sub_tex_modifier_frame_count_variation, same, same;
    }
}

impl NiPSysBombModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        bomb_axis, set_bomb_axis: Vec3 = bomb_modifier_bomb_axis, set_bomb_modifier_bomb_axis, vec3, vec3_to_ffi;
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = bomb_modifier_decay, set_bomb_modifier_decay, same, same;
        /// The change in velocity.
        delta_v, set_delta_v: f32 = bomb_modifier_delta_v, set_bomb_modifier_delta_v, same, same;
    }

    enum_field! {
        /// How the force falls off, or `None` for a value it doesn't list.
        decay_type, set_decay_type: DecayType = bomb_modifier_decay_type, set_bomb_modifier_decay_type
    }

    enum_field! {
        /// The shape of the force, or `None` for a value it doesn't list.
        symmetry_type, set_symmetry_type: SymmetryType = bomb_modifier_symmetry_type, set_bomb_modifier_symmetry_type
    }
}

impl NiPSysGrowFadeModifier {
    fields! {
        /// How long new particles take to grow to full size.
        grow_time, set_grow_time: f32 = grow_fade_modifier_grow_time, set_grow_fade_modifier_grow_time, same, same;
        /// The spawn generation that grows.
        grow_generation, set_grow_generation: u16 = grow_fade_modifier_grow_generation, set_grow_fade_modifier_grow_generation, same, same;
        /// How long dying particles take to shrink away.
        fade_time, set_fade_time: f32 = grow_fade_modifier_fade_time, set_grow_fade_modifier_fade_time, same, same;
        /// The spawn generation that fades.
        fade_generation, set_fade_generation: u16 = grow_fade_modifier_fade_generation, set_grow_fade_modifier_fade_generation, same, same;
        /// Saved from Fallout 3 on (stream 34).
        base_scale, set_base_scale: f32 = grow_fade_modifier_base_scale, set_grow_fade_modifier_base_scale, same, same;
    }
}

impl NiPSysFieldModifier {
    fields! {
        /// The strength of the field.
        magnitude, set_magnitude: f32 = field_modifier_magnitude, set_field_modifier_magnitude, same, same;
        /// How fast the field falls off with distance.
        attenuation, set_attenuation: f32 = field_modifier_attenuation, set_field_modifier_attenuation, same, same;
        /// Whether the field ends at [`max_distance`](Self::max_distance).
        use_max_distance, set_use_max_distance: bool = field_modifier_use_max_distance, set_field_modifier_use_max_distance, same, same;
        max_distance, set_max_distance: f32 = field_modifier_max_distance, set_field_modifier_max_distance, same, same;
    }
}

impl NiPSysVortexFieldModifier {
    fields! {
        /// The vortex's axis.
        direction, set_direction: Vec3 = vortex_field_modifier_direction, set_vortex_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysGravityFieldModifier {
    fields! {
        /// The direction of the force.
        direction, set_direction: Vec3 = gravity_field_modifier_direction, set_gravity_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysDragFieldModifier {
    fields! {
        /// Whether drag applies only along [`direction`](Self::direction).
        use_direction, set_use_direction: bool = drag_field_modifier_use_direction, set_drag_field_modifier_use_direction, same, same;
        direction, set_direction: Vec3 = drag_field_modifier_direction, set_drag_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysTurbulenceFieldModifier {
    fields! {
        /// How often the turbulence changes.
        frequency, set_frequency: f32 = turbulence_field_modifier_frequency, set_turbulence_field_modifier_frequency, same, same;
    }
}

impl NiPSysAirFieldModifier {
    fields! {
        /// The direction of the air flow.
        direction, set_direction: Vec3 = air_field_modifier_direction, set_air_field_modifier_direction, vec3, vec3_to_ffi;
        air_friction, set_air_friction: f32 = air_field_modifier_air_friction, set_air_field_modifier_air_friction, same, same;
        inherit_velocity, set_inherit_velocity: f32 = air_field_modifier_inherit_velocity, set_air_field_modifier_inherit_velocity, same, same;
        inherit_rotation, set_inherit_rotation: bool = air_field_modifier_inherit_rotation, set_air_field_modifier_inherit_rotation, same, same;
        component_only, set_component_only: bool = air_field_modifier_component_only, set_air_field_modifier_component_only, same, same;
        enable_spread, set_enable_spread: bool = air_field_modifier_enable_spread, set_air_field_modifier_enable_spread, same, same;
        spread, set_spread: f32 = air_field_modifier_spread, set_air_field_modifier_spread, same, same;
    }
}

impl NiPSysRadialFieldModifier {
    fields! {
        /// How the force falls off with distance: 0 for not at all.
        radial_type, set_radial_type: f32 = radial_field_modifier_radial_type, set_radial_field_modifier_radial_type, f32::from_bits, f32::to_bits;
    }
}

impl BsWindModifier {
    fields! {
        strength, set_strength: f32 = wind_modifier_strength, set_wind_modifier_strength, same, same;
    }
}

impl BsPSysRecycleBoundModifier {
    fields! {
        bound_offset, set_bound_offset: Vec3 = recycle_bound_modifier_bound_offset, set_recycle_bound_modifier_bound_offset, vec3, vec3_to_ffi;
        bound_extent, set_bound_extent: Vec3 = recycle_bound_modifier_bound_extent, set_recycle_bound_modifier_bound_extent, vec3, vec3_to_ffi;
    }
}

impl BsParentVelocityModifier {
    fields! {
        damping, set_damping: f32 = parent_velocity_modifier_damping, set_parent_velocity_modifier_damping, same, same;
    }
}

impl NiPSysEmitter {
    fields! {
        /// The speed of new particles.
        speed, set_speed: f32 = psys_emitter_speed, set_psys_emitter_speed, same, same;
        speed_variation, set_speed_variation: f32 = psys_emitter_speed_variation, set_psys_emitter_speed_variation, same, same;
        /// The vertical direction of new particles, in radians: 0 is up, pi down.
        declination, set_declination: f32 = psys_emitter_declination, set_psys_emitter_declination, same, same;
        declination_variation, set_declination_variation: f32 = psys_emitter_declination_variation, set_psys_emitter_declination_variation, same, same;
        /// The horizontal direction of new particles, in radians.
        planar_angle, set_planar_angle: f32 = psys_emitter_planar_angle, set_psys_emitter_planar_angle, same, same;
        planar_angle_variation, set_planar_angle_variation: f32 = psys_emitter_planar_angle_variation, set_psys_emitter_planar_angle_variation, same, same;
        /// The color of new particles, RGBA.
        initial_color, set_initial_color: [f32; 4] = psys_emitter_initial_color, set_psys_emitter_initial_color, same, same;
        /// The radius of new particles.
        initial_radius, set_initial_radius: f32 = psys_emitter_initial_radius, set_psys_emitter_initial_radius, same, same;
        radius_variation, set_radius_variation: f32 = psys_emitter_radius_variation, set_psys_emitter_radius_variation, same, same;
        /// How long new particles live, in seconds.
        life_span, set_life_span: f32 = psys_emitter_life_span, set_psys_emitter_life_span, same, same;
        life_span_variation, set_life_span_variation: f32 = psys_emitter_life_span_variation, set_psys_emitter_life_span_variation, same, same;
    }
}

impl NiPSysSphereEmitter {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = sphere_emitter_radius, set_sphere_emitter_radius, same, same;
    }
}

impl NiPSysCylinderEmitter {
    fields! {
        /// The cylinder's radius.
        radius, set_radius: f32 = cylinder_emitter_radius, set_cylinder_emitter_radius, same, same;
        /// The cylinder's height.
        height, set_height: f32 = cylinder_emitter_height, set_cylinder_emitter_height, same, same;
    }
}

impl NiPSysBoxEmitter {
    fields! {
        /// The box's size along x.
        width, set_width: f32 = box_emitter_width, set_box_emitter_width, same, same;
        /// The box's size along y.
        height, set_height: f32 = box_emitter_height, set_box_emitter_height, same, same;
        /// The box's size along z.
        depth, set_depth: f32 = box_emitter_depth, set_box_emitter_depth, same, same;
    }
}

impl NiPSysMeshEmitter {
    fields! {
        /// The direction of new particles with [`VelocityType::UseDirection`].
        emission_axis, set_emission_axis: Vec3 = mesh_emitter_emission_axis, set_mesh_emitter_emission_axis, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// How new particles' velocity is chosen, or `None` for a value it doesn't list.
        velocity_type, set_velocity_type: VelocityType = mesh_emitter_velocity_type, set_mesh_emitter_velocity_type
    }

    enum_field! {
        /// Where on the meshes particles start, or `None` for a value it doesn't list.
        emission_type, set_emission_type: EmitFrom = mesh_emitter_emission_type, set_mesh_emitter_emission_type
    }
}

impl NiPSysCollider {
    fields! {
        /// How much speed particles keep when they bounce off, from 0 to 1.
        bounce, set_bounce: f32 = psys_collider_bounce, set_psys_collider_bounce, same, same;
        /// Whether particles spawn new ones when they hit, through the [spawn modifier](crate::handle::View::spawn_modifier).
        spawn_on_collide, set_spawn_on_collide: bool = psys_collider_spawn_on_collide, set_psys_collider_spawn_on_collide, same, same;
        /// Whether particles die when they hit.
        die_on_collide, set_die_on_collide: bool = psys_collider_die_on_collide, set_psys_collider_die_on_collide, same, same;
    }
}

impl NiPSysSphericalCollider {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = spherical_collider_radius, set_spherical_collider_radius, same, same;
    }
}

impl NiPSysPlanarCollider {
    fields! {
        /// The plane's size along [`x_axis`](Self::x_axis).
        width, set_width: f32 = planar_collider_width, set_planar_collider_width, same, same;
        /// The plane's size along [`y_axis`](Self::y_axis).
        height, set_height: f32 = planar_collider_height, set_planar_collider_height, same, same;
        x_axis, set_x_axis: Vec3 = planar_collider_x_axis, set_planar_collider_x_axis, vec3, vec3_to_ffi;
        y_axis, set_y_axis: Vec3 = planar_collider_y_axis, set_planar_collider_y_axis, vec3, vec3_to_ffi;
    }
}

impl BsPSysMultiTargetEmitterCtlr {
    fields! {
        /// The most emitters the controller drives.
        max_emitters, set_max_emitters: u16 = multi_target_emitter_ctlr_max_emitters, set_multi_target_emitter_ctlr_max_emitters, same, same;
    }
}

impl NiPSysEmitterCtlrData {
    /// The birth rate keys.
    pub fn keys(&self) -> KeyGroup<f32> {
        group_from_ffi(ffi::emitter_ctlr_data_keys(&self.0))
    }

    /// Replaces [`keys`](Self::keys). Returns `false`, changing nothing, if the group has keys but
    /// no interpolation.
    pub fn set_keys(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_emitter_ctlr_data_keys(self.ffi_mut(), &group_to_ffi(keys))
    }

    /// When the emitter is on. Only each key's time and value are saved.
    pub fn visibility_keys(&self) -> Vec<Key<bool>> {
        ffi::emitter_ctlr_data_visibility_keys(&self.0).iter().map(key_from_ffi).collect()
    }

    /// Replaces [`visibility_keys`](Self::visibility_keys).
    pub fn set_visibility_keys(self: Pin<&mut Self>, keys: &[Key<bool>]) {
        let keys: Vec<_> = keys.iter().map(key_to_ffi).collect();
        ffi::set_emitter_ctlr_data_visibility_keys(self.ffi_mut(), &keys);
    }
}

impl NiPSysModifier {
    /// The modifier's name, which controllers use to find it.
    pub fn name(&self) -> &BStr {
        ffi::psys_modifier_name(&self.0).as_bstr()
    }

    /// Sets [`name`](Self::name).
    pub fn set_name(self: Pin<&mut Self>, name: impl AsRef<[u8]>) {
        ffi::set_psys_modifier_name(self.ffi_mut(), name.as_ref());
    }
}

impl BsPSysSimpleColorModifier {
    /// Values of unknown meaning, saved from Fallout 76 on.
    pub fn unknown_shorts(&self) -> [u16; 26] {
        ffi::simple_color_modifier_unknown_shorts(&self.0)
    }

    /// Sets [`unknown_shorts`](Self::unknown_shorts).
    pub fn set_unknown_shorts(self: Pin<&mut Self>, values: [u16; 26]) {
        ffi::set_simple_color_modifier_unknown_shorts(self.ffi_mut(), values);
    }
}

impl BsPSysScaleModifier {
    /// The scales over the particles' life.
    pub fn scales(&self) -> Vec<f32> {
        ffi::scale_modifier_scales(&self.0)
    }

    /// Replaces [`scales`](Self::scales).
    pub fn set_scales(self: Pin<&mut Self>, scales: &[f32]) {
        ffi::set_scale_modifier_scales(self.ffi_mut(), scales);
    }
}
