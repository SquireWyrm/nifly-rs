
# Type hierarchy

<details open>
<summary>4 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - [`NiTexture`]
      - [`NiSourceTexture`]
        - **`NiSourceCubeMap`**

</details>

# Handle methods

<details>
<summary>From <code>NiSourceTexture</code>, after <code>upcast</code></summary>

These are on handles of exactly [`NiSourceTexture`], so [`upcast`](crate::handle::View::upcast) to it first.

- `View`
  - [`pixel_data`](crate::handle::View::pixel_data)
- `Edit`
  - [`pixel_data`](crate::handle::Edit::pixel_data)
  - [`assign_pixel_data`](crate::handle::Edit::assign_pixel_data)
  - [`set_pixel_data`](crate::handle::Edit::set_pixel_data)

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
