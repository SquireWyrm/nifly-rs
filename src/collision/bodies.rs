//! World objects, rigid bodies, phantoms and actions.

use super::*;

/// Which collision layer and group an object is in. The default is layer 1, static objects in
/// every game, as in new blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HavokFilter {
    pub layer: u8,
    /// Flags in the top three bits, and a body part number in the rest.
    pub raw_flags_and_parts: u8,
    pub group: u16,
}

impl Default for HavokFilter {
    fn default() -> Self {
        Self { layer: 1, raw_flags_and_parts: 0, group: 0 }
    }
}

impl HavokFilter {
    pub(super) fn from_ffi(f: ffi::HavokFilterFfi) -> Self {
        Self { layer: f.layer, raw_flags_and_parts: f.flags_and_parts, group: f.group }
    }

    pub(super) fn to_ffi(self) -> ffi::HavokFilterFfi {
        ffi::HavokFilterFfi { layer: self.layer, flags_and_parts: self.raw_flags_and_parts, group: self.group }
    }
}

/// A Havok property record of unknown use, kept as stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CInfoProperty {
    pub data: u32,
    pub size: u32,
    pub capacity_and_raw_flags: u32,
}

impl CInfoProperty {
    pub(super) fn from_ffi(p: ffi::CInfoPropertyFfi) -> Self {
        Self { data: p.data, size: p.size, capacity_and_raw_flags: p.capacity_and_flags }
    }

    pub(super) fn to_ffi(self) -> ffi::CInfoPropertyFfi {
        ffi::CInfoPropertyFfi { data: self.data, size: self.size, capacity_and_flags: self.capacity_and_raw_flags }
    }
}

impl BhkWorldObject {
    converted_fields! {
        /// The collision layer and group.
        collision_filter, set_collision_filter: HavokFilter =
            world_object_filter, set_world_object_filter, HavokFilter::from_ffi, HavokFilter::to_ffi;
        property, set_property: CInfoProperty =
            world_object_property, set_world_object_property, CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    fields! {
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: i32 = world_object_unknown_int, set_world_object_unknown_int;
        /// How the broad phase handles the object, as a raw value.
        broad_phase_type, set_broad_phase_type: u8 =
            world_object_broad_phase_type, set_world_object_broad_phase_type;
        /// Values of unknown meaning.
        unknown_bytes, set_unknown_bytes: [u8; 3] =
            world_object_unknown_bytes, set_world_object_unknown_bytes;
    }
}

/// # Fields by version
///
/// Oblivion and Fallout 3 files have stream version 34 or lower, Skyrim's 83 or 100, and Fallout
/// 4's 130. Fields a file's version doesn't store can still be read and set, but aren't written
/// when it's saved.
///
/// Vectors are `[x, y, z, w]` in Havok units, and the rotation is a quaternion `[x, y, z, w]`.
impl BhkRigidBody {
    /// How the body responds to collisions, or `None` if the stored value isn't a
    /// [`CollisionResponse`].
    pub fn collision_response(&self) -> Option<CollisionResponse> {
        CollisionResponse::try_from(ffi::rigid_body_collision_response(&self.0)).ok()
    }

    pub fn set_collision_response(self: Pin<&mut Self>, response: CollisionResponse) {
        ffi::set_rigid_body_collision_response(self.ffi_mut(), response.into());
    }

    converted_fields! {
        /// A copy of the [world object's filter](BhkWorldObject::collision_filter).
        collision_filter_copy, set_collision_filter_copy: HavokFilter =
            rigid_body_filter_copy, set_rigid_body_filter_copy, HavokFilter::from_ffi, HavokFilter::to_ffi;
        /// The inertia tensor as three rows of four values.
        inertia, set_inertia: [[f32; 4]; 3] = rigid_body_inertia, set_rigid_body_inertia, rows, flatten;
    }

    fields! {
        /// How many simulation steps pass between contact callbacks.
        process_contact_callback_delay, set_process_contact_callback_delay: u16 =
            rigid_body_callback_delay, set_rigid_body_callback_delay;
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: u32 = rigid_body_unknown_int, set_rigid_body_unknown_int;
        /// Values of unknown meaning.
        unknown_shorts, set_unknown_shorts: [u16; 6] =
            rigid_body_unknown_shorts, set_rigid_body_unknown_shorts;
        translation, set_translation: [f32; 4] = rigid_body_translation, set_rigid_body_translation;
        rotation, set_rotation: [f32; 4] = rigid_body_rotation, set_rigid_body_rotation;
        linear_velocity, set_linear_velocity: [f32; 4] =
            rigid_body_linear_velocity, set_rigid_body_linear_velocity;
        angular_velocity, set_angular_velocity: [f32; 4] =
            rigid_body_angular_velocity, set_rigid_body_angular_velocity;
        /// The center of mass.
        center, set_center: [f32; 4] = rigid_body_center, set_rigid_body_center;
        mass, set_mass: f32 = rigid_body_mass, set_rigid_body_mass;
        linear_damping, set_linear_damping: f32 = rigid_body_linear_damping, set_rigid_body_linear_damping;
        angular_damping, set_angular_damping: f32 = rigid_body_angular_damping, set_rigid_body_angular_damping;
        /// Only saved in Skyrim (LE and SE).
        time_factor, set_time_factor: f32 = rigid_body_time_factor, set_rigid_body_time_factor;
        /// Only saved from Skyrim on.
        gravity_factor, set_gravity_factor: f32 = rigid_body_gravity_factor, set_rigid_body_gravity_factor;
        friction, set_friction: f32 = rigid_body_friction, set_rigid_body_friction;
        /// Only saved from Skyrim on.
        rolling_friction_multiplier, set_rolling_friction_multiplier: f32 =
            rigid_body_rolling_friction, set_rigid_body_rolling_friction;
        /// Bounciness.
        restitution, set_restitution: f32 = rigid_body_restitution, set_rigid_body_restitution;
        max_linear_velocity, set_max_linear_velocity: f32 =
            rigid_body_max_linear_velocity, set_rigid_body_max_linear_velocity;
        max_angular_velocity, set_max_angular_velocity: f32 =
            rigid_body_max_angular_velocity, set_rigid_body_max_angular_velocity;
        /// How deep other objects may go into the body.
        penetration_depth, set_penetration_depth: f32 =
            rigid_body_penetration_depth, set_rigid_body_penetration_depth;
        /// The motion system, as a raw value.
        motion_system, set_motion_system: u8 = rigid_body_motion_system, set_rigid_body_motion_system;
        /// The deactivator type, as a raw value.
        deactivator_type, set_deactivator_type: u8 = rigid_body_deactivator_type, set_rigid_body_deactivator_type;
        /// The solver deactivation, as a raw value.
        solver_deactivation, set_solver_deactivation: u8 =
            rigid_body_solver_deactivation, set_rigid_body_solver_deactivation;
        /// The motion quality, as a raw value.
        quality_type, set_quality_type: u8 = rigid_body_quality_type, set_rigid_body_quality_type;
        /// Only saved from Skyrim on.
        auto_remove_level, set_auto_remove_level: u8 =
            rigid_body_auto_remove_level, set_rigid_body_auto_remove_level;
        /// Only saved from Skyrim on.
        response_modifier_raw_flags, set_response_modifier_raw_flags: u8 =
            rigid_body_response_modifier_flags, set_rigid_body_response_modifier_flags;
        /// The number of shape keys in contact point properties. Only saved from Skyrim on.
        shape_keys_in_contact_points, set_shape_keys_in_contact_points: u8 =
            rigid_body_shape_keys, set_rigid_body_shape_keys;
        /// Only saved from Skyrim on.
        force_collide_onto_ppu, set_force_collide_onto_ppu: bool =
            rigid_body_force_collide_onto_ppu, set_rigid_body_force_collide_onto_ppu;
        /// The body flags. Only saved from Skyrim on; older files save
        /// [`body_raw_flags_32`](Self::body_raw_flags_32) instead.
        body_raw_flags, set_body_raw_flags: u16 = rigid_body_body_flags, set_rigid_body_body_flags;
        /// The body flags as 32 bits. Only saved before Skyrim. The two aren't converted into
        /// each other.
        body_raw_flags_32, set_body_raw_flags_32: u32 = rigid_body_body_flags_int, set_rigid_body_body_flags_int;
    }
}

/// How a [`BhkRigidBody`] responds to collisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u8)]
pub enum CollisionResponse {
    Invalid = 0,
    /// Collides and bounces off.
    SimpleContact = 1,
    /// Only reports the contact.
    Reporting = 2,
    None = 3,
}

impl BhkSimpleShapePhantom {
    converted_fields! {
        /// The phantom's transform, as four rows.
        transform, set_transform: [[f32; 4]; 4] =
            simple_phantom_transform, set_simple_phantom_transform, rows, flatten;
    }
}

impl BhkAabbPhantom {
    fields! {
        /// The box's minimum corner, `[x, y, z, w]`.
        aabb_min, set_aabb_min: [f32; 4] = aabb_phantom_min, set_aabb_phantom_min;
        /// The box's maximum corner, `[x, y, z, w]`.
        aabb_max, set_aabb_max: [f32; 4] = aabb_phantom_max, set_aabb_phantom_max;
    }
}

impl BhkLiquidAction {
    fields! {
        user_data, set_user_data: u32 = liquid_user_data, set_liquid_user_data;
        /// A value of unknown meaning.
        unknown_int_1, set_unknown_int_1: u32 = liquid_unknown_int_1, set_liquid_unknown_int_1;
        /// A value of unknown meaning.
        unknown_int_2, set_unknown_int_2: u32 = liquid_unknown_int_2, set_liquid_unknown_int_2;
        initial_stick_force, set_initial_stick_force: f32 =
            liquid_initial_stick_force, set_liquid_initial_stick_force;
        stick_strength, set_stick_strength: f32 = liquid_stick_strength, set_liquid_stick_strength;
        neighbor_distance, set_neighbor_distance: f32 =
            liquid_neighbor_distance, set_liquid_neighbor_distance;
        neighbor_strength, set_neighbor_strength: f32 =
            liquid_neighbor_strength, set_liquid_neighbor_strength;
    }
}

/// Its body is on [`View::hinged_body`].
impl BhkOrientHingedBodyAction {
    fields! {
        /// A value of unknown meaning.
        unknown_int_1, set_unknown_int_1: u32 = hinged_action_unknown_int_1, set_hinged_action_unknown_int_1;
        /// A value of unknown meaning.
        unknown_int_2, set_unknown_int_2: u32 = hinged_action_unknown_int_2, set_hinged_action_unknown_int_2;
        /// The hinge axis in the body's space, `[x, y, z, w]`.
        hinge_axis, set_hinge_axis: [f32; 4] = hinged_action_hinge_axis, set_hinged_action_hinge_axis;
        /// The forward direction in the body's space, `[x, y, z, w]`.
        forward, set_forward: [f32; 4] = hinged_action_forward, set_hinged_action_forward;
        strength, set_strength: f32 = hinged_action_strength, set_hinged_action_strength;
        damping, set_damping: f32 = hinged_action_damping, set_hinged_action_damping;
    }
}

/// A block with a collision [shape](View::shape): a [`BhkWorldObject`], [`BhkTransformShape`] or
/// [`BhkMoppBvTreeShape`], or one of their subtypes.
///
/// This trait is sealed.
pub trait HasShape: NiType {}

impl HasShape for BhkWorldObject {}
impl HasShape for BhkPhantom {}
impl HasShape for BhkShapePhantom {}
impl HasShape for BhkSimpleShapePhantom {}
impl HasShape for BhkAabbPhantom {}
impl HasShape for BhkEntity {}
impl HasShape for BhkRigidBody {}
impl HasShape for BhkRigidBodyT {}

/// Shapes, constraints and hinged bodies.
impl<'a, T: NiType> View<'a, T> {
    /// The block's collision shape, or for a transform or MOPP shape its child shape. `None` if
    /// it has none.
    pub fn shape(&self) -> Option<View<'a, BhkShape>>
    where
        T: HasShape,
    {
        self.follow(RefKind::HavokShape)
    }

    /// The rigid body's constraints: [`BhkConstraint`]s and [`BhkBallSocketConstraintChain`]s.
    /// Skips empty refs.
    pub fn constraints<I>(&self) -> BlockIter<'a, BhkSerializable, RefIndices>
    where
        T: IsA<BhkRigidBody, I>,
    {
        self.follow_list(RefList::Constraints)
    }

    /// The rigid body the action keeps oriented, or `None` if it has none.
    pub fn hinged_body<I>(&self) -> Option<View<'a, BhkRigidBody>>
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.follow(RefKind::HingedBody)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::shape`], for editing.
    pub fn shape(&mut self) -> Option<Edit<'_, BhkShape>>
    where
        T: HasShape,
    {
        self.follow(RefKind::HavokShape)
    }

    /// Sets the block's [shape](View::shape), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `shape` is a [`BhkShape`] that doesn't contain this block.
    pub fn set_shape(&mut self, shape: Option<Index>) -> bool
    where
        T: HasShape,
    {
        self.set_ref(RefKind::HavokShape, shape)
    }

    /// Replaces the rigid body's [constraints](View::constraints). Returns `false`, changing
    /// nothing, unless each is a [`BhkConstraint`] or [`BhkBallSocketConstraintChain`].
    pub fn set_constraints<I>(&mut self, constraints: &[Index]) -> bool
    where
        T: IsA<BhkRigidBody, I>,
    {
        self.set_ref_list(RefList::Constraints, constraints)
    }

    /// Like [`View::hinged_body`], for editing.
    pub fn hinged_body<I>(&mut self) -> Option<Edit<'_, BhkRigidBody>>
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.follow(RefKind::HingedBody)
    }

    /// Sets the action's [hinged body](View::hinged_body), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `body` is a [`BhkRigidBody`].
    pub fn set_hinged_body<I>(&mut self, body: Option<Index>) -> bool
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.set_ref(RefKind::HingedBody, body)
    }
}

/// An action type that [`NifFile::add_action`] can create.
///
/// This trait is sealed.
pub trait NewAction: NiType {
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32;
}

impl NewAction for BhkLiquidAction {
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32 {
        ffi::add_bhk_liquid_action(file)
    }
}

impl NewAction for BhkOrientHingedBodyAction {
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32 {
        ffi::add_bhk_orient_hinged_body_action(file)
    }
}
