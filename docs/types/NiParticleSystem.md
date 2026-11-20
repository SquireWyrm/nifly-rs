
# Type hierarchy

<details open>
<summary>3 supertypes, 2 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - [`objects::NiAvObject`](crate::objects::NiAvObject)
      - **`NiParticleSystem`**
        - [`NiMeshParticleSystem`]
        - [`BsStripParticleSystem`]

</details>

# Handle methods

<details open>
<summary>From <code>NiParticleSystem</code></summary>

- `View`
  - [`psys_data`](crate::handle::View::psys_data)
  - [`psys_skin_instance`](crate::handle::View::psys_skin_instance)
  - [`psys_shader`](crate::handle::View::psys_shader)
  - [`psys_alpha_property`](crate::handle::View::psys_alpha_property)
  - [`modifiers`](crate::handle::View::modifiers)
- `Edit`
  - [`psys_data`](crate::handle::Edit::psys_data)
  - [`set_psys_data`](crate::handle::Edit::set_psys_data)
  - [`psys_skin_instance`](crate::handle::Edit::psys_skin_instance)
  - [`set_psys_skin_instance`](crate::handle::Edit::set_psys_skin_instance)
  - [`psys_shader`](crate::handle::Edit::psys_shader)
  - [`set_psys_shader`](crate::handle::Edit::set_psys_shader)
  - [`psys_alpha_property`](crate::handle::Edit::psys_alpha_property)
  - [`set_psys_alpha_property`](crate::handle::Edit::set_psys_alpha_property)
  - [`set_modifiers`](crate::handle::Edit::set_modifiers)
  - [`add_modifier`](crate::handle::Edit::add_modifier)

</details>

<details>
<summary>From <code>NiAvObject</code></summary>

- `View`
  - [`collision`](crate::handle::View::collision)
  - [`parent`](crate::handle::View::parent)
  - [`properties`](crate::handle::View::properties)
- `Edit`
  - [`assign_collision`](crate::handle::Edit::assign_collision)
  - [`parent`](crate::handle::Edit::parent)
  - [`collision`](crate::handle::Edit::collision)
  - [`set_collision`](crate::handle::Edit::set_collision)
  - [`set_parent`](crate::handle::Edit::set_parent)
  - [`set_properties`](crate::handle::Edit::set_properties)
  - [`add_property`](crate::handle::Edit::add_property)

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
