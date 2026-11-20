//! Shader flags of each game.

use super::*;

/// A game's meaning of [`BsShaderProperty::flags_1`]: [`Fallout3ShaderFlags1`],
/// [`SkyrimShaderFlags1`] or [`Fallout4ShaderFlags1`].
pub trait ShaderFlags1: Flags<Bits = u32> + Sealed {}

/// A game's meaning of [`BsShaderProperty::flags_2`]: [`Fallout3ShaderFlags2`],
/// [`SkyrimShaderFlags2`] or [`Fallout4ShaderFlags2`].
pub trait ShaderFlags2: Flags<Bits = u32> + Sealed {}

bitflags! {
    /// [`BsShaderProperty::flags_1`] as Fallout 3 and New Vegas uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Fallout3ShaderFlags1: u32 {
        const SPECULAR = 1;
        const SKINNED = 1 << 1;
        const LOW_DETAIL = 1 << 2;
        const VERTEX_ALPHA = 1 << 3;
        const SINGLE_PASS = 1 << 5;
        const ENVIRONMENT_MAPPING = 1 << 7;
        const ALPHA_TEXTURE = 1 << 8;
        const FACE_GEN = 1 << 10;
        const PARALLAX = 1 << 11;
        const NON_PROJECTIVE_SHADOWS = 1 << 13;
        const REFRACTION = 1 << 15;
        const FIRE_REFRACTION = 1 << 16;
        const EYE_ENVIRONMENT_MAPPING = 1 << 17;
        const HAIR = 1 << 18;
        const DYNAMIC_ALPHA = 1 << 19;
        const LOCAL_MAP_HIDE_SECRET = 1 << 20;
        const WINDOW_ENVIRONMENT_MAPPING = 1 << 21;
        const TREE_BILLBOARD = 1 << 22;
        const SHADOW_FRUSTUM = 1 << 23;
        const MULTIPLE_TEXTURES = 1 << 24;
        const REMAPPABLE_TEXTURES = 1 << 25;
        const DECAL = 1 << 26;
        const DYNAMIC_DECAL = 1 << 27;
        const PARALLAX_OCCLUSION = 1 << 28;
        const EXTERNAL_EMITTANCE = 1 << 29;
        const SHADOW_MAP = 1 << 30;
        const Z_BUFFER_TEST = 1 << 31;
        const _ = !0;
    }
}

impl Sealed for Fallout3ShaderFlags1 {}
impl ShaderFlags1 for Fallout3ShaderFlags1 {}

bitflags! {
    /// [`BsShaderProperty::flags_2`] as Fallout 3 and New Vegas uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Fallout3ShaderFlags2: u32 {
        const Z_BUFFER_WRITE = 1;
        const LOD_LANDSCAPE = 1 << 1;
        const LOD_BUILDING = 1 << 2;
        const NO_FADE = 1 << 3;
        const REFRACTION_TINT = 1 << 4;
        const VERTEX_COLORS = 1 << 5;
        const FIRST_LIGHT_IS_POINT_LIGHT = 1 << 7;
        const SECOND_LIGHT = 1 << 8;
        const THIRD_LIGHT = 1 << 9;
        const VERTEX_LIGHTING = 1 << 10;
        const UNIFORM_SCALE = 1 << 11;
        const FIT_SLOPE = 1 << 12;
        const BILLBOARD_AND_ENVMAP_LIGHT_FADE = 1 << 13;
        const NO_LOD_LAND_BLEND = 1 << 14;
        const ENVMAP_LIGHT_FADE = 1 << 15;
        const WIREFRAME = 1 << 16;
        const VATS_SELECTION = 1 << 17;
        const SHOW_IN_LOCAL_MAP = 1 << 18;
        const PREMULT_ALPHA = 1 << 19;
        const SKIP_NORMAL_MAPS = 1 << 20;
        const ALPHA_DECAL = 1 << 21;
        const NO_TRANSPARENCY_MULTISAMPLING = 1 << 22;
        const _ = !0;
    }
}

impl Sealed for Fallout3ShaderFlags2 {}
impl ShaderFlags2 for Fallout3ShaderFlags2 {}

bitflags! {
    /// [`BsShaderProperty::flags_1`] as Skyrim (LE and SE) uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SkyrimShaderFlags1: u32 {
        const SPECULAR = 1;
        const SKINNED = 1 << 1;
        const TEMP_REFRACTION = 1 << 2;
        const VERTEX_ALPHA = 1 << 3;
        const GREYSCALE_TO_PALETTE_COLOR = 1 << 4;
        const GREYSCALE_TO_PALETTE_ALPHA = 1 << 5;
        const USE_FALLOFF = 1 << 6;
        const ENVIRONMENT_MAPPING = 1 << 7;
        const RECEIVE_SHADOWS = 1 << 8;
        const CAST_SHADOWS = 1 << 9;
        const FACE_GEN_DETAIL_MAP = 1 << 10;
        const PARALLAX = 1 << 11;
        const MODEL_SPACE_NORMALS = 1 << 12;
        const NON_PROJECTIVE_SHADOWS = 1 << 13;
        const LANDSCAPE = 1 << 14;
        const REFRACTION = 1 << 15;
        const FIRE_REFRACTION = 1 << 16;
        const EYE_ENVIRONMENT_MAPPING = 1 << 17;
        const HAIR_SOFT_LIGHTING = 1 << 18;
        const SCREENDOOR_ALPHA_FADE = 1 << 19;
        const LOCAL_MAP_HIDE_SECRET = 1 << 20;
        const FACE_GEN_RGB_TINT = 1 << 21;
        const OWN_EMIT = 1 << 22;
        const PROJECTED_UV = 1 << 23;
        const MULTIPLE_TEXTURES = 1 << 24;
        const REMAPPABLE_TEXTURES = 1 << 25;
        const DECAL = 1 << 26;
        const DYNAMIC_DECAL = 1 << 27;
        const PARALLAX_OCCLUSION = 1 << 28;
        const EXTERNAL_EMITTANCE = 1 << 29;
        const SOFT_EFFECT = 1 << 30;
        const Z_BUFFER_TEST = 1 << 31;
        const _ = !0;
    }
}

impl Sealed for SkyrimShaderFlags1 {}
impl ShaderFlags1 for SkyrimShaderFlags1 {}

bitflags! {
    /// [`BsShaderProperty::flags_2`] as Skyrim (LE and SE) uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SkyrimShaderFlags2: u32 {
        const Z_BUFFER_WRITE = 1;
        const LOD_LANDSCAPE = 1 << 1;
        const LOD_OBJECTS = 1 << 2;
        const NO_FADE = 1 << 3;
        const DOUBLE_SIDED = 1 << 4;
        const VERTEX_COLORS = 1 << 5;
        const GLOW_MAP = 1 << 6;
        const ASSUME_SHADOWMASK = 1 << 7;
        const PACKED_TANGENT = 1 << 8;
        const MULTI_INDEX_SNOW = 1 << 9;
        const VERTEX_LIGHTING = 1 << 10;
        const UNIFORM_SCALE = 1 << 11;
        const FIT_SLOPE = 1 << 12;
        const BILLBOARD = 1 << 13;
        const NO_LOD_LAND_BLEND = 1 << 14;
        const ENVMAP_LIGHT_FADE = 1 << 15;
        const WIREFRAME = 1 << 16;
        const WEAPON_BLOOD = 1 << 17;
        const HIDE_ON_LOCAL_MAP = 1 << 18;
        const PREMULT_ALPHA = 1 << 19;
        const CLOUD_LOD = 1 << 20;
        const ANISOTROPIC_LIGHTING = 1 << 21;
        const NO_TRANSPARENCY_MULTISAMPLING = 1 << 22;
        const MULTI_LAYER_PARALLAX = 1 << 24;
        const SOFT_LIGHTING = 1 << 25;
        const RIM_LIGHTING = 1 << 26;
        const BACK_LIGHTING = 1 << 27;
        const TREE_ANIM = 1 << 29;
        const EFFECT_LIGHTING = 1 << 30;
        const HD_LOD_OBJECTS = 1 << 31;
        const _ = !0;
    }
}

impl Sealed for SkyrimShaderFlags2 {}
impl ShaderFlags2 for SkyrimShaderFlags2 {}

bitflags! {
    /// [`BsShaderProperty::flags_1`] as Fallout 4 uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Fallout4ShaderFlags1: u32 {
        const SPECULAR = 1;
        const SKINNED = 1 << 1;
        const TEMP_REFRACTION = 1 << 2;
        const VERTEX_ALPHA = 1 << 3;
        const GREYSCALE_TO_PALETTE_COLOR = 1 << 4;
        const GREYSCALE_TO_PALETTE_ALPHA = 1 << 5;
        const USE_FALLOFF = 1 << 6;
        const ENVIRONMENT_MAPPING = 1 << 7;
        const RGB_FALLOFF = 1 << 8;
        const CAST_SHADOWS = 1 << 9;
        const FACE = 1 << 10;
        const UI_MASK_RECTS = 1 << 11;
        const MODEL_SPACE_NORMALS = 1 << 12;
        const NON_PROJECTIVE_SHADOWS = 1 << 13;
        const LANDSCAPE = 1 << 14;
        const REFRACTION = 1 << 15;
        const FIRE_REFRACTION = 1 << 16;
        const EYE_ENVIRONMENT_MAPPING = 1 << 17;
        const HAIR = 1 << 18;
        const SCREENDOOR_ALPHA_FADE = 1 << 19;
        const LOCAL_MAP_HIDE_SECRET = 1 << 20;
        const SKIN_TINT = 1 << 21;
        const OWN_EMIT = 1 << 22;
        const PROJECTED_UV = 1 << 23;
        const MULTIPLE_TEXTURES = 1 << 24;
        const TESSELLATE = 1 << 25;
        const DECAL = 1 << 26;
        const DYNAMIC_DECAL = 1 << 27;
        const CHARACTER_LIGHTING = 1 << 28;
        const EXTERNAL_EMITTANCE = 1 << 29;
        const SOFT_EFFECT = 1 << 30;
        const Z_BUFFER_TEST = 1 << 31;
        const _ = !0;
    }
}

impl Sealed for Fallout4ShaderFlags1 {}
impl ShaderFlags1 for Fallout4ShaderFlags1 {}

bitflags! {
    /// [`BsShaderProperty::flags_2`] as Fallout 4 uses it. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Fallout4ShaderFlags2: u32 {
        const Z_BUFFER_WRITE = 1;
        const LOD_LANDSCAPE = 1 << 1;
        const LOD_OBJECTS = 1 << 2;
        const NO_FADE = 1 << 3;
        const DOUBLE_SIDED = 1 << 4;
        const VERTEX_COLORS = 1 << 5;
        const GLOW_MAP = 1 << 6;
        const TRANSFORM_CHANGED = 1 << 7;
        const DISMEMBERMENT_MEATCUFF = 1 << 8;
        const TINT = 1 << 9;
        const GRASS_VERTEX_LIGHTING = 1 << 10;
        const GRASS_UNIFORM_SCALE = 1 << 11;
        const GRASS_FIT_SLOPE = 1 << 12;
        const GRASS_BILLBOARD = 1 << 13;
        const NO_LOD_LAND_BLEND = 1 << 14;
        const DISMEMBERMENT = 1 << 15;
        const WIREFRAME = 1 << 16;
        const WEAPON_BLOOD = 1 << 17;
        const HIDE_ON_LOCAL_MAP = 1 << 18;
        const PREMULT_ALPHA = 1 << 19;
        const VATS_TARGET = 1 << 20;
        const ANISOTROPIC_LIGHTING = 1 << 21;
        const SKEW_SPECULAR_ALPHA = 1 << 22;
        const MENU_SCREEN = 1 << 23;
        const MULTI_LAYER_PARALLAX = 1 << 24;
        const ALPHA_TEST = 1 << 25;
        const GRADIENT_REMAP = 1 << 26;
        const VATS_TARGET_DRAW_ALL = 1 << 27;
        const PIPBOY_SCREEN = 1 << 28;
        const TREE_ANIM = 1 << 29;
        const EFFECT_LIGHTING = 1 << 30;
        const REFRACTION_WRITES_DEPTH = 1 << 31;
        const _ = !0;
    }
}

impl Sealed for Fallout4ShaderFlags2 {}
impl ShaderFlags2 for Fallout4ShaderFlags2 {}
