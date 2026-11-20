
# Type hierarchy

<details open>
<summary>4 supertypes, 3 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`BhkRefObject`]
    - [`BhkSerializable`]
      - [`BhkWorldObject`]
        - **`BhkPhantom`**
          - [`BhkShapePhantom`]
            - [`BhkSimpleShapePhantom`]
            - [`BhkAabbPhantom`]

</details>

# Handle methods

<details>
<summary>From <code>HasShape</code></summary>

- `View`
  - [`shape`](crate::handle::View::shape)
- `Edit`
  - [`shape`](crate::handle::Edit::shape)
  - [`set_shape`](crate::handle::Edit::set_shape)
  - [`assign_shape`](crate::handle::Edit::assign_shape)

</details>
