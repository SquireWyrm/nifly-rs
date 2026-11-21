// Part of animation_shim.h: controller types and adding controllers.
#pragma once

#include "animation_shim.h"

namespace nifly_rs {
// Controller types.

NIFLY_RS_ANIMATION_FIELD(
    NiLookAtController,
    uint16_t,
    lookAtFlags,
    look_at_controller_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    uint16_t,
    pathFlags,
    path_controller_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    int32_t,
    bankDir,
    path_controller_bank_direction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    float,
    maxBankAngle,
    path_controller_max_bank_angle
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    float,
    smoothing,
    path_controller_smoothing
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    uint16_t,
    followAxis,
    path_controller_follow_axis
)
NIFLY_RS_ANIMATION_FIELD(
    NiUVController,
    uint16_t,
    textureSet,
    uv_controller_uv_set
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    linearVelocity,
    lag_bone_linear_velocity
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    linearRotation,
    lag_bone_linear_rotation
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    maxDistance,
    lag_bone_max_distance
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    subdivisions,
    lightning_subdivisions
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    numBranches,
    lightning_branch_count
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    numBranchesPerVariation,
    lightning_branch_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    length,
    lightning_length
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    lengthVariation,
    lightning_length_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    width,
    lightning_width
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    childWidthMult,
    lightning_child_width_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    arcOffset,
    lightning_arc_offset
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    fadeMainBolt,
    lightning_fade_main_bolt
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    fadeChildBolts,
    lightning_fade_child_bolts
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    animateArcOffset,
    lightning_animate_arc_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiPoint3InterpController,
    uint16_t,
    targetColor,
    point3_controller_target_color
)
NIFLY_RS_ANIMATION_FIELD(
    NiFlipController,
    uint32_t,
    textureSlot,
    flip_texture_slot
)
NIFLY_RS_ANIMATION_FIELD(NiFlipController, float, accumTime, flip_accum_time)
NIFLY_RS_ANIMATION_FIELD(NiFlipController, float, delta, flip_delta)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    bool,
    shaderMap,
    texture_transform_shader_map
)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    uint32_t,
    textureSlot,
    texture_transform_slot
)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    uint32_t,
    operation,
    texture_transform_operation
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyColorController,
    uint32_t,
    typeOfControlledColor,
    lighting_color_controller_color
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyFloatController,
    uint32_t,
    typeOfControlledVariable,
    lighting_float_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyUShortController,
    uint32_t,
    typeOfControlledVariable,
    lighting_ushort_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    BSEffectShaderPropertyColorController,
    uint32_t,
    typeOfControlledColor,
    effect_color_controller_color
)
NIFLY_RS_ANIMATION_FIELD(
    BSEffectShaderPropertyFloatController,
    uint32_t,
    typeOfControlledVariable,
    effect_float_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    NiGeomMorpherController,
    uint16_t,
    morpherFlags,
    geom_morpher_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiGeomMorpherController,
    bool,
    alwaysUpdate,
    geom_morpher_always_update
)
NIFLY_RS_ANIMATION_FIELD(NiBoneLODController, uint32_t, lod, bone_lod_lod)
NIFLY_RS_ANIMATION_FIELD(
    bhkBlendController,
    uint32_t,
    keys,
    blend_controller_keys
)

inline uint32_t
bone_lod_controller_count(const nifly::NiBoneLODController& obj) {
    return obj.numLODs;
}

inline rust::Slice<const uint8_t>
float_extra_data_controller_name(const nifly::NiFloatExtraDataController& obj) {
    auto& name = obj.extraData.get();
    return {reinterpret_cast<const uint8_t*>(name.data()), name.size()};
}

inline void set_float_extra_data_controller_name(
    nifly::NiFloatExtraDataController& obj,
    rust::Slice<const uint8_t> name
) {
    obj.extraData.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

inline rust::Vec<uint32_t>
geom_morpher_unknown_ints(const nifly::NiGeomMorpherController& obj) {
    rust::Vec<uint32_t> out;
    for (uint32_t v : unconst(obj).unknownInts)
        out.push_back(v);
    return out;
}

inline void set_geom_morpher_unknown_ints(
    nifly::NiGeomMorpherController& obj,
    rust::Slice<const uint32_t> ints
) {
    obj.unknownInts.clear();
    for (uint32_t v : ints)
        obj.unknownInts.push_back(v);
}

// The interpolators of the NiGeomMorpherController at `owner`, from the list
// its file version saves: weighted from 20.1.0.3 on, plain refs before (with
// the weights kept in memory, or 0). Empty if the block isn't one.
inline rust::Vec<MorphWeightFfi>
morph_weights(const nifly::NifFile& file, uint32_t owner) {
    using namespace nifly;
    rust::Vec<MorphWeightFfi> out;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiGeomMorpherController>(owner);
    if (!c)
        return out;
    if (hdr.GetVersion().File() >= NiFileVersion::V20_1_0_3) {
        for (auto& w : c->interpWeights)
            out.push_back({w.interpRef.index, w.weight});
        return out;
    }
    for (uint32_t i = 0; i < c->interpolatorRefs.GetSize(); i++) {
        float weight =
            i < c->interpWeights.size() ? c->interpWeights[i].weight : 0.0f;
        out.push_back({c->interpolatorRefs.GetBlockRef(i), weight});
    }
    return out;
}

// Replaces both interpolator lists of the NiGeomMorpherController at `owner`.
// False, changing nothing, unless each interpolator is empty or an
// NiInterpolator.
inline bool set_morph_weights(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const MorphWeightFfi> weights
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiGeomMorpherController>(owner);
    if (!c)
        return false;
    for (auto& w : weights)
        if (w.interpolator != NIF_NPOS
            && !hdr.GetBlock<NiInterpolator>(w.interpolator))
            return false;
    c->interpolatorRefs.Clear();
    c->interpWeights.clear();
    for (auto& w : weights) {
        c->interpolatorRefs.AddBlockRef(w.interpolator);
        MorphWeight m;
        m.interpRef.index = w.interpolator;
        m.weight = w.weight;
        c->interpWeights.push_back(m);
    }
    return true;
}

inline rust::Vec<NodeGroupFfi>
bone_lod_groups(const nifly::NiBoneLODController& obj) {
    rust::Vec<NodeGroupFfi> out;
    for (auto& group : unconst(obj).boneArrays) {
        NodeGroupFfi g;
        for (uint32_t i = 0; i < group.GetSize(); i++)
            g.nodes.push_back(group.GetBlockRef(i));
        out.push_back(std::move(g));
    }
    return out;
}

// Replaces the node groups of the NiBoneLODController at `owner` and sets its
// LOD count to their number. False, changing nothing, unless each node is an
// NiNode.
inline bool set_bone_lod_groups(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const NodeGroupFfi> groups
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiBoneLODController>(owner);
    if (!c)
        return false;
    for (auto& g : groups)
        for (uint32_t id : g.nodes)
            if (!hdr.GetBlock<NiNode>(id))
                return false;
    c->boneArrays.clear();
    for (auto& g : groups) {
        NiBlockPtrArray<NiNode> nodes;
        for (uint32_t id : g.nodes)
            nodes.AddBlockRef(id);
        c->boneArrays.push_back(nodes);
    }
    c->numLODs = static_cast<uint32_t>(groups.size());
    return true;
}

// NiFloatExtraDataController reads and writes the extra data name in every
// version, but files store it only from 10.2.0.0 on: older files are misread
// from there. FloatExtraDataController syncs it by version;
// FloatExtraDataControllerFactory loads every NiFloatExtraDataController as
// one, and add_ni_float_extra_data_controller creates them.
class FloatExtraDataController final: public nifly::NiFloatExtraDataController {
  public:
    void Get(nifly::NiIStream& stream) override {
        nifly::NiExtraDataController::Get(stream);
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0)
            extraData.Read(stream);
    }

    void Put(nifly::NiOStream& stream) override {
        nifly::NiExtraDataController::Put(stream);
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0)
            extraData.Write(stream);
    }

  private:
    FloatExtraDataController* Clone_impl() const override {
        return new FloatExtraDataController(*this);
    }
};

class FloatExtraDataControllerFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<FloatExtraDataController>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto c = std::make_unique<FloatExtraDataController>();
        c->Get(stream);
        return c;
    }
};

// Controllers.

NIFLY_RS_ANIMATION_FIELD(NiTimeController, uint16_t, flags, controller_flags)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    frequency,
    controller_frequency
)
NIFLY_RS_ANIMATION_FIELD(NiTimeController, float, phase, controller_phase)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    startTime,
    controller_start_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    stopTime,
    controller_stop_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiInterpController,
    bool,
    managerControlled,
    interp_controller_manager_controlled
)

// Appends `controller` to the controller chain of the NiObjectNET at `owner`
// and points its target at the owner. A ref that isn't a controller, or that
// repeats one already passed, ends the chain and is replaced.
inline void append_controller(
    nifly::NiHeader& hdr,
    nifly::NiObjectNET& owner,
    uint32_t owner_index,
    nifly::NiTimeController& controller,
    uint32_t index
) {
    using namespace nifly;
    controller.targetRef.index = owner_index;
    NiRef* link = &owner.controllerRef;
    std::set<uint32_t> seen;
    while (auto c = hdr.GetBlock<NiTimeController>(link->index)) {
        if (!seen.insert(link->index).second)
            break;
        link = &c->nextControllerRef;
    }
    link->index = index;
}

// Adds a new T to the end of the controller chain of the NiObjectNET at
// `owner`, targeting it. NIF_NPOS, adding nothing, if `owner` isn't one.
#define NIFLY_RS_ADD_CONTROLLER_AS(T, Make, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file, uint32_t owner) { \
        auto& hdr = file.GetHeader(); \
        auto net = hdr.GetBlock<nifly::NiObjectNET>(owner); \
        if (!net) \
            return nifly::NIF_NPOS; \
        auto block = std::make_unique<Make>(); \
        auto& controller = *block; \
        uint32_t index = hdr.AddBlock(std::move(block)); \
        append_controller(hdr, *net, owner, controller, index); \
        return index; \
    }
#define NIFLY_RS_ADD_CONTROLLER(T, snake) \
    NIFLY_RS_ADD_CONTROLLER_AS(T, nifly::T, snake)

NIFLY_RS_ADD_CONTROLLER(NiLookAtController, ni_look_at_controller)
NIFLY_RS_ADD_CONTROLLER(NiPathController, ni_path_controller)
NIFLY_RS_ADD_CONTROLLER(NiUVController, ni_uv_controller)
NIFLY_RS_ADD_CONTROLLER(BSFrustumFOVController, bs_frustum_fov_controller)
NIFLY_RS_ADD_CONTROLLER(BSLagBoneController, bs_lag_bone_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSProceduralLightningController,
    bs_procedural_lightning_controller
)
NIFLY_RS_ADD_CONTROLLER(NiBoneLODController, ni_bone_lod_controller)
NIFLY_RS_ADD_CONTROLLER(NiBSBoneLODController, ni_bs_bone_lod_controller)
NIFLY_RS_ADD_CONTROLLER(NiGeomMorpherController, ni_geom_morpher_controller)
NIFLY_RS_ADD_CONTROLLER(NiRollController, ni_roll_controller)
NIFLY_RS_ADD_CONTROLLER(NiMaterialColorController, ni_material_color_controller)
NIFLY_RS_ADD_CONTROLLER(NiLightColorController, ni_light_color_controller)
NIFLY_RS_ADD_CONTROLLER_AS(
    NiFloatExtraDataController,
    FloatExtraDataController,
    ni_float_extra_data_controller
)
NIFLY_RS_ADD_CONTROLLER(bhkBlendController, bhk_blend_controller)
NIFLY_RS_ADD_CONTROLLER(NiVisController, ni_vis_controller)
NIFLY_RS_ADD_CONTROLLER(NiFlipController, ni_flip_controller)
NIFLY_RS_ADD_CONTROLLER(
    NiTextureTransformController,
    ni_texture_transform_controller
)
NIFLY_RS_ADD_CONTROLLER(NiLightDimmerController, ni_light_dimmer_controller)
NIFLY_RS_ADD_CONTROLLER(NiLightRadiusController, ni_light_radius_controller)
NIFLY_RS_ADD_CONTROLLER(NiAlphaController, ni_alpha_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSNiAlphaPropertyTestRefController,
    bs_ni_alpha_property_test_ref_controller
)
NIFLY_RS_ADD_CONTROLLER(NiKeyframeController, ni_keyframe_controller)
NIFLY_RS_ADD_CONTROLLER(NiTransformController, ni_transform_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSMaterialEmittanceMultController,
    bs_material_emittance_mult_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSRefractionStrengthController,
    bs_refraction_strength_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSRefractionFirePeriodController,
    bs_refraction_fire_period_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyColorController,
    bs_lighting_shader_property_color_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyFloatController,
    bs_lighting_shader_property_float_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyUShortController,
    bs_lighting_shader_property_u_short_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSEffectShaderPropertyColorController,
    bs_effect_shader_property_color_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSEffectShaderPropertyFloatController,
    bs_effect_shader_property_float_controller
)
NIFLY_RS_ADD_CONTROLLER(
    NiMultiTargetTransformController,
    ni_multi_target_transform_controller
)
} // namespace nifly_rs
