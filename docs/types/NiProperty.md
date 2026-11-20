
# Type hierarchy

<details>
<summary>2 supertypes, 29 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - **`NiProperty`**
      - [`NiShader`]
        - [`NiShadeProperty`]
          - [`BsShaderProperty`]
            - [`WaterShaderProperty`]
            - [`HairShaderProperty`]
            - [`DistantLodShaderProperty`]
            - [`BsDistantTreeShaderProperty`]
            - [`TallGrassShaderProperty`]
            - [`VolumetricFogShaderProperty`]
            - [`BsLightingShaderProperty`]
            - [`BsEffectShaderProperty`]
            - [`BsWaterShaderProperty`]
            - [`BsSkyShaderProperty`]
            - [`BsShaderLightingProperty`]
              - [`SkyShaderProperty`]
              - [`TileShaderProperty`]
              - [`BsShaderNoLightingProperty`]
              - [`BsShaderPpLightingProperty`]
                - [`Lighting30ShaderProperty`]
        - [`NiMaterialProperty`]
      - [`NiAlphaProperty`]
      - [`NiStencilProperty`]
      - [`NiTexturingProperty`]
      - [`NiSpecularProperty`]
      - [`NiVertexColorProperty`]
      - [`NiDitherProperty`]
      - [`NiFogProperty`]
      - [`NiWireframeProperty`]
      - [`NiZBufferProperty`]

</details>

# Handle methods

<details>
<summary>From <code>NiObjectNet</code></summary>

- `View`
  - [`controller`](crate::handle::View::controller)
  - [`controllers`](crate::handle::View::controllers)
  - [`extra_data`](crate::handle::View::extra_data)
  - [`extra_data_chain`](crate::handle::View::extra_data_chain)
- `Edit`
  - [`controller`](crate::handle::Edit::controller)
  - [`set_controller`](crate::handle::Edit::set_controller)
  - [`set_controllers`](crate::handle::Edit::set_controllers)
  - [`add_controller`](crate::handle::Edit::add_controller)
  - [`assign_extra_data`](crate::handle::Edit::assign_extra_data)
  - [`set_extra_data`](crate::handle::Edit::set_extra_data)
  - [`set_extra_data_chain`](crate::handle::Edit::set_extra_data_chain)

</details>
