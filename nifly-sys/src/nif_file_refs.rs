//! Refs between blocks, by kind, for the safe crate's `View`/`Edit` handles.

#[cxx::bridge]
pub mod ffi {
    /// A single block ref, for `block_ref` and `set_block_ref`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    enum RefKind {
        /// `NiObjectNET::controllerRef` (`NiTimeController`).
        Controller,
        /// `NiAVObject::collisionRef` (`NiCollisionObject`).
        Collision,
        /// `NiShape::ShaderPropertyRef()` (`NiShader`).
        Shader,
        /// `NiShape::AlphaPropertyRef()` (`NiAlphaProperty`).
        AlphaProperty,
        /// `NiShape::SkinInstanceRef()` (`NiBoneContainer`).
        SkinInstance,
        /// `NiShader::TextureSetRef()` (`BSShaderTextureSet`).
        TextureSet,
        /// `BSMultiBoundNode::multiBoundRef` (`BSMultiBound`).
        MultiBound,
        /// `BSMultiBound::dataRef` (`BSMultiBoundData`).
        MultiBoundData,
        /// `NiLODNode::lodLevelData` (`NiLODData`).
        LodData,
        /// `NiSkinInstance::dataRef` (`NiSkinData`).
        SkinData,
        /// `NiSkinInstance::skinPartitionRef` (`NiSkinPartition`).
        SkinPartition,
        /// `BSSkinInstance::dataRef` (`BSSkinBoneData`).
        SkinBoneData,
        /// `NiSkinData::skinPartitionRef` (`NiSkinPartition`).
        SkinDataPartition,
        /// `NiGeometryData::additionalDataRef` (`AdditionalGeomData`).
        AdditionalData,
        /// `NiSourceTexture::dataRef` (`TextureRenderData`).
        PixelData,
        /// `TextureRenderData::paletteRef` (`NiPalette`).
        Palette,
        /// `NiCollisionObject::targetRef` (`NiAVObject`), a pointer.
        CollisionTarget,
        /// `bhkNiCollisionObject::bodyRef`: a `bhkWorldObject`, or for a `bhkNPCollisionObject`
        /// a `bhkPhysicsSystem` or `bhkRagdollSystem`.
        CollisionBody,
        /// The `shapeRef` (`bhkShape`) of a `bhkWorldObject`, `bhkTransformShape` or
        /// `bhkMoppBvTreeShape`. A shape can't contain itself.
        HavokShape,
        /// `bhkOrientHingedBodyAction::bodyRef` (`bhkRigidBody`), a pointer.
        HingedBody,
        /// `bhkCompressedMeshShape::targetRef` (`NiAVObject`), a pointer.
        CompressedMeshTarget,
        /// `bhkCompressedMeshShape::dataRef` (`bhkCompressedMeshShapeData`).
        CompressedMeshData,
        /// `bhkPackedNiTriStripsShape::dataRef` (`hkPackedNiTriStripsData`).
        PackedData,
        /// The first entity (`bhkEntity`) of a `bhkConstraint` (`entityRefs`, sized to two
        /// when set) or `bhkBallSocketConstraintChain` (`entityARef`), a pointer.
        EntityA,
        /// The second entity, like `EntityA` (`entityBRef` for a chain).
        EntityB,
        /// The first entity (`bhkEntity`) of a `bhkBreakableConstraint`'s or
        /// `bhkMalleableConstraint`'s `subConstraint`, sized to two when set, a pointer.
        WrappedEntityA,
        /// The second entity, like `WrappedEntityA`.
        WrappedEntityB,
        /// The `dataRef` of a key-based interpolator (`NiBoolData`, `NiFloatData`, `NiPosData` or
        /// `NiTransformData` by type), a `BSTreadTransfInterpolator` (`NiFloatData`), or a
        /// controller: `NiKeyframeData`, `NiPosData` (material and light color), `NiVisData`,
        /// `NiFloatData` (alpha, roll), `NiUVData` or `NiMorphData`.
        KeyData,
        /// The `pathDataRef` (`NiPosData`) of an `NiPathInterpolator` or `NiPathController`.
        PathData,
        /// The `percentDataRef` (`NiFloatData`) of an `NiPathInterpolator` or `NiPathController`.
        PercentData,
        /// `NiLookAtInterpolator::lookAtRef` or `NiLookAtController::lookAtNodePtr` (`NiNode`),
        /// a pointer.
        LookAtNode,
        /// `NiLookAtInterpolator::translateInterpRef` (`NiPoint3Interpolator`).
        LookAtTranslation,
        /// `NiLookAtInterpolator::rollInterpRef` (`NiFloatInterpolator`).
        LookAtRoll,
        /// `NiLookAtInterpolator::scaleInterpRef` (`NiFloatInterpolator`).
        LookAtScale,
        /// `NiBlendInterpolator::singleInterpolatorRef` (`NiInterpolator` that doesn't use the
        /// owner).
        SingleInterpolator,
        /// `NiBSplineInterpolator::splineDataRef` (`NiBSplineData`).
        SplineData,
        /// `NiBSplineInterpolator::basisDataRef` (`NiBSplineBasisData`).
        SplineBasis,
        /// `NiTimeController::nextControllerRef` (`NiTimeController`). The chain can't loop back
        /// to the owner.
        NextController,
        /// `NiTimeController::targetRef` (`NiObjectNET`), a pointer.
        ControllerTarget,
        /// The `interpolatorRef` (`NiInterpolator`) of an `NiSingleInterpController` or
        /// `BSFrustumFOVController`.
        Interpolator,
        /// `BSProceduralLightningController::generationInterpRef` (`NiInterpolator`).
        LightningGeneration,
        /// `BSProceduralLightningController::mutationInterpRef` (`NiInterpolator`).
        LightningMutation,
        /// `BSProceduralLightningController::subdivisionInterpRef` (`NiInterpolator`).
        LightningSubdivision,
        /// `BSProceduralLightningController::numBranchesInterpRef` (`NiInterpolator`).
        LightningBranches,
        /// `BSProceduralLightningController::numBranchesVarInterpRef` (`NiInterpolator`).
        LightningBranchVariation,
        /// `BSProceduralLightningController::lengthInterpRef` (`NiInterpolator`).
        LightningLength,
        /// `BSProceduralLightningController::lengthVarInterpRef` (`NiInterpolator`).
        LightningLengthVariation,
        /// `BSProceduralLightningController::widthInterpRef` (`NiInterpolator`).
        LightningWidth,
        /// `BSProceduralLightningController::arcOffsetInterpRef` (`NiInterpolator`).
        LightningArcOffset,
        /// `BSProceduralLightningController::shaderPropertyRef` (`BSShaderProperty`).
        LightningShader,
        /// `NiParticleSystem::psysDataRef` and `dataRef`, kept equal (`NiPSysData`).
        ParticleData,
        /// `NiParticleSystem::skinInstanceRef` (`NiBoneContainer`).
        ParticleSkinInstance,
        /// `NiParticleSystem::shaderPropertyRef` (`BSShaderProperty`).
        ParticleShader,
        /// `NiParticleSystem::alphaPropertyRef` (`NiAlphaProperty`).
        ParticleAlpha,
        /// `NiPSysModifier::targetRef` (`NiParticleSystem`), a pointer.
        ParticleSystem,
        /// The `spawnModifierRef` (`NiPSysSpawnModifier`) of an `NiPSysAgeDeathModifier` or
        /// `NiPSysCollider`.
        SpawnModifier,
        /// The object a particle modifier or collider works around: `NiNode` for
        /// `NiPSysBombModifier::bombNodeRef` and `NiPSysVolumeEmitter::emitterNodeRef`,
        /// `NiAVObject` for the gravity, drag, inherit velocity and recycle bound modifiers'
        /// pointers, `NiPSysFieldModifier::fieldObjectRef` and `NiPSysCollider::colliderNodeRef`.
        ModifierObject,
        /// `NiPSysColorModifier::dataRef` or `NiParticleColorModifier::colorDataRef`
        /// (`NiColorData`).
        ColorData,
        /// `BSPSysHavokUpdateModifier::modifierRef` (`NiPSysModifier`).
        HavokUpdateModifier,
        /// `NiMeshPSysData::nodeRef` (`NiNode`).
        MeshPSysNode,
        /// `NiPSysCollider::nextColliderRef` (`NiPSysCollider`). The chain can't loop back to the
        /// owner.
        NextCollider,
        /// `NiPSysCollider::managerRef` (`NiPSysColliderManager`), a pointer.
        ColliderManager,
        /// `NiPSysEmitterCtlr::dataRef` (`NiPSysEmitterCtlrData`).
        EmitterCtlrData,
        /// `NiPSysEmitterCtlr::visInterpolatorRef` (`NiInterpolator`).
        VisibilityInterpolator,
        /// `BSPSysMultiTargetEmitterCtlr::masterParticleSystemRef` (`BSMasterParticleSystem`), a
        /// pointer.
        MasterParticleSystem,
        /// `NiParticleModifier::nextModifierRef` (`NiParticleModifier`) or
        /// `NiEmitterModifier::nextModifierRef` (`NiEmitterModifier`).
        NextParticleModifier,
        /// `NiParticleModifier::controllerRef` or `NiEmitterModifier::controllerRef`
        /// (`NiParticleSystemController`), a pointer.
        ParticleController,
        /// `NiParticleSystemController::emitterRef` (`NiAVObject`), a pointer.
        ParticleEmitter,
        /// `NiParticleMeshesData::dataRef` (`NiNode`).
        ContainerNode,
        /// `NiParticles::dataRef` (`NiParticlesData`).
        ParticlesData,
    }

    /// A list of block refs, for `block_ref_list` and `set_block_ref_list`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    enum RefList {
        /// `NiNode::childRefs` (`NiAVObject`).
        Children,
        /// `NiAVObject::propertyRefs` (`NiProperty`).
        Properties,
        /// `NiObjectNET::extraDataRefs` (`NiExtraData`).
        ExtraData,
        /// `NiObjectNET::extraDataRef` and each `NiExtraData::nextExtraDataRef` after it.
        ExtraDataChain,
        /// `NiBoneContainer::boneRefs` (`NiNode`), as many as the skin data has bones.
        Bones,
        /// `BSMasterParticleSystem::particleSysRefs` (`NiAVObject`).
        ParticleSystems,
        /// `bhkRigidBody::constraintRefs` (`bhkConstraint` or `bhkBallSocketConstraintChain`).
        Constraints,
        /// `bhkListShape::subShapeRefs` (`bhkShape`) or `bhkConvexListShape::shapeRefs`
        /// (`bhkConvexShape`). A shape can't contain itself. A list shape's filters are resized
        /// to match if there was one per sub-shape.
        SubShapes,
        /// `bhkNiTriStripsShape::partRefs` (`NiTriStripsData`). The filters are resized to match
        /// if there was one per ref.
        StripsData,
        /// `bhkBallSocketConstraintChain::chainedEntityRefs` (`bhkRigidBody`), pointers.
        ChainedEntities,
        /// `bhkRagdollTemplate::boneRefs` (`bhkRagdollTemplateData`).
        RagdollBones,
        /// `NiObjectNET::controllerRef` and each `NiTimeController::nextControllerRef` after it.
        Controllers,
        /// `NiFlipController::sourceRefs` (`NiSourceTexture`).
        FlipSources,
        /// `NiMultiTargetTransformController::targetRefs` (`NiAVObject` or empty), pointers, at most
        /// 65535.
        ExtraTargets,
        /// `NiParticleSystem::modifierRefs` (`NiPSysModifier`).
        Modifiers,
        /// The meshes (`NiAVObject`) of an `NiPSysMeshUpdateModifier`, `NiPSysMeshEmitter`
        /// (pointers) or `NiParticleMeshModifier`.
        ParticleMeshes,
        /// `BSPSysHavokUpdateModifier::nodeRefs` (`NiNode`).
        HavokUpdateNodes,
        /// `NiPSysColliderManager::colliderRef` and each `NiPSysCollider::nextColliderRef` after
        /// it.
        Colliders,
        /// `NiParticleSystemController::particleModifierRef` (`NiParticleModifier`) and each
        /// `NiParticleModifier::nextModifierRef` after it.
        ParticleModifiers,
        /// `NiParticleSystemController::particleColliderRef` (`NiParticleCollider`) and each
        /// `NiParticleModifier::nextModifierRef` (`NiParticleModifier`) after it.
        ParticleColliders,
        /// `NiParticleSystemController::emitterModifierRef` and each
        /// `NiEmitterModifier::nextModifierRef` after it (`NiEmitterModifier`).
        EmitterModifiers,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("NifFile.hpp");

        type NifFile = crate::nif_file::ffi::NifFile;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("nif_file_shim.h");

        /// The block index in the `kind` ref of the block at `owner`; `u32::MAX` if it's empty or
        /// the block has no such ref.
        fn block_ref(file: &NifFile, owner: u32, kind: RefKind) -> u32;
        /// Points the `kind` ref of the block at `owner` at `target`, or empties it for
        /// `u32::MAX`. Returns false, changing nothing, if the block has no such ref or `target`
        /// is the wrong type.
        fn set_block_ref(file: Pin<&mut NifFile>, owner: u32, kind: RefKind, target: u32) -> bool;
        /// Points the `NiGeometry` at `shape` at the geometry data at `target`, keeping the
        /// shape's cached data pointer in step. Returns false, changing nothing, if the data type
        /// doesn't fit the shape.
        fn set_shape_geometry_data(file: Pin<&mut NifFile>, shape: u32, target: u32) -> bool;
        /// The `list` refs of the block at `owner`, empty ones included. Empty if it has none.
        fn block_ref_list(file: &NifFile, owner: u32, list: RefList) -> Vec<u32>;
        /// Replaces the `list` refs of the block at `owner`. Returns false, changing nothing, if a
        /// target has the wrong type, a child would make a cycle, or the chain repeats a block.
        fn set_block_ref_list(file: Pin<&mut NifFile>, owner: u32, list: RefList, targets: &[u32]) -> bool;
    }
}
