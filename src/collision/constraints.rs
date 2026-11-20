//! Constraints and their refs.

use super::*;

/// Its entities are on [`View::entities`].
impl BhkConstraint {
    fields! {
        /// The solver priority, as a raw value.
        priority, set_priority: u32 = constraint_priority, set_constraint_priority;
    }
}

/// The wrapped constraint's fields, for each wrapping type.
macro_rules! wrapped {
    ($($ty:ty => $get:ident, $set:ident, $ty_get:ident, $prio:ident, $set_prio:ident,
        $tau:ident, $set_tau:ident, $damping:ident, $set_damping:ident,
        $strength:ident, $set_strength:ident;)*) => {$(
        /// It wraps another constraint, whose entities are on [`View::wrapped_entities`].
        /// [`tau`](Self::tau) and [`damping`](Self::damping) are only saved up to Oblivion, and
        /// [`strength`](Self::strength) only from Fallout 3 on.
        impl $ty {
            /// The wrapped constraint's settings, or `None` if its type is unknown.
            pub fn wrapped(&self) -> Option<ConstraintDescriptor> {
                ConstraintDescriptor::from_ffi(&ffi::$get(&self.0))
            }

            /// The wrapped constraint's type as stored, for a [`wrapped`](Self::wrapped) of
            /// `None`.
            pub fn wrapped_type(&self) -> u32 {
                ffi::$ty_get(&self.0)
            }

            /// Sets the wrapped constraint's type and settings.
            pub fn set_wrapped(&mut self, descriptor: ConstraintDescriptor) {
                assert!(ffi::$set(self.ffi_mut(), &descriptor.to_ffi()), "a known type");
            }

            fields! {
                /// The wrapped constraint's solver priority, as a raw value.
                wrapped_priority, set_wrapped_priority: u32 = $prio, $set_prio;
                tau, set_tau: f32 = $tau, $set_tau;
                damping, set_damping: f32 = $damping, $set_damping;
                /// The strength; for a breakable constraint, the force that breaks it.
                strength, set_strength: f32 = $strength, $set_strength;
            }
        }
    )*};
}

wrapped! {
    BhkBreakableConstraint => breakable_wrapped, set_breakable_wrapped, breakable_wrapped_type,
        breakable_wrapped_priority, set_breakable_wrapped_priority, breakable_tau, set_breakable_tau,
        breakable_damping, set_breakable_damping, breakable_strength, set_breakable_strength;
    BhkMalleableConstraint => malleable_wrapped, set_malleable_wrapped, malleable_wrapped_type,
        malleable_wrapped_priority, set_malleable_wrapped_priority, malleable_tau, set_malleable_tau,
        malleable_damping, set_malleable_damping, malleable_strength, set_malleable_strength;
}

impl BhkBreakableConstraint {
    fields! {
        remove_when_broken, set_remove_when_broken: bool =
            breakable_remove_when_broken, set_breakable_remove_when_broken;
    }
}

/// Its entities are on [`View::entities`] and the bodies along it on
/// [`View::chained_entities`].
impl BhkBallSocketConstraintChain {
    /// The pivots, `[x, y, z, w]` in Havok units.
    pub fn pivots(&self) -> Vec<[f32; 4]> {
        ffi::chain_pivots(&self.0).as_chunks::<4>().0.to_vec()
    }

    pub fn set_pivots(&mut self, pivots: &[[f32; 4]]) {
        ffi::set_chain_pivots(self.ffi_mut(), pivots.as_flattened());
    }

    fields! {
        tau, set_tau: f32 = chain_tau, set_chain_tau;
        damping, set_damping: f32 = chain_damping, set_chain_damping;
        /// The constraint force mixing.
        cfm, set_cfm: f32 = chain_cfm, set_chain_cfm;
        max_error_distance, set_max_error_distance: f32 = chain_max_error_distance, set_chain_max_error_distance;
        /// The solver priority, as a raw value.
        priority, set_priority: u32 = chain_priority, set_chain_priority;
    }
}

/// A constraint with two [entities](View::entities): a [`BhkConstraint`] or one of its
/// subtypes, or a [`BhkBallSocketConstraintChain`].
///
/// This trait is sealed.
pub trait HasEntities: NiType {}

impl HasEntities for BhkConstraint {}
impl HasEntities for BhkHingeConstraint {}
impl HasEntities for BhkLimitedHingeConstraint {}
impl HasEntities for BhkBreakableConstraint {}
impl HasEntities for BhkRagdollConstraint {}
impl HasEntities for BhkStiffSpringConstraint {}
impl HasEntities for BhkPrismaticConstraint {}
impl HasEntities for BhkMalleableConstraint {}
impl HasEntities for BhkBallAndSocketConstraint {}
impl HasEntities for BhkBallSocketConstraintChain {}

/// A constraint that wraps another: a [`BhkBreakableConstraint`] or [`BhkMalleableConstraint`].
///
/// This trait is sealed.
pub trait WrapsConstraint: NiType {}

impl WrapsConstraint for BhkBreakableConstraint {}
impl WrapsConstraint for BhkMalleableConstraint {}

/// Constraint entities.
impl<'a, T: NiType> View<'a, T> {
    /// The two entities the constraint connects, `None` where a slot is empty.
    pub fn entities(&self) -> [Option<View<'a, BhkEntity>>; 2]
    where
        T: HasEntities,
    {
        [self.follow(RefKind::EntityA), self.follow(RefKind::EntityB)]
    }

    /// The two entities of the wrapped constraint, `None` where a slot is empty. Game files
    /// leave both empty.
    pub fn wrapped_entities(&self) -> [Option<View<'a, BhkEntity>>; 2]
    where
        T: WrapsConstraint,
    {
        [
            self.follow(RefKind::WrappedEntityA),
            self.follow(RefKind::WrappedEntityB),
        ]
    }

    /// The rigid bodies along the chain. Skips empty refs.
    pub fn chained_entities<I>(&self) -> BlockIter<'a, BhkRigidBody, RefIndices>
    where
        T: IsA<BhkBallSocketConstraintChain, I>,
    {
        self.follow_list(RefList::ChainedEntities)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Whether each of `entities` is empty or a [`BhkEntity`] in this file.
    fn entities_ok(&self, entities: [Option<Index>; 2]) -> bool {
        let file = self.view().file();
        entities
            .iter()
            .all(|e| e.is_none_or(|e| file.block_of::<BhkEntity>(e).is_some()))
    }

    /// Sets the two entities the constraint connects, `None` emptying a slot. Returns `false`,
    /// changing nothing, unless each is a [`BhkEntity`].
    pub fn set_entities(&mut self, entities: [Option<Index>; 2]) -> bool
    where
        T: HasEntities,
    {
        self.entities_ok(entities)
            && self.set_ref(RefKind::EntityA, entities[0])
            && self.set_ref(RefKind::EntityB, entities[1])
    }

    /// Sets the wrapped constraint's two [entities](View::wrapped_entities), like
    /// [`set_entities`](Self::set_entities).
    pub fn set_wrapped_entities(&mut self, entities: [Option<Index>; 2]) -> bool
    where
        T: WrapsConstraint,
    {
        self.entities_ok(entities)
            && self.set_ref(RefKind::WrappedEntityA, entities[0])
            && self.set_ref(RefKind::WrappedEntityB, entities[1])
    }

    /// Replaces the chain's [rigid bodies](View::chained_entities). Returns `false`, changing
    /// nothing, unless each is a [`BhkRigidBody`].
    pub fn set_chained_entities<I>(&mut self, bodies: &[Index]) -> bool
    where
        T: IsA<BhkBallSocketConstraintChain, I>,
    {
        self.set_ref_list(RefList::ChainedEntities, bodies)
    }

    /// Adds a new constraint of type `C` with default settings at the end of the rigid body's
    /// [constraints](View::constraints), with this body as its first [entity](View::entities),
    /// and returns its index.
    pub fn add_constraint<C: NewConstraint, I>(&mut self) -> Index
    where
        T: IsA<BhkRigidBody, I>,
    {
        let body = self.index() as u32;
        index_from_ffi(C::add(self.file_mut().inner.pin_mut(), body))
            .expect("the block is a rigid body")
    }
}

/// A constraint type that [`Edit::add_constraint`] can create.
///
/// This trait is sealed.
pub trait NewConstraint: NiType {
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>, body: u32) -> u32;
}

macro_rules! new_constraint {
    ($($ty:ty => $add:ident;)*) => {$(
        impl NewConstraint for $ty {
            fn add(file: Pin<&mut nif_ffi::NifFile>, body: u32) -> u32 {
                ffi::$add(file, body)
            }
        }
    )*};
}

new_constraint! {
    BhkHingeConstraint => add_bhk_hinge_constraint;
    BhkLimitedHingeConstraint => add_bhk_limited_hinge_constraint;
    BhkRagdollConstraint => add_bhk_ragdoll_constraint;
    BhkStiffSpringConstraint => add_bhk_stiff_spring_constraint;
    BhkPrismaticConstraint => add_bhk_prismatic_constraint;
    BhkBallAndSocketConstraint => add_bhk_ball_and_socket_constraint;
    BhkBreakableConstraint => add_bhk_breakable_constraint;
    BhkMalleableConstraint => add_bhk_malleable_constraint;
    BhkBallSocketConstraintChain => add_bhk_ball_socket_constraint_chain;
}
