//! Pose arrays and ragdoll template data.

use super::*;

/// A bone's transform in a [`BhkPoseArray`] pose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneTransform {
    pub translation: Vec3,
    /// A quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
    pub scale: Vec3,
}

impl Default for BoneTransform {
    fn default() -> Self {
        Self { translation: Vec3::ZERO, rotation: [0.0, 0.0, 0.0, 1.0], scale: Vec3::new(1.0, 1.0, 1.0) }
    }
}

impl BhkPoseArray {
    /// The bone names, as stored.
    pub fn bones(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::pose_array_bone_count(&self.0)).map(|i| ffi::pose_array_bone(&self.0, i).as_bstr())
    }

    pub fn set_bones<S: AsRef<[u8]>>(&mut self, bones: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(bones);
        ffi::set_pose_array_bones(self.ffi_mut(), &bytes, &lengths);
    }

    /// The poses, each usually with one transform per [bone](Self::bones).
    pub fn poses(&self) -> Vec<Vec<BoneTransform>> {
        (0..ffi::pose_array_pose_count(&self.0))
            .map(|i| {
                ffi::pose_array_pose(&self.0, i)
                    .into_iter()
                    .map(|t| BoneTransform {
                        translation: Vec3::from_ffi(t.translation),
                        rotation: t.rotation,
                        scale: Vec3::from_ffi(t.scale),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn set_poses<P: AsRef<[BoneTransform]>>(&mut self, poses: &[P]) {
        let lengths: Vec<_> = poses.iter().map(|p| p.as_ref().len()).collect();
        let transforms: Vec<_> = poses
            .iter()
            .flat_map(|p| p.as_ref())
            .map(|t| ffi::BoneTransformFfi {
                translation: t.translation.to_ffi(),
                rotation: t.rotation,
                scale: t.scale.to_ffi(),
            })
            .collect();
        assert!(ffi::set_pose_array_poses(self.ffi_mut(), &transforms, &lengths), "lengths add up");
    }
}

/// A constraint of a [`BhkRagdollTemplateData`], applied to the bone's joint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointConstraint {
    pub descriptor: ConstraintDescriptor,
    /// The solver priority, as a raw value.
    pub priority: u32,
    /// Only saved up to Oblivion.
    pub tau: f32,
    /// Only saved up to Oblivion.
    pub damping: f32,
    /// Only saved from Fallout 3 on.
    pub strength: f32,
}

/// One bone of a ragdoll: its name, physical settings and joint constraints.
impl BhkRagdollTemplateData {
    /// The bone's name, as stored.
    pub fn name(&self) -> &BStr {
        ffi::template_data_name(&self.0).as_bstr()
    }

    pub fn set_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_template_data_name(self.ffi_mut(), name.as_ref());
    }

    fields! {
        mass, set_mass: f32 = template_data_mass, set_template_data_mass;
        /// Bounciness.
        restitution, set_restitution: f32 = template_data_restitution, set_template_data_restitution;
        friction, set_friction: f32 = template_data_friction, set_template_data_friction;
        radius, set_radius: f32 = template_data_radius, set_template_data_radius;
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = template_data_material, set_template_data_material;
    }

    /// The joint's constraints. Constraints of an unknown type are left out.
    pub fn joint_constraints(&self) -> Vec<JointConstraint> {
        ffi::template_data_constraints(&self.0)
            .into_iter()
            .filter_map(|c| {
                Some(JointConstraint {
                    descriptor: ConstraintDescriptor::from_ffi(&c.descriptor)?,
                    priority: c.priority,
                    tau: c.tau,
                    damping: c.damping,
                    strength: c.strength,
                })
            })
            .collect()
    }

    /// Replaces the [joint constraints](Self::joint_constraints). Each keeps the entities of the
    /// one it replaces; new ones have none.
    pub fn set_joint_constraints(&mut self, constraints: &[JointConstraint]) {
        let constraints: Vec<_> = constraints
            .iter()
            .map(|c| ffi::JointConstraintFfi {
                descriptor: c.descriptor.to_ffi(),
                priority: c.priority,
                tau: c.tau,
                damping: c.damping,
                strength: c.strength,
                entity_a: u32::MAX,
                entity_b: u32::MAX,
            })
            .collect();
        assert!(ffi::set_template_data_constraints(self.ffi_mut(), &constraints), "known types");
    }
}

/// Ragdoll template bones.
impl<'a, T: NiType> View<'a, T> {
    /// The ragdoll template's bones. Skips empty refs.
    pub fn ragdoll_bones<I>(&self) -> BlockIter<'a, BhkRagdollTemplateData, RefIndices>
    where
        T: IsA<BhkRagdollTemplate, I>,
    {
        self.follow_list(RefList::RagdollBones)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Replaces the ragdoll template's [bones](View::ragdoll_bones). Returns `false`, changing
    /// nothing, unless each is a [`BhkRagdollTemplateData`].
    pub fn set_ragdoll_bones<I>(&mut self, bones: &[Index]) -> bool
    where
        T: IsA<BhkRagdollTemplate, I>,
    {
        self.set_ref_list(RefList::RagdollBones, bones)
    }
}
