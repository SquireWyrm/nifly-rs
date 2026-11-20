//! Constraint motors and descriptors.

use super::*;

/// A constraint motor, which drives the constraint toward a target. Only saved from Fallout 3
/// on, and only by [limited hinge](LimitedHingeDescriptor), [ragdoll](RagdollDescriptor) and
/// [prismatic](PrismaticDescriptor) descriptors.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Motor {
    #[default]
    None,
    Position(PositionMotor),
    Velocity(VelocityMotor),
    SpringDamper(SpringDamperMotor),
}

/// A [`Motor`] driving toward a position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub tau: f32,
    pub damping: f32,
    pub proportional_recovery_velocity: f32,
    pub constant_recovery_velocity: f32,
    pub enabled: bool,
}

impl Default for PositionMotor {
    fn default() -> Self {
        Self {
            min_force: -1_000_000.0,
            max_force: 1_000_000.0,
            tau: 0.8,
            damping: 1.0,
            proportional_recovery_velocity: 2.0,
            constant_recovery_velocity: 1.0,
            enabled: false,
        }
    }
}

/// A [`Motor`] driving toward a velocity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub tau: f32,
    pub velocity_target: f32,
    pub use_velocity_target_from_constraint_targets: bool,
    pub enabled: bool,
}

impl Default for VelocityMotor {
    fn default() -> Self {
        Self {
            min_force: -1_000_000.0,
            max_force: 1_000_000.0,
            tau: 0.0,
            velocity_target: 0.0,
            use_velocity_target_from_constraint_targets: false,
            enabled: false,
        }
    }
}

/// A [`Motor`] acting as a spring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringDamperMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub spring_constant: f32,
    pub spring_damping: f32,
    pub enabled: bool,
}

impl Default for SpringDamperMotor {
    fn default() -> Self {
        Self {
            min_force: -1_000_000.0,
            max_force: 1_000_000.0,
            spring_constant: 0.0,
            spring_damping: 0.0,
            enabled: false,
        }
    }
}

impl Motor {
    pub(super) fn from_ffi(m: &ffi::MotorFfi) -> Self {
        match m.motor_type {
            1 => Self::Position(PositionMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                tau: m.tau,
                damping: m.damping,
                proportional_recovery_velocity: m.proportional_recovery_velocity,
                constant_recovery_velocity: m.constant_recovery_velocity,
                enabled: m.enabled,
            }),
            2 => Self::Velocity(VelocityMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                tau: m.tau,
                velocity_target: m.velocity_target,
                use_velocity_target_from_constraint_targets: m.use_velocity_target,
                enabled: m.enabled,
            }),
            3 => Self::SpringDamper(SpringDamperMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                spring_constant: m.spring_constant,
                spring_damping: m.spring_damping,
                enabled: m.enabled,
            }),
            _ => Self::None,
        }
    }

    pub(super) fn to_ffi(self) -> ffi::MotorFfi {
        let mut m = ffi::MotorFfi {
            motor_type: 0,
            min_force: 0.0,
            max_force: 0.0,
            tau: 0.0,
            damping: 0.0,
            proportional_recovery_velocity: 0.0,
            constant_recovery_velocity: 0.0,
            velocity_target: 0.0,
            use_velocity_target: false,
            spring_constant: 0.0,
            spring_damping: 0.0,
            enabled: false,
        };
        match self {
            Self::None => {}
            Self::Position(p) => {
                m.motor_type = 1;
                (m.min_force, m.max_force, m.tau, m.damping) =
                    (p.min_force, p.max_force, p.tau, p.damping);
                m.proportional_recovery_velocity = p.proportional_recovery_velocity;
                m.constant_recovery_velocity = p.constant_recovery_velocity;
                m.enabled = p.enabled;
            }
            Self::Velocity(v) => {
                m.motor_type = 2;
                (m.min_force, m.max_force, m.tau, m.velocity_target) =
                    (v.min_force, v.max_force, v.tau, v.velocity_target);
                m.use_velocity_target = v.use_velocity_target_from_constraint_targets;
                m.enabled = v.enabled;
            }
            Self::SpringDamper(s) => {
                m.motor_type = 3;
                (m.min_force, m.max_force) = (s.min_force, s.max_force);
                (m.spring_constant, m.spring_damping) = (s.spring_constant, s.spring_damping);
                m.enabled = s.enabled;
            }
        }
        m
    }
}

/// Builds a `DescriptorFfi` from vectors, floats and a motor.
fn descriptor_ffi(vectors: &[[f32; 4]], floats: &[f32], motor: Motor) -> ffi::DescriptorFfi {
    let mut d = ffi::DescriptorFfi {
        constraint_type: 0,
        vectors: [0.0; 32],
        floats: [0.0; 6],
        motor: motor.to_ffi(),
    };
    d.vectors[..vectors.len() * 4].copy_from_slice(vectors.as_flattened());
    d.floats[..floats.len()].copy_from_slice(floats);
    d
}

fn vector(d: &ffi::DescriptorFfi, i: usize) -> [f32; 4] {
    d.vectors[i * 4..i * 4 + 4].try_into().unwrap()
}

/// A [`BhkBallAndSocketConstraint`]'s settings: a point on each entity, held together.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BallAndSocketDescriptor {
    pub translation_a: [f32; 4],
    pub translation_b: [f32; 4],
}

impl BallAndSocketDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            translation_a: vector(d, 0),
            translation_b: vector(d, 1),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        descriptor_ffi(&[self.translation_a, self.translation_b], &[], Motor::None)
    }
}

/// A [`BhkStiffSpringConstraint`]'s settings: a point on each entity, kept `length` apart.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StiffSpringDescriptor {
    pub pivot_a: [f32; 4],
    pub pivot_b: [f32; 4],
    pub length: f32,
}

impl StiffSpringDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            pivot_a: vector(d, 0),
            pivot_b: vector(d, 1),
            length: d.floats[0],
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        descriptor_ffi(&[self.pivot_a, self.pivot_b], &[self.length], Motor::None)
    }
}

/// A [`BhkHingeConstraint`]'s settings. Vectors are `[x, y, z, w]` in each entity's space, in
/// Havok units.
///
/// Oblivion files save only `pivot_a`, `axle_in_a1`, `axle_in_a2`, `pivot_b` and `axle_b`.
/// Files between 20.0.0.5 and 20.2.0.7 save none.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HingeDescriptor {
    pub axle_a: [f32; 4],
    pub axle_in_a1: [f32; 4],
    pub axle_in_a2: [f32; 4],
    pub pivot_a: [f32; 4],
    pub axle_b: [f32; 4],
    pub axle_in_b1: [f32; 4],
    pub axle_in_b2: [f32; 4],
    pub pivot_b: [f32; 4],
}

impl HingeDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            axle_a: vector(d, 0),
            axle_in_a1: vector(d, 1),
            axle_in_a2: vector(d, 2),
            pivot_a: vector(d, 3),
            axle_b: vector(d, 4),
            axle_in_b1: vector(d, 5),
            axle_in_b2: vector(d, 6),
            pivot_b: vector(d, 7),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.axle_a,
            self.axle_in_a1,
            self.axle_in_a2,
            self.pivot_a,
            self.axle_b,
            self.axle_in_b1,
            self.axle_in_b2,
            self.pivot_b,
        ];
        descriptor_ffi(&v, &[], Motor::None)
    }
}

/// A [`BhkLimitedHingeConstraint`]'s settings: a [hinge](HingeDescriptor) with angle limits, in
/// radians. Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Oblivion files don't save `axle_in_b1` or the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LimitedHingeDescriptor {
    pub axle_a: [f32; 4],
    pub axle_in_a1: [f32; 4],
    pub axle_in_a2: [f32; 4],
    pub pivot_a: [f32; 4],
    pub axle_b: [f32; 4],
    pub axle_in_b1: [f32; 4],
    pub axle_in_b2: [f32; 4],
    pub pivot_b: [f32; 4],
    pub min_angle: f32,
    pub max_angle: f32,
    pub max_friction: f32,
    pub motor: Motor,
}

impl LimitedHingeDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            axle_a: vector(d, 0),
            axle_in_a1: vector(d, 1),
            axle_in_a2: vector(d, 2),
            pivot_a: vector(d, 3),
            axle_b: vector(d, 4),
            axle_in_b1: vector(d, 5),
            axle_in_b2: vector(d, 6),
            pivot_b: vector(d, 7),
            min_angle: d.floats[0],
            max_angle: d.floats[1],
            max_friction: d.floats[2],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.axle_a,
            self.axle_in_a1,
            self.axle_in_a2,
            self.pivot_a,
            self.axle_b,
            self.axle_in_b1,
            self.axle_in_b2,
            self.pivot_b,
        ];
        descriptor_ffi(
            &v,
            &[self.min_angle, self.max_angle, self.max_friction],
            self.motor,
        )
    }
}

/// A [`BhkRagdollConstraint`]'s settings: a joint with a cone and twist limits, in radians.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Oblivion files don't save `motor_a`, `motor_b` or the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RagdollDescriptor {
    pub twist_a: [f32; 4],
    pub plane_a: [f32; 4],
    pub motor_a: [f32; 4],
    pub pivot_a: [f32; 4],
    pub twist_b: [f32; 4],
    pub plane_b: [f32; 4],
    pub motor_b: [f32; 4],
    pub pivot_b: [f32; 4],
    pub cone_max_angle: f32,
    pub plane_min_angle: f32,
    pub plane_max_angle: f32,
    pub twist_min_angle: f32,
    pub twist_max_angle: f32,
    pub max_friction: f32,
    pub motor: Motor,
}

impl RagdollDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            twist_a: vector(d, 0),
            plane_a: vector(d, 1),
            motor_a: vector(d, 2),
            pivot_a: vector(d, 3),
            twist_b: vector(d, 4),
            plane_b: vector(d, 5),
            motor_b: vector(d, 6),
            pivot_b: vector(d, 7),
            cone_max_angle: d.floats[0],
            plane_min_angle: d.floats[1],
            plane_max_angle: d.floats[2],
            twist_min_angle: d.floats[3],
            twist_max_angle: d.floats[4],
            max_friction: d.floats[5],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.twist_a,
            self.plane_a,
            self.motor_a,
            self.pivot_a,
            self.twist_b,
            self.plane_b,
            self.motor_b,
            self.pivot_b,
        ];
        let f = [
            self.cone_max_angle,
            self.plane_min_angle,
            self.plane_max_angle,
            self.twist_min_angle,
            self.twist_max_angle,
            self.max_friction,
        ];
        descriptor_ffi(&v, &f, self.motor)
    }
}

/// A [`BhkPrismaticConstraint`]'s settings: a sliding joint with distance limits. Vectors are
/// `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Files between 20.0.0.5 and 20.2.0.7 save none of the vectors, and Oblivion files don't
/// save the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PrismaticDescriptor {
    pub sliding_a: [f32; 4],
    pub rotation_a: [f32; 4],
    pub plane_a: [f32; 4],
    pub pivot_a: [f32; 4],
    pub sliding_b: [f32; 4],
    pub rotation_b: [f32; 4],
    pub plane_b: [f32; 4],
    pub pivot_b: [f32; 4],
    pub min_distance: f32,
    pub max_distance: f32,
    pub friction: f32,
    pub motor: Motor,
}

impl PrismaticDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            sliding_a: vector(d, 0),
            rotation_a: vector(d, 1),
            plane_a: vector(d, 2),
            pivot_a: vector(d, 3),
            sliding_b: vector(d, 4),
            rotation_b: vector(d, 5),
            plane_b: vector(d, 6),
            pivot_b: vector(d, 7),
            min_distance: d.floats[0],
            max_distance: d.floats[1],
            friction: d.floats[2],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.sliding_a,
            self.rotation_a,
            self.plane_a,
            self.pivot_a,
            self.sliding_b,
            self.rotation_b,
            self.plane_b,
            self.pivot_b,
        ];
        descriptor_ffi(
            &v,
            &[self.min_distance, self.max_distance, self.friction],
            self.motor,
        )
    }
}

/// The settings of a constraint wrapped by a [`BhkBreakableConstraint`] or
/// [`BhkMalleableConstraint`]. The variant is the wrapped constraint's type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstraintDescriptor {
    BallAndSocket(BallAndSocketDescriptor),
    Hinge(HingeDescriptor),
    LimitedHinge(LimitedHingeDescriptor),
    Prismatic(PrismaticDescriptor),
    Ragdoll(RagdollDescriptor),
    StiffSpring(StiffSpringDescriptor),
}

impl ConstraintDescriptor {
    pub(super) fn from_ffi(d: &ffi::DescriptorFfi) -> Option<Self> {
        Some(match d.constraint_type {
            0 => Self::BallAndSocket(BallAndSocketDescriptor::from_ffi(d)),
            1 => Self::Hinge(HingeDescriptor::from_ffi(d)),
            2 => Self::LimitedHinge(LimitedHingeDescriptor::from_ffi(d)),
            6 => Self::Prismatic(PrismaticDescriptor::from_ffi(d)),
            7 => Self::Ragdoll(RagdollDescriptor::from_ffi(d)),
            8 => Self::StiffSpring(StiffSpringDescriptor::from_ffi(d)),
            _ => return None,
        })
    }

    pub(super) fn to_ffi(self) -> ffi::DescriptorFfi {
        let (constraint_type, mut d) = match self {
            Self::BallAndSocket(c) => (0, c.to_ffi()),
            Self::Hinge(c) => (1, c.to_ffi()),
            Self::LimitedHinge(c) => (2, c.to_ffi()),
            Self::Prismatic(c) => (6, c.to_ffi()),
            Self::Ragdoll(c) => (7, c.to_ffi()),
            Self::StiffSpring(c) => (8, c.to_ffi()),
        };
        d.constraint_type = constraint_type;
        d
    }
}

/// A descriptor getter and setter for each constraint type.
macro_rules! descriptors {
    ($($ty:ty => $desc:ty, $get:ident, $set:ident;)*) => {$(
        impl $ty {
            /// The constraint's settings.
            pub fn descriptor(&self) -> $desc {
                <$desc>::from_ffi(&ffi::$get(&self.0))
            }

            /// Sets the [descriptor](Self::descriptor).
            pub fn set_descriptor(&mut self, descriptor: $desc) {
                ffi::$set(self.ffi_mut(), &descriptor.to_ffi());
            }
        }
    )*};
}

descriptors! {
    BhkHingeConstraint => HingeDescriptor, hinge_descriptor, set_hinge_descriptor;
    BhkLimitedHingeConstraint => LimitedHingeDescriptor, limited_hinge_descriptor, set_limited_hinge_descriptor;
    BhkRagdollConstraint => RagdollDescriptor, ragdoll_descriptor, set_ragdoll_descriptor;
    BhkStiffSpringConstraint => StiffSpringDescriptor, stiff_spring_descriptor, set_stiff_spring_descriptor;
    BhkPrismaticConstraint => PrismaticDescriptor, prismatic_descriptor, set_prismatic_descriptor;
    BhkBallAndSocketConstraint => BallAndSocketDescriptor, ball_and_socket_descriptor, set_ball_and_socket_descriptor;
}
