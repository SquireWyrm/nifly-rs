
# Type hierarchy

<details open>
<summary>2 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiBoneContainer`]
    - **`BsSkinInstance`**

</details>

# Handle methods

<details open>
<summary>From <code>BsSkinInstance</code></summary>

- `View`
  - [`bone_data`](crate::handle::View::bone_data)
- `Edit`
  - [`bone_data`](crate::handle::Edit::bone_data)
  - [`set_bone_data`](crate::handle::Edit::set_bone_data)

</details>

<details>
<summary>From <code>NiBoneContainer</code>, after <code>upcast</code></summary>

These are on handles of exactly [`NiBoneContainer`], so [`upcast`](crate::handle::View::upcast) to it first.

- `View`
  - [`root`](crate::handle::View::root)
  - [`bones`](crate::handle::View::bones)
- `Edit`
  - [`set_bones`](crate::handle::Edit::set_bones)
  - [`root`](crate::handle::Edit::root)
  - [`set_root`](crate::handle::Edit::set_root)

</details>
