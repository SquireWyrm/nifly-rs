//! Tests for pose arrays and ragdoll templates.

use nifly::{
    collision::{
        BhkPoseArray, BhkRagdollTemplate, BhkRagdollTemplateData, BoneTransform,
        ConstraintDescriptor, JointConstraint, Motor, RagdollDescriptor, StiffSpringDescriptor,
        VelocityMotor,
    },
    file::{NifFile, NifVersion},
    object3d::Vec3,
};

use crate::common::reload_in_place;

#[test]
fn pose_array() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let poses = nif.add_pose_array();
    let pose = |x: f32| {
        vec![
            BoneTransform {
                translation: Vec3::new(x, 0.0, 0.0),
                ..Default::default()
            },
            BoneTransform {
                rotation: [0.0, 0.0, 1.0, 0.0],
                scale: Vec3::new(2.0, 2.0, 2.0),
                ..Default::default()
            },
        ]
    };
    let p = nif.block_of_mut::<BhkPoseArray>(poses).unwrap();
    p.set_bones(["Bip01 Pelvis", "Bip01 Spine"]);
    p.set_poses(&[pose(1.0), pose(2.0), vec![]]);

    let saved = reload_in_place(&mut nif);
    let p = saved.block_of::<BhkPoseArray>(poses).unwrap();
    assert_eq!(
        p.bones().collect::<Vec<_>>(),
        ["Bip01 Pelvis", "Bip01 Spine"]
    );
    assert_eq!(p.poses(), [pose(1.0), pose(2.0), vec![]]);
}

#[test]
fn ragdoll_template() {
    let ragdoll = JointConstraint {
        descriptor: ConstraintDescriptor::Ragdoll(RagdollDescriptor {
            cone_max_angle: 0.5,
            motor: Motor::Velocity(VelocityMotor {
                tau: 0.5,
                ..Default::default()
            }),
            ..Default::default()
        }),
        priority: 1,
        tau: 0.25,
        damping: 0.5,
        strength: 10.0,
    };
    let spring = JointConstraint {
        descriptor: ConstraintDescriptor::StiffSpring(StiffSpringDescriptor {
            length: 2.0,
            ..Default::default()
        }),
        ..ragdoll
    };

    for version in [NifVersion::OBLIVION, NifVersion::SKYRIM_SE] {
        let mut nif = NifFile::create(version);
        let root = nif.root_index().unwrap();
        let template = nif.add_ragdoll_template();
        let bones = [
            nif.add_ragdoll_template_data(),
            nif.add_ragdoll_template_data(),
        ];
        assert!(nif.root_node_mut().unwrap().set_extra_data(&[template]));
        let mut edit = nif.edit::<BhkRagdollTemplate>(template).unwrap();
        assert!(!edit.set_ragdoll_bones(&[root]), "not template data");
        assert!(edit.set_ragdoll_bones(&bones));

        let d = nif
            .block_of_mut::<BhkRagdollTemplateData>(bones[0])
            .unwrap();
        assert_eq!(
            (d.mass(), d.material(), d.joint_constraints()),
            (9.0, 7, vec![]),
            "new block defaults"
        );
        d.set_name("Bip01 Head");
        d.set_mass(4.0);
        d.set_restitution(0.5);
        d.set_friction(0.25);
        d.set_radius(0.125);
        d.set_material(3);
        d.set_joint_constraints(&[ragdoll, spring]);

        let saved = reload_in_place(&mut nif);
        let t = saved.view::<BhkRagdollTemplate>(template).unwrap();
        assert_eq!(t.ragdoll_bones().indices().collect::<Vec<_>>(), bones);
        let d = saved.block_of::<BhkRagdollTemplateData>(bones[0]).unwrap();
        assert_eq!(d.name(), "Bip01 Head");
        assert_eq!(
            (
                d.mass(),
                d.restitution(),
                d.friction(),
                d.radius(),
                d.material()
            ),
            (4.0, 0.5, 0.25, 0.125, 3)
        );

        let oblivion = version == NifVersion::OBLIVION;
        let by_version = |c: JointConstraint| {
            let descriptor = match c.descriptor {
                ConstraintDescriptor::Ragdoll(r) if oblivion => {
                    ConstraintDescriptor::Ragdoll(RagdollDescriptor {
                        motor: Motor::None,
                        ..r
                    })
                }
                d => d,
            };
            let (tau, damping, strength) = if oblivion {
                (c.tau, c.damping, 0.0)
            } else {
                (0.0, 0.0, c.strength)
            };
            JointConstraint {
                descriptor,
                tau,
                damping,
                strength,
                ..c
            }
        };
        assert_eq!(
            d.joint_constraints(),
            [by_version(ragdoll), by_version(spring)]
        );
    }
}
