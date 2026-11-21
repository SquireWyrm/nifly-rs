//! Tests for constraints: entities, descriptors and motors, wrapped constraints, chains, and
//! creating them.

use nifly::{
    collision::{
        BallAndSocketDescriptor, BhkBallAndSocketConstraint, BhkBallSocketConstraintChain,
        BhkBreakableConstraint, BhkCollisionObject, BhkConstraint, BhkHingeConstraint,
        BhkLimitedHingeConstraint, BhkMalleableConstraint, BhkPrismaticConstraint,
        BhkRagdollConstraint, BhkRigidBody, BhkStiffSpringConstraint, ConstraintDescriptor,
        HingeDescriptor, LimitedHingeDescriptor, Motor, PositionMotor, PrismaticDescriptor,
        RagdollDescriptor, SpringDamperMotor, StiffSpringDescriptor, VelocityMotor,
    },
    file::{Index, NifFile, NifVersion},
    object3d::Transform,
};

use crate::common::reload_in_place;

/// A new file of `version` with rigid bodies on the root and on a child node.
fn with_bodies(version: NifVersion) -> (NifFile, Index, Index) {
    let mut nif = NifFile::create(version);
    let child = nif
        .root_node_mut()
        .unwrap()
        .add_child("Child", Transform::default());
    let mut bodies = [0, 0];
    for (body, node) in bodies.iter_mut().zip([nif.root_index().unwrap(), child]) {
        let collision = nif
            .edit::<nifly::nodes::NiNode>(node)
            .unwrap()
            .assign_collision::<BhkCollisionObject, _>();
        *body = nif
            .edit::<BhkCollisionObject>(collision)
            .unwrap()
            .assign_body::<BhkRigidBody, _>()
            .unwrap();
    }
    (nif, bodies[0], bodies[1])
}

fn entities(nif: &NifFile, constraint: Index) -> [Option<Index>; 2] {
    nif.view::<BhkConstraint>(constraint)
        .unwrap()
        .entities()
        .map(|e| e.map(|e| e.index()))
}

fn v(x: f32) -> [f32; 4] {
    [x, x + 1.0, x + 2.0, 0.0]
}

#[test]
fn entities_and_priority() {
    let (mut nif, a, b) = with_bodies(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    let constraint = nif
        .edit::<BhkRigidBody>(a)
        .unwrap()
        .add_constraint::<BhkRagdollConstraint, _>();
    assert_eq!(entities(&nif, constraint), [Some(a), None]);

    let mut edit = nif.edit::<BhkRagdollConstraint>(constraint).unwrap();
    assert!(!edit.set_entities([Some(a), Some(root)]), "not an entity");
    assert!(edit.set_entities([Some(b), Some(a)]));
    edit.set_priority(2);

    let saved = reload_in_place(&mut nif);
    assert_eq!(entities(&saved, constraint), [Some(b), Some(a)]);
    assert_eq!(
        saved
            .block_of::<BhkConstraint>(constraint)
            .unwrap()
            .priority(),
        2
    );
    let constraints: Vec<_> = saved
        .view::<BhkRigidBody>(a)
        .unwrap()
        .constraints()
        .indices()
        .collect();
    assert_eq!(constraints, [constraint]);

    let mut nif = saved;
    assert!(
        nif.edit::<BhkConstraint>(constraint)
            .unwrap()
            .set_entities([None, Some(a)])
    );
    assert_eq!(
        entities(&reload_in_place(&mut nif), constraint),
        [None, Some(a)],
        "empty slots are kept"
    );
}

#[test]
fn descriptors_and_motors() {
    let (mut nif, a, _) = with_bodies(NifVersion::SKYRIM_SE);
    let position = Motor::Position(PositionMotor {
        tau: 0.5,
        enabled: true,
        ..Default::default()
    });
    let velocity = Motor::Velocity(VelocityMotor {
        velocity_target: 3.0,
        use_velocity_target_from_constraint_targets: true,
        ..Default::default()
    });
    let spring = Motor::SpringDamper(SpringDamperMotor {
        spring_constant: 4.0,
        spring_damping: 0.5,
        ..Default::default()
    });
    let hinge = HingeDescriptor {
        axle_a: v(1.0),
        axle_in_b2: v(2.0),
        pivot_b: v(3.0),
        ..Default::default()
    };
    let limited = LimitedHingeDescriptor {
        axle_in_b1: v(1.0),
        min_angle: -1.0,
        max_angle: 1.0,
        motor: position,
        ..Default::default()
    };
    let ragdoll = RagdollDescriptor {
        motor_a: v(4.0),
        cone_max_angle: 0.5,
        max_friction: 2.0,
        motor: velocity,
        ..Default::default()
    };
    let prismatic = PrismaticDescriptor {
        sliding_b: v(5.0),
        max_distance: 9.0,
        motor: spring,
        ..Default::default()
    };
    let spring_desc = StiffSpringDescriptor {
        pivot_a: v(6.0),
        pivot_b: v(7.0),
        length: 1.5,
    };
    let ball = BallAndSocketDescriptor {
        translation_a: v(8.0),
        translation_b: v(9.0),
    };

    let mut body = nif.edit::<BhkRigidBody>(a).unwrap();
    let ids = [
        body.add_constraint::<BhkHingeConstraint, _>(),
        body.add_constraint::<BhkLimitedHingeConstraint, _>(),
        body.add_constraint::<BhkRagdollConstraint, _>(),
        body.add_constraint::<BhkPrismaticConstraint, _>(),
        body.add_constraint::<BhkStiffSpringConstraint, _>(),
        body.add_constraint::<BhkBallAndSocketConstraint, _>(),
    ];
    nif.block_of_mut::<BhkHingeConstraint>(ids[0])
        .unwrap()
        .set_descriptor(hinge);
    nif.block_of_mut::<BhkLimitedHingeConstraint>(ids[1])
        .unwrap()
        .set_descriptor(limited);
    nif.block_of_mut::<BhkRagdollConstraint>(ids[2])
        .unwrap()
        .set_descriptor(ragdoll);
    nif.block_of_mut::<BhkPrismaticConstraint>(ids[3])
        .unwrap()
        .set_descriptor(prismatic);
    nif.block_of_mut::<BhkStiffSpringConstraint>(ids[4])
        .unwrap()
        .set_descriptor(spring_desc);
    nif.block_of_mut::<BhkBallAndSocketConstraint>(ids[5])
        .unwrap()
        .set_descriptor(ball);

    let saved = reload_in_place(&mut nif);
    assert_eq!(
        saved
            .view::<BhkRigidBody>(a)
            .unwrap()
            .constraints()
            .indices()
            .collect::<Vec<_>>(),
        ids
    );
    assert_eq!(
        saved
            .block_of::<BhkHingeConstraint>(ids[0])
            .unwrap()
            .descriptor(),
        hinge
    );
    assert_eq!(
        saved
            .block_of::<BhkLimitedHingeConstraint>(ids[1])
            .unwrap()
            .descriptor(),
        limited
    );
    assert_eq!(
        saved
            .block_of::<BhkRagdollConstraint>(ids[2])
            .unwrap()
            .descriptor(),
        ragdoll
    );
    assert_eq!(
        saved
            .block_of::<BhkPrismaticConstraint>(ids[3])
            .unwrap()
            .descriptor(),
        prismatic
    );
    assert_eq!(
        saved
            .block_of::<BhkStiffSpringConstraint>(ids[4])
            .unwrap()
            .descriptor(),
        spring_desc
    );
    assert_eq!(
        saved
            .block_of::<BhkBallAndSocketConstraint>(ids[5])
            .unwrap()
            .descriptor(),
        ball
    );
}

#[test]
fn oblivion_descriptors() {
    let (mut nif, a, _) = with_bodies(NifVersion::OBLIVION);
    let motor = Motor::Position(PositionMotor::default());
    let limited = LimitedHingeDescriptor {
        axle_in_b1: v(1.0),
        axle_in_b2: v(2.0),
        min_angle: 0.5,
        motor,
        ..Default::default()
    };
    let hinge = HingeDescriptor {
        axle_a: v(3.0),
        pivot_a: v(4.0),
        axle_b: v(5.0),
        ..Default::default()
    };
    let mut body = nif.edit::<BhkRigidBody>(a).unwrap();
    let (l, h) = (
        body.add_constraint::<BhkLimitedHingeConstraint, _>(),
        body.add_constraint::<BhkHingeConstraint, _>(),
    );
    nif.block_of_mut::<BhkLimitedHingeConstraint>(l)
        .unwrap()
        .set_descriptor(limited);
    nif.block_of_mut::<BhkHingeConstraint>(h)
        .unwrap()
        .set_descriptor(hinge);

    let saved = reload_in_place(&mut nif);
    let expected = LimitedHingeDescriptor {
        axle_in_b1: [0.0; 4],
        motor: Motor::None,
        ..limited
    };
    assert_eq!(
        saved
            .block_of::<BhkLimitedHingeConstraint>(l)
            .unwrap()
            .descriptor(),
        expected
    );
    let expected = HingeDescriptor {
        axle_a: [0.0; 4],
        ..hinge
    };
    assert_eq!(
        saved
            .block_of::<BhkHingeConstraint>(h)
            .unwrap()
            .descriptor(),
        expected
    );
}

#[test]
fn wrapped_constraints() {
    for version in [NifVersion::OBLIVION, NifVersion::SKYRIM_SE] {
        let (mut nif, a, b) = with_bodies(version);
        let mut body = nif.edit::<BhkRigidBody>(a).unwrap();
        let breakable = body.add_constraint::<BhkBreakableConstraint, _>();
        let malleable = body.add_constraint::<BhkMalleableConstraint, _>();
        let hinge = ConstraintDescriptor::Hinge(HingeDescriptor {
            pivot_a: v(1.0),
            ..Default::default()
        });
        let ragdoll = ConstraintDescriptor::Ragdoll(RagdollDescriptor {
            twist_a: v(2.0),
            ..Default::default()
        });

        let mut edit = nif.edit::<BhkBreakableConstraint>(breakable).unwrap();
        assert_eq!(
            edit.view().wrapped_entities().map(|e| e.is_none()),
            [true; 2]
        );
        assert!(
            !edit.set_wrapped_entities([Some(a), Some(0)]),
            "not an entity"
        );
        assert!(edit.set_wrapped_entities([Some(a), Some(b)]));
        let c = &mut *edit;
        assert_eq!(
            c.wrapped(),
            Some(ConstraintDescriptor::BallAndSocket(Default::default()))
        );
        c.set_wrapped(hinge);
        c.set_strength(100.0);
        c.set_tau(0.25);
        c.set_wrapped_priority(3);
        c.set_remove_when_broken(true);
        let m = nif
            .block_of_mut::<BhkMalleableConstraint>(malleable)
            .unwrap();
        m.set_wrapped(ragdoll);
        m.set_damping(0.75);

        let saved = reload_in_place(&mut nif);
        let c = saved.view::<BhkBreakableConstraint>(breakable).unwrap();
        assert_eq!(
            c.wrapped_entities().map(|e| e.map(|e| e.index())),
            [Some(a), Some(b)]
        );
        assert_eq!(
            (c.wrapped(), c.wrapped_priority(), c.remove_when_broken()),
            (Some(hinge), 3, true)
        );
        let m = saved.block_of::<BhkMalleableConstraint>(malleable).unwrap();
        assert_eq!(m.wrapped(), Some(ragdoll));
        let oblivion = version == NifVersion::OBLIVION;
        let kept = |value: f32, saved: bool| if saved { value } else { 0.0 };
        assert_eq!(
            c.strength(),
            kept(100.0, !oblivion),
            "strength only from Fallout 3 on"
        );
        assert_eq!(
            (c.tau(), m.damping()),
            (kept(0.25, oblivion), kept(0.75, oblivion)),
            "only up to Oblivion"
        );
    }
}

#[test]
fn constraint_chain() {
    let (mut nif, a, b) = with_bodies(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    let chain = nif
        .edit::<BhkRigidBody>(a)
        .unwrap()
        .add_constraint::<BhkBallSocketConstraintChain, _>();
    let mut edit = nif.edit::<BhkBallSocketConstraintChain>(chain).unwrap();
    assert_eq!(
        edit.view().entities().map(|e| e.map(|e| e.index())),
        [Some(a), None]
    );
    assert!(edit.set_entities([Some(a), Some(b)]));
    assert!(!edit.set_chained_entities(&[a, root]), "not a rigid body");
    assert!(edit.set_chained_entities(&[a, b]));
    let c = &mut *edit;
    c.set_pivots(&[v(1.0), v(2.0)]);
    c.set_tau(0.5);
    c.set_damping(0.25);
    c.set_cfm(0.125);
    c.set_max_error_distance(2.0);
    c.set_priority(4);

    let saved = reload_in_place(&mut nif);
    let c = saved.view::<BhkBallSocketConstraintChain>(chain).unwrap();
    assert_eq!(
        c.entities().map(|e| e.map(|e| e.index())),
        [Some(a), Some(b)]
    );
    assert_eq!(c.chained_entities().indices().collect::<Vec<_>>(), [a, b]);
    assert_eq!(c.pivots(), [v(1.0), v(2.0)]);
    assert_eq!(
        (
            c.tau(),
            c.damping(),
            c.cfm(),
            c.max_error_distance(),
            c.priority()
        ),
        (0.5, 0.25, 0.125, 2.0, 4)
    );
}
