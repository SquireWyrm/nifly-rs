
# Type hierarchy

<details open>
<summary>2 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiTimeController`]
    - **`NiPathController`**

</details>

# Handle methods

<details>
<summary>From <code>HasPath</code></summary>

- `View`
  - [`path_data`](crate::handle::View::path_data)
  - [`percent_data`](crate::handle::View::percent_data)
- `Edit`
  - [`path_data`](crate::handle::Edit::path_data)
  - [`set_path_data`](crate::handle::Edit::set_path_data)
  - [`percent_data`](crate::handle::Edit::percent_data)
  - [`set_percent_data`](crate::handle::Edit::set_percent_data)

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
