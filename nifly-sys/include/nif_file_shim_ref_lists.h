// Part of nif_file_shim.h: lists of block refs, by RefList.
#pragma once

#include "nif_file_shim.h"
#include "nifly-sys/src/nif_file_refs.rs.h"

namespace nifly_rs {
template<class T>
rust::Vec<uint32_t> ref_array(const nifly::NiBlockRefArray<T>& refs) {
    rust::Vec<uint32_t> out;
    for (uint32_t i = 0; i < refs.GetSize(); i++)
        out.push_back(refs.GetBlockRef(i));
    return out;
}

template<class T>
void set_ref_array(
    nifly::NiBlockRefArray<T>& refs,
    rust::Slice<const uint32_t> targets
) {
    refs.Clear();
    for (uint32_t id : targets)
        refs.AddBlockRef(id);
}

// The `list` refs of the block at `owner`, in order, empty ones included.
// The extra data chain is followed from its head until a ref is empty, isn't
// extra data or repeats. Empty if the block has no such list.
inline rust::Vec<uint32_t>
block_ref_list(const nifly::NifFile& file, uint32_t owner, RefList list) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<NiObject>(owner);
    switch (list) {
        case RefList::Children:
            if (auto node = dynamic_cast<NiNode*>(obj))
                return ref_array(node->childRefs);
            break;
        case RefList::Properties:
            if (auto av = dynamic_cast<NiAVObject*>(obj))
                return ref_array(av->propertyRefs);
            break;
        case RefList::ExtraData:
            if (auto net = dynamic_cast<NiObjectNET*>(obj))
                return ref_array(net->extraDataRefs);
            break;
        case RefList::ExtraDataChain:
            if (auto net = dynamic_cast<NiObjectNET*>(obj)) {
                rust::Vec<uint32_t> out;
                std::set<uint32_t> seen;
                auto extra = hdr.GetBlock<NiExtraData>(net->extraDataRef);
                while (extra && seen.insert(hdr.GetBlockID(extra)).second) {
                    out.push_back(hdr.GetBlockID(extra));
                    extra = hdr.GetBlock<NiExtraData>(extra->nextExtraDataRef);
                }
                return out;
            }
            break;
        case RefList::Bones:
            if (auto skin = dynamic_cast<NiBoneContainer*>(obj))
                return ref_array(skin->boneRefs);
            break;
        case RefList::ParticleSystems:
            if (auto master = dynamic_cast<BSMasterParticleSystem*>(obj))
                return ref_array(master->particleSysRefs);
            break;
        case RefList::Constraints:
            if (auto body = dynamic_cast<bhkRigidBody*>(obj))
                return ref_array(body->constraintRefs);
            break;
        case RefList::SubShapes:
            if (auto list = dynamic_cast<bhkListShape*>(obj))
                return ref_array(list->subShapeRefs);
            if (auto list = dynamic_cast<bhkConvexListShape*>(obj))
                return ref_array(list->shapeRefs);
            break;
        case RefList::StripsData:
            if (auto shape = dynamic_cast<bhkNiTriStripsShape*>(obj))
                return ref_array(shape->partRefs);
            break;
        case RefList::ChainedEntities:
            if (auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(obj))
                return ref_array(chain->chainedEntityRefs);
            break;
        case RefList::RagdollBones:
            if (auto t = dynamic_cast<bhkRagdollTemplate*>(obj))
                return ref_array(t->boneRefs);
            break;
        case RefList::FlipSources:
            if (auto c = dynamic_cast<NiFlipController*>(obj))
                return ref_array(c->sourceRefs);
            break;
        case RefList::ExtraTargets:
            if (auto c = dynamic_cast<NiMultiTargetTransformController*>(obj))
                return ref_array(c->targetRefs);
            break;
        case RefList::Modifiers:
            if (auto system = dynamic_cast<NiParticleSystem*>(obj))
                return ref_array(system->modifierRefs);
            break;
        case RefList::ParticleMeshes:
            if (auto m = dynamic_cast<NiPSysMeshUpdateModifier*>(obj))
                return ref_array(m->meshRefs);
            if (auto m = dynamic_cast<NiPSysMeshEmitter*>(obj))
                return ref_array(m->meshRefs);
            if (auto m = dynamic_cast<NiParticleMeshModifier*>(obj))
                return ref_array(m->particleMeshRefs);
            break;
        case RefList::HavokUpdateNodes:
            if (auto m = dynamic_cast<BSPSysHavokUpdateModifier*>(obj))
                return ref_array(m->nodeRefs);
            break;
        case RefList::Colliders:
            if (auto manager = dynamic_cast<NiPSysColliderManager*>(obj)) {
                rust::Vec<uint32_t> out;
                std::set<uint32_t> seen;
                auto c = hdr.GetBlock<NiPSysCollider>(manager->colliderRef);
                while (c && seen.insert(hdr.GetBlockID(c)).second) {
                    out.push_back(hdr.GetBlockID(c));
                    c = hdr.GetBlock<NiPSysCollider>(c->nextColliderRef);
                }
                return out;
            }
            break;
        case RefList::ParticleModifiers:
            if (auto c = dynamic_cast<NiParticleSystemController*>(obj))
                return particle_modifier_chain<
                    NiParticleModifier,
                    NiParticleModifier>(hdr, c->particleModifierRef.index);
            break;
        case RefList::ParticleColliders:
            if (auto c = dynamic_cast<NiParticleSystemController*>(obj))
                return particle_modifier_chain<
                    NiParticleCollider,
                    NiParticleModifier>(hdr, c->particleColliderRef.index);
            break;
        case RefList::EmitterModifiers:
            if (auto c = dynamic_cast<NiParticleSystemController*>(obj))
                return particle_modifier_chain<
                    NiEmitterModifier,
                    NiEmitterModifier>(hdr, c->emitterModifierRef.index);
            break;
        case RefList::Controllers:
            if (auto net = dynamic_cast<NiObjectNET*>(obj)) {
                rust::Vec<uint32_t> out;
                std::set<uint32_t> seen;
                auto c = hdr.GetBlock<NiTimeController>(net->controllerRef);
                while (c && seen.insert(hdr.GetBlockID(c)).second) {
                    out.push_back(hdr.GetBlockID(c));
                    c = hdr.GetBlock<NiTimeController>(c->nextControllerRef);
                }
                return out;
            }
            break;
        default:
            break;
    }
    return {};
}

// The bone count of a skin's data, or -1 if it has none.
inline int64_t
skin_data_bone_count(const nifly::NiHeader& hdr, nifly::NiBoneContainer& skin) {
    if (auto inst = dynamic_cast<nifly::NiSkinInstance*>(&skin))
        if (auto data = hdr.GetBlock<nifly::NiSkinData>(inst->dataRef))
            return static_cast<int64_t>(data->bones.size());
    if (auto inst = dynamic_cast<nifly::BSSkinInstance*>(&skin))
        if (auto data = hdr.GetBlock<nifly::BSSkinBoneData>(inst->dataRef))
            return static_cast<int64_t>(data->boneXforms.size());
    return -1;
}

template<class T>
bool all_are(const nifly::NiHeader& hdr, rust::Slice<const uint32_t> ids) {
    for (uint32_t id : ids)
        if (!hdr.GetBlock<T>(id))
            return false;
    return true;
}

// Replaces the `list` refs of the block at `owner` with `targets`. Children
// must be NiAVObjects that aren't the node or above it; properties
// NiProperty; extra data NiExtraData or BSExtraData, and the chain
// NiExtraData, and the controller chain NiTimeController. Chains link each target to the
// next, so they must be distinct. Bones are nodes, as many as the skin's data
// has. Returns false, changing nothing, if a target doesn't fit or the block
// has no such list.
// Links distinct `targets` in order from `head`, through the next modifier refs
// of `Link`s. False, changing nothing, unless the first is a `Head` and all
// are `Link`s.
template<class Head, class Link>
bool set_particle_modifier_chain(
    nifly::NiHeader& hdr,
    nifly::NiRef& head,
    rust::Slice<const uint32_t> targets
) {
    if (!all_are<Link>(hdr, targets)
        || (!targets.empty() && !hdr.GetBlock<Head>(targets[0])))
        return false;
    std::set<uint32_t> distinct(targets.begin(), targets.end());
    if (distinct.size() != targets.size())
        return false;
    head.index = targets.empty() ? nifly::NIF_NPOS : targets[0];
    for (size_t i = 0; i < targets.size(); i++)
        hdr.GetBlock<Link>(targets[i])->nextModifierRef.index =
            i + 1 < targets.size() ? targets[i + 1] : nifly::NIF_NPOS;
    return true;
}

inline bool set_block_ref_list(
    nifly::NifFile& file,
    uint32_t owner,
    RefList list,
    rust::Slice<const uint32_t> targets
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<NiObject>(owner);
    switch (list) {
        case RefList::Children: {
            auto node = dynamic_cast<NiNode*>(obj);
            if (!node || !all_are<NiAVObject>(hdr, targets))
                return false;
            for (uint32_t id : targets)
                if (in_node_subtree(hdr, id, owner))
                    return false;
            set_ref_array(node->childRefs, targets);
            return true;
        }
        case RefList::Properties: {
            auto av = dynamic_cast<NiAVObject*>(obj);
            if (!av || !all_are<NiProperty>(hdr, targets))
                return false;
            set_ref_array(av->propertyRefs, targets);
            return true;
        }
        case RefList::ExtraData: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net)
                return false;
            for (uint32_t id : targets)
                if (!hdr.GetBlock<NiExtraData>(id)
                    && !hdr.GetBlock<BSExtraData>(id))
                    return false;
            set_ref_array(net->extraDataRefs, targets);
            return true;
        }
        case RefList::ExtraDataChain: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net || !all_are<NiExtraData>(hdr, targets))
                return false;
            std::set<uint32_t> distinct(targets.begin(), targets.end());
            if (distinct.size() != targets.size())
                return false;
            net->extraDataRef.index = targets.empty() ? NIF_NPOS : targets[0];
            for (size_t i = 0; i < targets.size(); i++)
                hdr.GetBlock<NiExtraData>(targets[i])->nextExtraDataRef.index =
                    i + 1 < targets.size() ? targets[i + 1] : NIF_NPOS;
            return true;
        }
        case RefList::Bones: {
            auto skin = dynamic_cast<NiBoneContainer*>(obj);
            if (!skin || !all_are<NiNode>(hdr, targets))
                return false;
            int64_t count = skin_data_bone_count(hdr, *skin);
            if (count >= 0 && size_t(count) != targets.size())
                return false;
            set_ref_array(skin->boneRefs, targets);
            return true;
        }
        case RefList::ParticleSystems: {
            auto master = dynamic_cast<BSMasterParticleSystem*>(obj);
            if (!master || !all_are<NiAVObject>(hdr, targets))
                return false;
            set_ref_array(master->particleSysRefs, targets);
            return true;
        }
        case RefList::Constraints: {
            auto body = dynamic_cast<bhkRigidBody*>(obj);
            if (!body)
                return false;
            for (uint32_t id : targets)
                if (!hdr.GetBlock<bhkConstraint>(id)
                    && !hdr.GetBlock<bhkBallSocketConstraintChain>(id))
                    return false;
            set_ref_array(body->constraintRefs, targets);
            return true;
        }
        case RefList::SubShapes: {
            for (uint32_t id : targets)
                if (in_shape_tree(hdr, id, owner))
                    return false;
            if (auto list = dynamic_cast<bhkListShape*>(obj)) {
                if (!all_are<bhkShape>(hdr, targets))
                    return false;
                if (list->filters.size() == list->subShapeRefs.GetSize())
                    list->filters.resize(static_cast<uint32_t>(targets.size()));
                set_ref_array(list->subShapeRefs, targets);
                return true;
            }
            if (auto list = dynamic_cast<bhkConvexListShape*>(obj)) {
                if (!all_are<bhkConvexShape>(hdr, targets))
                    return false;
                set_ref_array(list->shapeRefs, targets);
                return true;
            }
            return false;
        }
        case RefList::StripsData: {
            auto shape = dynamic_cast<bhkNiTriStripsShape*>(obj);
            if (!shape || !all_are<NiTriStripsData>(hdr, targets))
                return false;
            if (shape->filters.size() == shape->partRefs.GetSize())
                shape->filters.resize(static_cast<uint32_t>(targets.size()));
            set_ref_array(shape->partRefs, targets);
            return true;
        }
        case RefList::ChainedEntities: {
            auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(obj);
            if (!chain || !all_are<bhkRigidBody>(hdr, targets))
                return false;
            set_ref_array(chain->chainedEntityRefs, targets);
            return true;
        }
        case RefList::RagdollBones: {
            auto t = dynamic_cast<bhkRagdollTemplate*>(obj);
            if (!t || !all_are<bhkRagdollTemplateData>(hdr, targets))
                return false;
            set_ref_array(t->boneRefs, targets);
            return true;
        }
        case RefList::FlipSources: {
            auto c = dynamic_cast<NiFlipController*>(obj);
            if (!c || !all_are<NiSourceTexture>(hdr, targets))
                return false;
            set_ref_array(c->sourceRefs, targets);
            return true;
        }
        case RefList::ExtraTargets: {
            auto c = dynamic_cast<NiMultiTargetTransformController*>(obj);
            if (!c || targets.size() > UINT16_MAX)
                return false;
            for (uint32_t id : targets)
                if (id != NIF_NPOS && !hdr.GetBlock<NiAVObject>(id))
                    return false;
            set_ref_array(c->targetRefs, targets);
            return true;
        }
        case RefList::Modifiers: {
            auto system = dynamic_cast<NiParticleSystem*>(obj);
            if (!system || !all_are<NiPSysModifier>(hdr, targets))
                return false;
            set_ref_array(system->modifierRefs, targets);
            return true;
        }
        case RefList::ParticleMeshes: {
            if (!all_are<NiAVObject>(hdr, targets))
                return false;
            if (auto m = dynamic_cast<NiPSysMeshUpdateModifier*>(obj)) {
                set_ref_array(m->meshRefs, targets);
                return true;
            }
            if (auto m = dynamic_cast<NiPSysMeshEmitter*>(obj)) {
                set_ref_array(m->meshRefs, targets);
                return true;
            }
            if (auto m = dynamic_cast<NiParticleMeshModifier*>(obj)) {
                set_ref_array(m->particleMeshRefs, targets);
                return true;
            }
            return false;
        }
        case RefList::HavokUpdateNodes: {
            auto m = dynamic_cast<BSPSysHavokUpdateModifier*>(obj);
            if (!m || !all_are<NiNode>(hdr, targets))
                return false;
            set_ref_array(m->nodeRefs, targets);
            return true;
        }
        case RefList::Colliders: {
            auto manager = dynamic_cast<NiPSysColliderManager*>(obj);
            if (!manager || !all_are<NiPSysCollider>(hdr, targets))
                return false;
            std::set<uint32_t> distinct(targets.begin(), targets.end());
            if (distinct.size() != targets.size())
                return false;
            manager->colliderRef.index =
                targets.empty() ? NIF_NPOS : targets[0];
            for (size_t i = 0; i < targets.size(); i++)
                hdr.GetBlock<NiPSysCollider>(targets[i])
                    ->nextColliderRef.index =
                    i + 1 < targets.size() ? targets[i + 1] : NIF_NPOS;
            return true;
        }
        case RefList::ParticleModifiers:
        case RefList::ParticleColliders:
        case RefList::EmitterModifiers: {
            auto c = dynamic_cast<NiParticleSystemController*>(obj);
            if (!c)
                return false;
            if (list == RefList::EmitterModifiers)
                return set_particle_modifier_chain<
                    NiEmitterModifier,
                    NiEmitterModifier>(hdr, c->emitterModifierRef, targets);
            if (list == RefList::ParticleColliders)
                return set_particle_modifier_chain<
                    NiParticleCollider,
                    NiParticleModifier>(hdr, c->particleColliderRef, targets);
            return set_particle_modifier_chain<
                NiParticleModifier,
                NiParticleModifier>(hdr, c->particleModifierRef, targets);
        }
        case RefList::Controllers: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net || !all_are<NiTimeController>(hdr, targets))
                return false;
            std::set<uint32_t> distinct(targets.begin(), targets.end());
            if (distinct.size() != targets.size())
                return false;
            net->controllerRef.index = targets.empty() ? NIF_NPOS : targets[0];
            for (size_t i = 0; i < targets.size(); i++)
                hdr.GetBlock<NiTimeController>(targets[i])
                    ->nextControllerRef.index =
                    i + 1 < targets.size() ? targets[i + 1] : NIF_NPOS;
            return true;
        }
        default:
            return false;
    }
}
} // namespace nifly_rs
