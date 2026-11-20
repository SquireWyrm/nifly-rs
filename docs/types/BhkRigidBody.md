
# Type hierarchy

<details open>
<summary>5 supertypes, 1 subtype</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`BhkRefObject`]
    - [`BhkSerializable`]
      - [`BhkWorldObject`]
        - [`BhkEntity`]
          - **`BhkRigidBody`**
            - [`BhkRigidBodyT`]

</details>

# Handle methods

<details open>
<summary>From <code>BhkRigidBody</code></summary>

- `View`
  - [`constraints`](crate::handle::View::constraints)
- `Edit`
  - [`set_constraints`](crate::handle::Edit::set_constraints)
  - [`add_constraint`](crate::handle::Edit::add_constraint)

</details>

<details>
<summary>From <code>HasShape</code></summary>

- `View`
  - [`shape`](crate::handle::View::shape)
- `Edit`
  - [`shape`](crate::handle::Edit::shape)
  - [`set_shape`](crate::handle::Edit::set_shape)
  - [`assign_shape`](crate::handle::Edit::assign_shape)

</details>
