
# Type hierarchy

<details open>
<summary>2 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiTimeController`]
    - **`NiControllerManager`**

</details>

# Handle methods

<details open>
<summary>From <code>NiControllerManager</code></summary>

- `View`
  - [`sequences`](crate::handle::View::sequences)
  - [`object_palette`](crate::handle::View::object_palette)
- `Edit`
  - [`set_sequences`](crate::handle::Edit::set_sequences)
  - [`add_sequence`](crate::handle::Edit::add_sequence)
  - [`object_palette`](crate::handle::Edit::object_palette)
  - [`set_object_palette`](crate::handle::Edit::set_object_palette)

</details>

<details>
<summary>From <code>NiTimeController</code></summary>

- `View`
  - [`next_controller`](crate::handle::View::next_controller)
  - [`controller_target`](crate::handle::View::controller_target)
- `Edit`
  - [`next_controller`](crate::handle::Edit::next_controller)
  - [`set_next_controller`](crate::handle::Edit::set_next_controller)
  - [`controller_target`](crate::handle::Edit::controller_target)
  - [`set_controller_target`](crate::handle::Edit::set_controller_target)

</details>
