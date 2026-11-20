
# Type hierarchy

<details open>
<summary>3 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiBoneContainer`]
    - [`NiSkinInstance`]
      - **`BsDismemberSkinInstance`**

</details>

# Handle methods

<details>
<summary>From <code>NiSkinInstance</code>, after <code>upcast</code></summary>

These are on handles of exactly [`NiSkinInstance`], so [`upcast`](crate::handle::View::upcast) to it first.

- `View`
  - [`skin_data`](crate::handle::View::skin_data)
  - [`skin_partition`](crate::handle::View::skin_partition)
- `Edit`
  - [`skin_data`](crate::handle::Edit::skin_data)
  - [`skin_partition`](crate::handle::Edit::skin_partition)
  - [`set_skin_data`](crate::handle::Edit::set_skin_data)
  - [`set_skin_partition`](crate::handle::Edit::set_skin_partition)

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
