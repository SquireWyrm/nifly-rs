//! `NiDynamicEffect` and its subclasses: lights and texture effects.
//!
//! For each type `T` with direct base `Parent`:
//! - `upcast_<t>(&T) -> &Parent`
//! - `downcast_<t>(&NiObject) -> *const T`. Null unless the object is a `T` or a subclass of `T`.

#[cxx::bridge]
pub mod ffi {
    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Objects.hpp");

        type NiDynamicEffect;
        type NiTextureEffect;
        type NiLight;
        type NiAmbientLight;
        type NiDirectionalLight;
        type NiPointLight;
        type NiSpotLight;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("NifFile.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiAVObject = crate::objects::ffi::NiAVObject;
        type NiNode = crate::nodes::ffi::NiNode;
        type NifFile = crate::nif_file::ffi::NifFile;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("effects_shim.h");

        fn upcast_ni_dynamic_effect(obj: &NiDynamicEffect) -> &NiAVObject;
        fn downcast_ni_dynamic_effect(obj: &NiObject) -> *const NiDynamicEffect;
        fn upcast_ni_texture_effect(obj: &NiTextureEffect) -> &NiDynamicEffect;
        fn downcast_ni_texture_effect(obj: &NiObject) -> *const NiTextureEffect;
        fn upcast_ni_light(obj: &NiLight) -> &NiDynamicEffect;
        fn downcast_ni_light(obj: &NiObject) -> *const NiLight;
        fn upcast_ni_ambient_light(obj: &NiAmbientLight) -> &NiLight;
        fn downcast_ni_ambient_light(obj: &NiObject) -> *const NiAmbientLight;
        fn upcast_ni_directional_light(obj: &NiDirectionalLight) -> &NiLight;
        fn downcast_ni_directional_light(obj: &NiObject) -> *const NiDirectionalLight;
        fn upcast_ni_point_light(obj: &NiPointLight) -> &NiLight;
        fn downcast_ni_point_light(obj: &NiObject) -> *const NiPointLight;
        fn upcast_ni_spot_light(obj: &NiSpotLight) -> &NiPointLight;
        fn downcast_ni_spot_light(obj: &NiObject) -> *const NiSpotLight;

        /// `NiDynamicEffect` fields. `affectedNodes` are block indices (`NIF_NPOS` if empty);
        /// `affectedNodePointers` are raw pointer hashes.
        fn effect_switch_state(e: &NiDynamicEffect) -> bool;
        fn set_effect_switch_state(e: Pin<&mut NiDynamicEffect>, v: bool);
        fn effect_affected_nodes(e: &NiDynamicEffect) -> Vec<u32>;
        fn effect_affected_node_pointers(e: &NiDynamicEffect) -> Vec<u32>;
        fn set_effect_affected_node_pointers(e: Pin<&mut NiDynamicEffect>, v: &[u32]);

        /// `NiTextureEffect` fields. The matrix is 3 rows; enums are raw values;
        /// `sourceTexture` is a block index (`NIF_NPOS` if empty).
        fn texture_effect_matrix(e: &NiTextureEffect, out: &mut [Vec3]);
        fn set_texture_effect_matrix(e: Pin<&mut NiTextureEffect>, rows: &[Vec3]);
        fn texture_effect_translation(e: &NiTextureEffect) -> Vec3;
        fn set_texture_effect_translation(e: Pin<&mut NiTextureEffect>, v: Vec3);
        fn texture_effect_filtering(e: &NiTextureEffect) -> u32;
        fn set_texture_effect_filtering(e: Pin<&mut NiTextureEffect>, v: u32);
        fn texture_effect_clamping(e: &NiTextureEffect) -> u32;
        fn set_texture_effect_clamping(e: Pin<&mut NiTextureEffect>, v: u32);
        fn texture_effect_type(e: &NiTextureEffect) -> u32;
        fn set_texture_effect_type(e: Pin<&mut NiTextureEffect>, v: u32);
        fn texture_effect_coordinates(e: &NiTextureEffect) -> u32;
        fn set_texture_effect_coordinates(e: Pin<&mut NiTextureEffect>, v: u32);
        fn texture_effect_source(e: &NiTextureEffect) -> u32;
        fn texture_effect_clipping(e: &NiTextureEffect) -> u8;
        fn set_texture_effect_clipping(e: Pin<&mut NiTextureEffect>, v: u8);
        fn texture_effect_plane_normal(e: &NiTextureEffect) -> Vec3;
        fn set_texture_effect_plane_normal(e: Pin<&mut NiTextureEffect>, v: Vec3);
        fn texture_effect_plane_constant(e: &NiTextureEffect) -> f32;
        fn set_texture_effect_plane_constant(e: Pin<&mut NiTextureEffect>, v: f32);
        fn texture_effect_ps2_l(e: &NiTextureEffect) -> i16;
        fn set_texture_effect_ps2_l(e: Pin<&mut NiTextureEffect>, v: i16);
        fn texture_effect_ps2_k(e: &NiTextureEffect) -> i16;
        fn set_texture_effect_ps2_k(e: Pin<&mut NiTextureEffect>, v: i16);
        fn texture_effect_unknown(e: &NiTextureEffect) -> u16;
        fn set_texture_effect_unknown(e: Pin<&mut NiTextureEffect>, v: u16);

        /// Light fields. Colors are r, g, b.
        fn light_dimmer(l: &NiLight) -> f32;
        fn set_light_dimmer(l: Pin<&mut NiLight>, v: f32);
        fn light_ambient(l: &NiLight) -> Vec3;
        fn set_light_ambient(l: Pin<&mut NiLight>, v: Vec3);
        fn light_diffuse(l: &NiLight) -> Vec3;
        fn set_light_diffuse(l: Pin<&mut NiLight>, v: Vec3);
        fn light_specular(l: &NiLight) -> Vec3;
        fn set_light_specular(l: Pin<&mut NiLight>, v: Vec3);
        fn point_light_constant(l: &NiPointLight) -> f32;
        fn set_point_light_constant(l: Pin<&mut NiPointLight>, v: f32);
        fn point_light_linear(l: &NiPointLight) -> f32;
        fn set_point_light_linear(l: Pin<&mut NiPointLight>, v: f32);
        fn point_light_quadratic(l: &NiPointLight) -> f32;
        fn set_point_light_quadratic(l: Pin<&mut NiPointLight>, v: f32);
        fn spot_light_outer(l: &NiSpotLight) -> f32;
        fn set_spot_light_outer(l: Pin<&mut NiSpotLight>, v: f32);
        fn spot_light_inner(l: &NiSpotLight) -> f32;
        fn set_spot_light_inner(l: Pin<&mut NiSpotLight>, v: f32);
        fn spot_light_exponent(l: &NiSpotLight) -> f32;
        fn set_spot_light_exponent(l: Pin<&mut NiSpotLight>, v: f32);

        /// `NiNode::effectRefs` block indices (`NIF_NPOS` if empty).
        fn node_effects(node: &NiNode) -> Vec<u32>;

        /// Replace `NiNode::effectRefs` of the node at `node`, the affected nodes of the effect
        /// at `effect`, or the source texture of the texture effect at `effect` (`NIF_NPOS`
        /// clears it). Return false, changing nothing, if a block has the wrong type.
        fn set_node_effects(file: Pin<&mut NifFile>, node: u32, effects: &[u32]) -> bool;
        fn set_effect_affected_nodes(file: Pin<&mut NifFile>, effect: u32, nodes: &[u32]) -> bool;
        fn set_texture_effect_source(file: Pin<&mut NifFile>, effect: u32, texture: u32) -> bool;

        /// Append a new effect to the effects of the node at `node`. Return its index, or
        /// `NIF_NPOS` (changing nothing) if `node` isn't an `NiNode`.
        fn add_texture_effect(file: Pin<&mut NifFile>, node: u32) -> u32;
        fn add_ambient_light(file: Pin<&mut NifFile>, node: u32) -> u32;
        fn add_directional_light(file: Pin<&mut NifFile>, node: u32) -> u32;
        fn add_point_light(file: Pin<&mut NifFile>, node: u32) -> u32;
        fn add_spot_light(file: Pin<&mut NifFile>, node: u32) -> u32;
    }
}
