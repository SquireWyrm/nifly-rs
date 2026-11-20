// References to private fields of nifly classes that have no accessors.
//
// An explicit instantiation may name a private member, so `Steal` turns a
// pointer to one into a friend function found by argument-dependent lookup. The
// instantiations must appear once in the program, hence this file instead of a
// header.

#include "Animation.hpp"

namespace {
template<class Tag, typename Tag::type Member>
struct Steal {
    friend typename Tag::type member(Tag) {
        return Member;
    }
};

#define NIFLY_RS_PRIVATE(Class, Type, field) \
    struct Class##_##field { \
        using type = Type nifly::Class::*; \
        friend type member(Class##_##field); \
    }; \
    template struct Steal<Class##_##field, &nifly::Class::field>;

NIFLY_RS_PRIVATE(NiPathInterpolator, nifly::PathFlags, pathFlags)
NIFLY_RS_PRIVATE(NiPathInterpolator, int, bankDir)
NIFLY_RS_PRIVATE(NiPathInterpolator, float, maxBankAngle)
NIFLY_RS_PRIVATE(NiPathInterpolator, float, smoothing)
NIFLY_RS_PRIVATE(NiPathInterpolator, uint16_t, followAxis)
NIFLY_RS_PRIVATE(
    NiPathInterpolator,
    nifly::NiBlockRef<nifly::NiPosData>,
    pathDataRef
)
NIFLY_RS_PRIVATE(
    NiPathInterpolator,
    nifly::NiBlockRef<nifly::NiFloatData>,
    percentDataRef
)
} // namespace

namespace nifly_rs {
nifly::PathFlags& path_flags_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_pathFlags {});
}

int& path_bank_dir_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_bankDir {});
}

float& path_max_bank_angle_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_maxBankAngle {});
}

float& path_smoothing_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_smoothing {});
}

uint16_t& path_follow_axis_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_followAxis {});
}

nifly::NiBlockRef<nifly::NiPosData>&
path_data_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_pathDataRef {});
}

nifly::NiBlockRef<nifly::NiFloatData>&
path_percent_field(nifly::NiPathInterpolator& p) {
    return p.*member(NiPathInterpolator_percentDataRef {});
}
} // namespace nifly_rs
