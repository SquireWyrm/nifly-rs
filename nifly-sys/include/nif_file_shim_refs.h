// Part of nif_file_shim.h: single block refs, by RefKind.
#pragma once

#include "nif_file_shim.h"
#include "nifly-sys/src/nif_file_refs.rs.h"

namespace nifly_rs {
// Refs to existing blocks, by RefKind (nif_file_refs.rs).

// The ref of kind `kind` on `obj`, or null if it has none of that kind.
// `target_ok` checks a block can go in it.
struct RefSlot {
    nifly::NiRef* ref = nullptr;
    bool (*target_ok)(nifly::NiObject*) = nullptr;
};

template<class T>
bool is_a(nifly::NiObject* obj) {
    return dynamic_cast<T*>(obj) != nullptr;
}

template<class O, class T, class Get>
RefSlot ref_slot(nifly::NiObject& obj, Get get) {
    auto owner = dynamic_cast<O*>(&obj);
    if (!owner)
        return {};
    return {get(*owner), &is_a<T>};
}

// Slot `i` of a constraint's entity array, or none if it has fewer slots.
inline RefSlot
entity_slot(nifly::NiBlockRefArray<nifly::bhkEntity>& refs, uint32_t i) {
    if (refs.GetSize() <= i)
        return {};
    return {&*(refs.begin() + i), &is_a<nifly::bhkEntity>};
}

// Gives the entity arrays behind `kind` on `obj` their two slots, which every
// save writes, so the slots exist to set.
inline void make_entity_slots(nifly::NiObject& obj, RefKind kind) {
    using namespace nifly;
    if (kind == RefKind::EntityA || kind == RefKind::EntityB) {
        if (auto c = dynamic_cast<bhkConstraint*>(&obj))
            two_entity_slots(c->entityRefs);
    } else if (
        kind == RefKind::WrappedEntityA || kind == RefKind::WrappedEntityB
    ) {
        if (auto c = dynamic_cast<bhkBreakableConstraint*>(&obj))
            two_entity_slots(c->subConstraint.entityRefs);
        if (auto c = dynamic_cast<bhkMalleableConstraint*>(&obj))
            two_entity_slots(c->subConstraint.entityRefs);
    }
}

inline RefSlot find_ref(nifly::NiObject& obj, RefKind kind) {
    using namespace nifly;
    switch (kind) {
        case RefKind::Controller:
            return ref_slot<NiObjectNET, NiTimeController>(obj, [](auto& o) {
                return &o.controllerRef;
            });
        case RefKind::Collision:
            return ref_slot<NiAVObject, NiCollisionObject>(obj, [](auto& o) {
                return &o.collisionRef;
            });
        case RefKind::Shader:
            return ref_slot<NiShape, NiShader>(obj, [](auto& o) {
                return o.ShaderPropertyRef();
            });
        case RefKind::AlphaProperty:
            return ref_slot<NiShape, NiAlphaProperty>(obj, [](auto& o) {
                return o.AlphaPropertyRef();
            });
        case RefKind::SkinInstance:
            return ref_slot<NiShape, NiBoneContainer>(obj, [](auto& o) {
                return o.SkinInstanceRef();
            });
        case RefKind::TextureSet:
            return ref_slot<NiShader, BSShaderTextureSet>(obj, [](auto& o) {
                return o.TextureSetRef();
            });
        case RefKind::MultiBound:
            return ref_slot<BSMultiBoundNode, BSMultiBound>(obj, [](auto& o) {
                return &o.multiBoundRef;
            });
        case RefKind::MultiBoundData:
            return ref_slot<BSMultiBound, BSMultiBoundData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::LodData:
            return ref_slot<NiLODNode, NiLODData>(obj, [](auto& o) {
                return &o.lodLevelData;
            });
        case RefKind::SkinData:
            return ref_slot<NiSkinInstance, NiSkinData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::SkinPartition:
            return ref_slot<NiSkinInstance, NiSkinPartition>(obj, [](auto& o) {
                return &o.skinPartitionRef;
            });
        case RefKind::SkinBoneData:
            return ref_slot<BSSkinInstance, BSSkinBoneData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::SkinDataPartition:
            return ref_slot<NiSkinData, NiSkinPartition>(obj, [](auto& o) {
                return &o.skinPartitionRef;
            });
        case RefKind::AdditionalData:
            return ref_slot<NiGeometryData, AdditionalGeomData>(
                obj,
                [](auto& o) { return &o.additionalDataRef; }
            );
        case RefKind::PixelData:
            return ref_slot<NiSourceTexture, TextureRenderData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::Palette:
            return ref_slot<TextureRenderData, NiPalette>(obj, [](auto& o) {
                return &o.paletteRef;
            });
        case RefKind::CollisionTarget:
            return ref_slot<NiCollisionObject, NiAVObject>(obj, [](auto& o) {
                return &o.targetRef;
            });
        case RefKind::CollisionBody:
            if (auto np = dynamic_cast<bhkNPCollisionObject*>(&obj))
                return {&np->bodyRef, [](NiObject* target) {
                            return is_a<bhkPhysicsSystem>(target)
                                || is_a<bhkRagdollSystem>(target);
                        }};
            return ref_slot<bhkNiCollisionObject, bhkWorldObject>(
                obj,
                [](auto& o) { return &o.bodyRef; }
            );
        case RefKind::HavokShape:
            if (auto ref = child_shape_ref(&obj))
                return {ref, &is_a<bhkShape>};
            return {};
        case RefKind::HingedBody:
            return ref_slot<bhkOrientHingedBodyAction, bhkRigidBody>(
                obj,
                [](auto& o) { return &o.bodyRef; }
            );
        case RefKind::CompressedMeshTarget:
            return ref_slot<bhkCompressedMeshShape, NiAVObject>(
                obj,
                [](auto& o) { return &o.targetRef; }
            );
        case RefKind::CompressedMeshData:
            return ref_slot<bhkCompressedMeshShape, bhkCompressedMeshShapeData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::EntityA:
        case RefKind::EntityB: {
            uint32_t i = kind == RefKind::EntityA ? 0 : 1;
            if (auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(&obj))
                return {
                    i == 0 ? &chain->entityARef : &chain->entityBRef,
                    &is_a<bhkEntity>
                };
            if (auto c = dynamic_cast<bhkConstraint*>(&obj))
                return entity_slot(c->entityRefs, i);
            return {};
        }
        case RefKind::WrappedEntityA:
        case RefKind::WrappedEntityB: {
            uint32_t i = kind == RefKind::WrappedEntityA ? 0 : 1;
            if (auto c = dynamic_cast<bhkBreakableConstraint*>(&obj))
                return entity_slot(c->subConstraint.entityRefs, i);
            if (auto c = dynamic_cast<bhkMalleableConstraint*>(&obj))
                return entity_slot(c->subConstraint.entityRefs, i);
            return {};
        }
        case RefKind::PackedData:
            return ref_slot<bhkPackedNiTriStripsShape, hkPackedNiTriStripsData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::KeyData:
            if (auto i = dynamic_cast<NiBoolInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiBoolData>};
            if (auto i = dynamic_cast<NiFloatInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiFloatData>};
            if (auto i = dynamic_cast<NiPoint3Interpolator*>(&obj))
                return {&i->dataRef, &is_a<NiPosData>};
            if (auto i = dynamic_cast<NiTransformInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiTransformData>};
            if (auto i = dynamic_cast<BSTreadTransfInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiKeyframeController*>(&obj))
                return {&c->dataRef, &is_a<NiKeyframeData>};
            if (auto c = dynamic_cast<NiMaterialColorController*>(&obj))
                return {&c->dataRef, &is_a<NiPosData>};
            if (auto c = dynamic_cast<NiLightColorController*>(&obj))
                return {&c->dataRef, &is_a<NiPosData>};
            if (auto c = dynamic_cast<NiVisController*>(&obj))
                return {&c->dataRef, &is_a<NiVisData>};
            if (auto c = dynamic_cast<NiAlphaController*>(&obj))
                return {&c->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiRollController*>(&obj))
                return {&c->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiUVController*>(&obj))
                return {&c->dataRef, &is_a<NiUVData>};
            if (auto c = dynamic_cast<NiGeomMorpherController*>(&obj))
                return {&c->dataRef, &is_a<NiMorphData>};
            return {};
        case RefKind::PathData:
            if (auto i = dynamic_cast<NiPathInterpolator*>(&obj))
                return {&path_data_field(*i), &is_a<NiPosData>};
            return ref_slot<NiPathController, NiPosData>(obj, [](auto& o) {
                return &o.pathDataRef;
            });
        case RefKind::PercentData:
            if (auto i = dynamic_cast<NiPathInterpolator*>(&obj))
                return {&path_percent_field(*i), &is_a<NiFloatData>};
            return ref_slot<NiPathController, NiFloatData>(obj, [](auto& o) {
                return &o.percentDataRef;
            });
        case RefKind::LookAtNode:
            if (auto c = dynamic_cast<NiLookAtController*>(&obj))
                return {&c->lookAtNodePtr, &is_a<NiNode>};
            return ref_slot<NiLookAtInterpolator, NiNode>(obj, [](auto& o) {
                return &o.lookAtRef;
            });
        case RefKind::LookAtTranslation:
            return ref_slot<NiLookAtInterpolator, NiPoint3Interpolator>(
                obj,
                [](auto& o) { return &o.translateInterpRef; }
            );
        case RefKind::LookAtRoll:
            return ref_slot<NiLookAtInterpolator, NiFloatInterpolator>(
                obj,
                [](auto& o) { return &o.rollInterpRef; }
            );
        case RefKind::LookAtScale:
            return ref_slot<NiLookAtInterpolator, NiFloatInterpolator>(
                obj,
                [](auto& o) { return &o.scaleInterpRef; }
            );
        case RefKind::SingleInterpolator:
            return ref_slot<NiBlendInterpolator, NiInterpolator>(
                obj,
                [](auto& o) { return &o.singleInterpolatorRef; }
            );
        case RefKind::SplineData:
            return ref_slot<NiBSplineInterpolator, NiBSplineData>(
                obj,
                [](auto& o) { return &o.splineDataRef; }
            );
        case RefKind::SplineBasis:
            return ref_slot<NiBSplineInterpolator, NiBSplineBasisData>(
                obj,
                [](auto& o) { return &o.basisDataRef; }
            );
        case RefKind::NextController:
            return ref_slot<NiTimeController, NiTimeController>(
                obj,
                [](auto& o) { return &o.nextControllerRef; }
            );
        case RefKind::ControllerTarget:
            return ref_slot<NiTimeController, NiObjectNET>(obj, [](auto& o) {
                return &o.targetRef;
            });
        case RefKind::Interpolator:
            if (auto c = dynamic_cast<BSFrustumFOVController*>(&obj))
                return {&c->interpolatorRef, &is_a<NiInterpolator>};
            return ref_slot<NiSingleInterpController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.interpolatorRef; }
            );
        case RefKind::LightningGeneration:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.generationInterpRef; }
            );
        case RefKind::LightningMutation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.mutationInterpRef; }
            );
        case RefKind::LightningSubdivision:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.subdivisionInterpRef; }
            );
        case RefKind::LightningBranches:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.numBranchesInterpRef; }
            );
        case RefKind::LightningBranchVariation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.numBranchesVarInterpRef; }
            );
        case RefKind::LightningLength:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.lengthInterpRef; }
            );
        case RefKind::LightningLengthVariation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.lengthVarInterpRef; }
            );
        case RefKind::LightningWidth:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.widthInterpRef; }
            );
        case RefKind::LightningArcOffset:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.arcOffsetInterpRef; }
            );
        case RefKind::LightningShader:
            return ref_slot<BSProceduralLightningController, BSShaderProperty>(
                obj,
                [](auto& o) { return &o.shaderPropertyRef; }
            );
        case RefKind::ParticleData:
            return ref_slot<NiParticleSystem, NiPSysData>(obj, [](auto& o) {
                return &o.psysDataRef;
            });
        case RefKind::ParticleSkinInstance:
            return ref_slot<NiParticleSystem, NiBoneContainer>(
                obj,
                [](auto& o) { return &o.skinInstanceRef; }
            );
        case RefKind::ParticleShader:
            return ref_slot<NiParticleSystem, BSShaderProperty>(
                obj,
                [](auto& o) { return &o.shaderPropertyRef; }
            );
        case RefKind::ParticleAlpha:
            return ref_slot<NiParticleSystem, NiAlphaProperty>(
                obj,
                [](auto& o) { return &o.alphaPropertyRef; }
            );
        case RefKind::ParticleSystem:
            return ref_slot<NiPSysModifier, NiParticleSystem>(obj, [](auto& o) {
                return &o.targetRef;
            });
        case RefKind::SpawnModifier:
            if (auto m = dynamic_cast<NiPSysAgeDeathModifier*>(&obj))
                return {&m->spawnModifierRef, &is_a<NiPSysSpawnModifier>};
            return ref_slot<NiPSysCollider, NiPSysSpawnModifier>(
                obj,
                [](auto& o) { return &o.spawnModifierRef; }
            );
        case RefKind::ModifierObject:
            if (auto m = dynamic_cast<NiPSysGravityModifier*>(&obj))
                return {&m->gravityObjRef, &is_a<NiAVObject>};
            if (auto m = dynamic_cast<NiPSysDragModifier*>(&obj))
                return {&m->parentRef, &is_a<NiAVObject>};
            if (auto m = dynamic_cast<BSPSysInheritVelocityModifier*>(&obj))
                return {&m->targetNodeRef, &is_a<NiAVObject>};
            if (auto m = dynamic_cast<NiPSysBombModifier*>(&obj))
                return {&m->bombNodeRef, &is_a<NiNode>};
            if (auto m = dynamic_cast<NiPSysFieldModifier*>(&obj))
                return {&m->fieldObjectRef, &is_a<NiAVObject>};
            if (auto m = dynamic_cast<BSPSysRecycleBoundModifier*>(&obj))
                return {&m->targetNodeRef, &is_a<NiAVObject>};
            if (auto m = dynamic_cast<NiPSysVolumeEmitter*>(&obj))
                return {&m->emitterNodeRef, &is_a<NiNode>};
            return ref_slot<NiPSysCollider, NiAVObject>(obj, [](auto& o) {
                return &o.colliderNodeRef;
            });
        case RefKind::ColorData:
            if (auto m = dynamic_cast<NiPSysColorModifier*>(&obj))
                return {&m->dataRef, &is_a<NiColorData>};
            return ref_slot<NiParticleColorModifier, NiColorData>(
                obj,
                [](auto& o) { return &o.colorDataRef; }
            );
        case RefKind::HavokUpdateModifier:
            return ref_slot<BSPSysHavokUpdateModifier, NiPSysModifier>(
                obj,
                [](auto& o) { return &o.modifierRef; }
            );
        case RefKind::MeshPSysNode:
            return ref_slot<NiMeshPSysData, NiNode>(obj, [](auto& o) {
                return &o.nodeRef;
            });
        case RefKind::NextCollider:
            return ref_slot<NiPSysCollider, NiPSysCollider>(obj, [](auto& o) {
                return &o.nextColliderRef;
            });
        case RefKind::ColliderManager:
            return ref_slot<NiPSysCollider, NiPSysColliderManager>(
                obj,
                [](auto& o) { return &o.managerRef; }
            );
        case RefKind::EmitterCtlrData:
            return ref_slot<NiPSysEmitterCtlr, NiPSysEmitterCtlrData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::VisibilityInterpolator:
            return ref_slot<NiPSysEmitterCtlr, NiInterpolator>(
                obj,
                [](auto& o) { return &o.visInterpolatorRef; }
            );
        case RefKind::MasterParticleSystem:
            return ref_slot<
                BSPSysMultiTargetEmitterCtlr,
                BSMasterParticleSystem>(obj, [](auto& o) {
                return &o.masterParticleSystemRef;
            });
        case RefKind::NextParticleModifier:
            if (auto m = dynamic_cast<NiEmitterModifier*>(&obj))
                return {&m->nextModifierRef, &is_a<NiEmitterModifier>};
            return ref_slot<NiParticleModifier, NiParticleModifier>(
                obj,
                [](auto& o) { return &o.nextModifierRef; }
            );
        case RefKind::ParticleController:
            if (auto m = dynamic_cast<NiEmitterModifier*>(&obj))
                return {&m->controllerRef, &is_a<NiParticleSystemController>};
            return ref_slot<NiParticleModifier, NiParticleSystemController>(
                obj,
                [](auto& o) { return &o.controllerRef; }
            );
        case RefKind::ParticleEmitter:
            return ref_slot<NiParticleSystemController, NiAVObject>(
                obj,
                [](auto& o) { return &o.emitterRef; }
            );
        case RefKind::ContainerNode:
            return ref_slot<NiParticleMeshesData, NiNode>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::ParticlesData:
            // NiGeometry's dataRef is protected.
            return ref_slot<NiParticles, NiParticlesData>(obj, [](auto& o) {
                return o.DataRef();
            });
        default:
            return {};
    }
}

// Whether the shape at `target` is the one at `root` or contained in it,
// through child shape and sub-shape refs.
inline bool
in_shape_tree(const nifly::NiHeader& hdr, uint32_t root, uint32_t target) {
    using namespace nifly;
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {root};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == target)
            return true;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        auto obj = hdr.GetBlock<bhkShape>(id);
        if (!obj)
            continue;
        if (auto ref = child_shape_ref(obj))
            stack.push_back(ref->index);
        if (auto list = dynamic_cast<bhkListShape*>(obj))
            for (uint32_t i = 0; i < list->subShapeRefs.GetSize(); i++)
                stack.push_back(list->subShapeRefs.GetBlockRef(i));
        if (auto list = dynamic_cast<bhkConvexListShape*>(obj))
            for (uint32_t i = 0; i < list->shapeRefs.GetSize(); i++)
                stack.push_back(list->shapeRefs.GetBlockRef(i));
    }
    return false;
}

// Whether the collider chain from `from`, through next collider refs, reaches
// the block at `target`. Stops at a repeat.
inline bool collider_chain_reaches(
    const nifly::NiHeader& hdr,
    uint32_t from,
    uint32_t target
) {
    std::set<uint32_t> seen;
    for (uint32_t at = from; at != nifly::NIF_NPOS;) {
        if (at == target)
            return true;
        auto c = hdr.GetBlock<nifly::NiPSysCollider>(at);
        if (!c || !seen.insert(at).second)
            return false;
        at = c->nextColliderRef.index;
    }
    return false;
}

// The next modifier ref of an old particle modifier or emitter modifier, or
// null for another block.
inline nifly::NiRef* next_particle_modifier(nifly::NiObject* obj) {
    if (auto m = dynamic_cast<nifly::NiParticleModifier*>(obj))
        return &m->nextModifierRef;
    if (auto m = dynamic_cast<nifly::NiEmitterModifier*>(obj))
        return &m->nextModifierRef;
    return nullptr;
}

// Whether the old particle modifier chain from `from`, through next modifier
// refs, reaches the block at `target`. Stops at a repeat.
inline bool particle_modifier_chain_reaches(
    const nifly::NiHeader& hdr,
    uint32_t from,
    uint32_t target
) {
    std::set<uint32_t> seen;
    for (uint32_t at = from; at != nifly::NIF_NPOS;) {
        if (at == target)
            return true;
        auto next = next_particle_modifier(hdr.GetBlock<nifly::NiObject>(at));
        if (!next || !seen.insert(at).second)
            return false;
        at = next->index;
    }
    return false;
}

// The chain of old particle modifiers from `first`, a `Head`, through the
// next modifier refs of `Link`s. Stops at a block of another type or a repeat.
template<class Head, class Link>
rust::Vec<uint32_t>
particle_modifier_chain(const nifly::NiHeader& hdr, uint32_t first) {
    rust::Vec<uint32_t> out;
    if (!hdr.GetBlock<Head>(first))
        return out;
    std::set<uint32_t> seen;
    for (uint32_t at = first; auto m = hdr.GetBlock<Link>(at);) {
        if (!seen.insert(at).second)
            break;
        out.push_back(at);
        at = m->nextModifierRef.index;
    }
    return out;
}

// Whether the controller chain from `from`, through next controller refs,
// reaches the block at `target`. Stops at a repeat.
inline bool controller_chain_reaches(
    const nifly::NiHeader& hdr,
    uint32_t from,
    uint32_t target
) {
    std::set<uint32_t> seen;
    for (uint32_t at = from; at != nifly::NIF_NPOS;) {
        if (at == target)
            return true;
        auto c = hdr.GetBlock<nifly::NiTimeController>(at);
        if (!c || !seen.insert(at).second)
            return false;
        at = c->nextControllerRef.index;
    }
    return false;
}

// The block index in the `kind` ref of the block at `owner`; NIF_NPOS if it's
// empty or the block has no such ref.
inline uint32_t
block_ref(const nifly::NifFile& file, uint32_t owner, RefKind kind) {
    auto obj = file.GetHeader().GetBlock<nifly::NiObject>(owner);
    if (!obj)
        return nifly::NIF_NPOS;
    RefSlot slot = find_ref(*obj, kind);
    return slot.ref ? slot.ref->index : nifly::NIF_NPOS;
}

// Points the `kind` ref of the block at `owner` at `target`, or empties it if
// `target` is NIF_NPOS. Returns false, changing nothing, if the block has no
// such ref or `target` is the wrong type.
inline bool set_block_ref(
    nifly::NifFile& file,
    uint32_t owner,
    RefKind kind,
    uint32_t target
) {
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<nifly::NiObject>(owner);
    if (!obj)
        return false;
    // Entity slots may need creating, so check the target first.
    bool entity = kind == RefKind::EntityA || kind == RefKind::EntityB
        || kind == RefKind::WrappedEntityA || kind == RefKind::WrappedEntityB;
    if (entity && target != nifly::NIF_NPOS
        && !is_a<nifly::bhkEntity>(hdr.GetBlock<nifly::NiObject>(target)))
        return false;
    if (entity)
        make_entity_slots(*obj, kind);
    RefSlot slot = find_ref(*obj, kind);
    if (!slot.ref)
        return false;
    if (target != nifly::NIF_NPOS) {
        auto block = hdr.GetBlock<nifly::NiObject>(target);
        if (!block || !slot.target_ok(block))
            return false;
        if (kind == RefKind::HavokShape && in_shape_tree(hdr, target, owner))
            return false;
        if (kind == RefKind::SingleInterpolator
            && interpolator_uses(hdr, target, owner))
            return false;
        if (kind == RefKind::NextController
            && controller_chain_reaches(hdr, target, owner))
            return false;
        if (kind == RefKind::NextCollider
            && collider_chain_reaches(hdr, target, owner))
            return false;
        if (kind == RefKind::NextParticleModifier
            && particle_modifier_chain_reaches(hdr, target, owner))
            return false;
    }
    slot.ref->index = target;
    if (kind == RefKind::ParticleData)
        static_cast<nifly::NiParticleSystem*>(obj)->dataRef.index = target;
    return true;
}

// Points an NiGeometry's data ref at the NiGeometryData at `target`. Shapes
// that cache their data (NiTriShape, NiTriStrips, NiLines, BSLODTriShape and
// subtypes) only take their own data type, and the cache follows. Returns
// false, changing nothing, otherwise.
inline bool
set_shape_geometry_data(nifly::NifFile& file, uint32_t shape, uint32_t target) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto geom = hdr.GetBlock<NiGeometry>(shape);
    auto data = hdr.GetBlock<NiGeometryData>(target);
    if (!geom || !data)
        return false;
    bool cached = geom->HasType<NiTriShape>() || geom->HasType<NiTriStrips>()
        || geom->HasType<NiLines>() || geom->HasType<BSLODTriShape>();
    if (cached) {
        geom->SetGeomData(data);
        if (geom->GetGeomData() != data)
            return false;
    }
    geom->DataRef()->index = target;
    return true;
}

// Whether `target` is the node at `root` or in its node subtree, through
// child refs.
inline bool
in_node_subtree(const nifly::NiHeader& hdr, uint32_t root, uint32_t target) {
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {root};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == target)
            return true;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        if (auto node = hdr.GetBlock<nifly::NiNode>(id))
            for (uint32_t i = 0; i < node->childRefs.GetSize(); i++)
                stack.push_back(node->childRefs.GetBlockRef(i));
    }
    return false;
}
} // namespace nifly_rs
