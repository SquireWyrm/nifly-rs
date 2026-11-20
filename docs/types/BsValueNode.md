
# Type hierarchy

<details open>
<summary>4 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - [`objects::NiAvObject`](crate::objects::NiAvObject)
      - [`NiNode`]
        - **`BsValueNode`**

</details>

# Handle methods

<details>
<summary>From <code>NiNode</code></summary>

- `View`
  - [`effects`](crate::handle::View::effects)
  - [`children`](crate::handle::View::children)
  - [`children_of`](crate::handle::View::children_of)
- `Edit`
  - [`set_effects`](crate::handle::Edit::set_effects)
  - [`add_effect`](crate::handle::Edit::add_effect)
  - [`set_children`](crate::handle::Edit::set_children)
  - [`clone_node`](crate::handle::Edit::clone_node)
  - [`add_child`](crate::handle::Edit::add_child)
  - [`add_child_of`](crate::handle::Edit::add_child_of)
  - [`add_particle_system`](crate::handle::Edit::add_particle_system)

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
