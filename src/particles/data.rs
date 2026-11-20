//! Particle data and per-particle lists.

use super::*;

impl NiParticlesData {
    fields! {
        /// The particles' radius. Saved up to 10.0.1.0.
        particle_radius, set_particle_radius: f32 = particles_data_particle_radius, set_particles_data_particle_radius, same, same;
        /// The number of particles alive when the file was saved.
        active_count, set_active_count: u16 = particles_data_active_count, set_particles_data_active_count, same, same;
        /// Whether the particles have [radii](Self::radii). Saved from 10.1.0.0 on.
        has_radii, set_has_radii: bool = particles_data_has_radii, set_particles_data_has_radii, same, same;
        /// Whether the particles have [sizes](Self::sizes).
        has_sizes, set_has_sizes: bool = particles_data_has_sizes, set_particles_data_has_sizes, same, same;
        /// Whether the particles have [rotations](Self::rotations). Saved from 10.0.1.0 on.
        has_rotations, set_has_rotations: bool = particles_data_has_rotations, set_particles_data_has_rotations, same, same;
        /// Whether the particles have [rotation angles](Self::rotation_angles). Saved from 20.0.0.4 on.
        has_rotation_angles, set_has_rotation_angles: bool = particles_data_has_rotation_angles, set_particles_data_has_rotation_angles, same, same;
        /// Whether the particles have [rotation axes](Self::rotation_axes). Saved from 20.0.0.4 on.
        has_rotation_axes, set_has_rotation_axes: bool = particles_data_has_rotation_axes, set_particles_data_has_rotation_axes, same, same;
        /// Whether the particles use [subtexture offsets](Self::subtexture_offsets). Saved only in 20.2.0.7.
        has_texture_indices, set_has_texture_indices: bool = particles_data_has_texture_indices, set_particles_data_has_texture_indices, same, same;
        /// The aspect ratio of the subtexture quads. Saved only in 20.2.0.7, from Skyrim on.
        aspect_ratio, set_aspect_ratio: f32 = particles_data_aspect_ratio, set_particles_data_aspect_ratio, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        aspect_flags, set_aspect_flags: AspectFlags = particles_data_aspect_flags, set_particles_data_aspect_flags, AspectFlags::from_bits_retain, |f: AspectFlags| f.bits();
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_aspect_2, set_speed_to_aspect_aspect_2: f32 = particles_data_speed_to_aspect_aspect_2, set_particles_data_speed_to_aspect_aspect_2, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_speed_1, set_speed_to_aspect_speed_1: f32 = particles_data_speed_to_aspect_speed_1, set_particles_data_speed_to_aspect_speed_1, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_speed_2, set_speed_to_aspect_speed_2: f32 = particles_data_speed_to_aspect_speed_2, set_particles_data_speed_to_aspect_speed_2, same, same;
    }
}

impl NiPSysData {
    fields! {
        /// Whether the particles have [rotation speeds](Self::rotation_speeds). Saved from 20.0.0.2 on.
        has_rotation_speeds, set_has_rotation_speeds: bool = psys_data_has_rotation_speeds, set_psys_data_has_rotation_speeds, same, same;
        /// Saved except in 20.2.0.7.
        added_particle_count, set_added_particle_count: u16 = psys_data_added_particle_count, set_psys_data_added_particle_count, same, same;
        /// Saved except in 20.2.0.7.
        added_particles_base, set_added_particles_base: u16 = psys_data_added_particles_base, set_psys_data_added_particles_base, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_vector, set_unknown_vector: Vec3 = psys_data_unknown_vector, set_psys_data_unknown_vector, vec3, vec3_to_ffi;
    }
}

impl NiMeshPSysData {
    fields! {
        /// The default size of each generation's pool of meshes.
        default_pool_size, set_default_pool_size: u32 = mesh_psys_data_default_pool_size, set_mesh_psys_data_default_pool_size, same, same;
        /// Whether to fill the pools on load.
        fill_pools_on_load, set_fill_pools_on_load: bool = mesh_psys_data_fill_pools_on_load, set_mesh_psys_data_fill_pools_on_load, same, same;
    }
}

impl BsStripPSysData {
    fields! {
        /// The most points a strip has.
        max_point_count, set_max_point_count: u16 = strip_psys_data_max_point_count, set_strip_psys_data_max_point_count, same, same;
        start_cap_size, set_start_cap_size: f32 = strip_psys_data_start_cap_size, set_strip_psys_data_start_cap_size, f32::from_bits, f32::to_bits;
        end_cap_size, set_end_cap_size: f32 = strip_psys_data_end_cap_size, set_strip_psys_data_end_cap_size, f32::from_bits, f32::to_bits;
        do_z_prepass, set_do_z_prepass: bool = strip_psys_data_do_z_prepass, set_strip_psys_data_do_z_prepass, same, same;
    }
}

/// Morrowind rotations, saved up to 4.2.2.0. Later files use
/// [`NiParticlesData::rotations`].
impl NiRotatingParticlesData {
    fields! {
        /// Whether the particles have [rotations](Self::rotations_2).
        has_rotations_2, set_has_rotations_2: bool = rotating_particles_data_has_rotations_2, set_rotating_particles_data_has_rotations_2, same, same;
    }

    /// The particles' rotations as quaternions, `[x, y, z, w]`.
    pub fn rotations_2(&self) -> Vec<[f32; 4]> {
        ffi::rotating_particles_data_rotations_2(&self.0)
            .as_chunks::<4>()
            .0
            .to_vec()
    }

    /// Replaces [`rotations_2`](Self::rotations_2). Returns `false`, changing nothing, unless
    /// there are none or one per vertex.
    pub fn set_rotations_2(&mut self, rotations: &[[f32; 4]]) -> bool {
        ffi::set_rotating_particles_data_rotations_2(self.ffi_mut(), rotations.as_flattened())
    }
}

/// Per-particle lists. Each holds nothing or one value per vertex, and files from 20.2.0.7 save
/// only whether it's there.
impl NiParticlesData {
    /// Sets the [vertex count](NiGeometryData::vertex_count), the most particles the system
    /// holds, and resizes every per-vertex list that has values to match, padding with zeros.
    pub fn set_vertex_count(&mut self, count: u16) {
        ffi::set_particles_data_vertex_count(self.ffi_mut(), count);
    }

    /// The particles' radii.
    pub fn radii(&self) -> &[f32] {
        ffi::particles_data_radii(&self.0)
    }

    /// Replaces [`radii`](Self::radii). Returns `false`, changing nothing, unless there are none
    /// or one per vertex.
    pub fn set_radii(&mut self, radii: &[f32]) -> bool {
        ffi::set_particles_data_radii(self.ffi_mut(), radii)
    }

    /// The particles' sizes.
    pub fn sizes(&self) -> &[f32] {
        ffi::particles_data_sizes(&self.0)
    }

    /// Replaces [`sizes`](Self::sizes). Returns `false`, changing nothing, unless there are none
    /// or one per vertex.
    pub fn set_sizes(&mut self, sizes: &[f32]) -> bool {
        ffi::set_particles_data_sizes(self.ffi_mut(), sizes)
    }

    /// The particles' rotations as quaternions, `[x, y, z, w]`.
    pub fn rotations(&self) -> Vec<[f32; 4]> {
        ffi::particles_data_rotations(&self.0)
            .as_chunks::<4>()
            .0
            .to_vec()
    }

    /// Replaces [`rotations`](Self::rotations). Returns `false`, changing nothing, unless there
    /// are none or one per vertex.
    pub fn set_rotations(&mut self, rotations: &[[f32; 4]]) -> bool {
        ffi::set_particles_data_rotations(self.ffi_mut(), rotations.as_flattened())
    }

    /// The particles' rotation angles, in radians.
    pub fn rotation_angles(&self) -> &[f32] {
        ffi::particles_data_rotation_angles(&self.0)
    }

    /// Replaces [`rotation_angles`](Self::rotation_angles). Returns `false`, changing nothing,
    /// unless there are none or one per vertex.
    pub fn set_rotation_angles(&mut self, angles: &[f32]) -> bool {
        ffi::set_particles_data_rotation_angles(self.ffi_mut(), angles)
    }

    /// The particles' rotation axes.
    pub fn rotation_axes(&self) -> Vec<Vec3> {
        ffi::particles_data_rotation_axes(&self.0)
            .into_iter()
            .map(Vec3::from_ffi)
            .collect()
    }

    /// Replaces [`rotation_axes`](Self::rotation_axes). Returns `false`, changing nothing, unless
    /// there are none or one per vertex.
    pub fn set_rotation_axes(&mut self, axes: &[Vec3]) -> bool {
        let axes: Vec<_> = axes.iter().map(|a| a.to_ffi()).collect();
        ffi::set_particles_data_rotation_axes(self.ffi_mut(), &axes)
    }

    /// The UV offsets of the subtextures a [`BsPSysSubTexModifier`] cycles through. Saved only in
    /// 20.2.0.7; before Skyrim, at most 255 are saved.
    pub fn subtexture_offsets(&self) -> Vec<[f32; 4]> {
        ffi::particles_data_subtexture_offsets(&self.0)
            .as_chunks::<4>()
            .0
            .to_vec()
    }

    /// Replaces [`subtexture_offsets`](Self::subtexture_offsets).
    pub fn set_subtexture_offsets(&mut self, offsets: &[[f32; 4]]) {
        ffi::set_particles_data_subtexture_offsets(self.ffi_mut(), offsets.as_flattened());
    }
}

impl NiPSysData {
    /// The particles' rotation speeds. Saved when
    /// [`has_rotation_speeds`](Self::has_rotation_speeds), except in 20.2.0.7.
    pub fn rotation_speeds(&self) -> &[f32] {
        ffi::psys_data_rotation_speeds(&self.0)
    }

    /// Replaces [`rotation_speeds`](Self::rotation_speeds). Returns `false`, changing nothing,
    /// unless there are none or one per vertex.
    pub fn set_rotation_speeds(&mut self, speeds: &[f32]) -> bool {
        ffi::set_psys_data_rotation_speeds(self.ffi_mut(), speeds)
    }

    /// Each particle's motion. Saved except in 20.2.0.7.
    pub fn particle_info(&self) -> Vec<ParticleInfo> {
        ffi::psys_data_particle_info(&self.0)
            .into_iter()
            .map(ParticleInfo::from_ffi)
            .collect()
    }

    /// Replaces [`particle_info`](Self::particle_info). Returns `false`, changing nothing, unless
    /// there is none or one per vertex.
    pub fn set_particle_info(&mut self, info: &[ParticleInfo]) -> bool {
        let info: Vec<_> = info.iter().copied().map(ParticleInfo::to_ffi).collect();
        ffi::set_psys_data_particle_info(self.ffi_mut(), &info)
    }
}

/// The pool fields are saved from 10.2.0.0 on.
impl NiMeshPSysData {
    /// The pool size of each spawn generation.
    pub fn generation_pool_sizes(&self) -> Vec<u32> {
        ffi::mesh_psys_data_generation_pool_sizes(&self.0)
    }

    /// Replaces [`generation_pool_sizes`](Self::generation_pool_sizes).
    pub fn set_generation_pool_sizes(&mut self, sizes: &[u32]) {
        ffi::set_mesh_psys_data_generation_pool_sizes(self.ffi_mut(), sizes);
    }
}
