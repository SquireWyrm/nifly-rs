
# Type hierarchy

<details open>
<summary>8 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - [`NiProperty`]
      - [`NiShader`]
        - [`NiShadeProperty`]
          - [`BsShaderProperty`]
            - [`BsShaderLightingProperty`]
              - [`BsShaderPpLightingProperty`]
                - **`Lighting30ShaderProperty`**

</details>

# Handle methods

<details>
<summary>From <code>NiShader</code>, after <code>upcast</code></summary>

These are on handles of exactly [`NiShader`], so [`upcast`](crate::handle::View::upcast) to it first.

- `View`
  - [`texture_set`](crate::handle::View::texture_set)
- `Edit`
  - [`texture_set`](crate::handle::Edit::texture_set)
  - [`set_texture_set`](crate::handle::Edit::set_texture_set)

</details>

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
