
# Type hierarchy

<details open>
<summary>1 supertype, 2 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - **`NiPSysCollider`**
    - [`NiPSysSphericalCollider`]
    - [`NiPSysPlanarCollider`]

</details>

# Handle methods

<details open>
<summary>From <code>NiPSysCollider</code></summary>

- `View`
  - [`next_collider`](crate::handle::View::next_collider)
  - [`collider_manager`](crate::handle::View::collider_manager)
- `Edit`
  - [`next_collider`](crate::handle::Edit::next_collider)
  - [`set_next_collider`](crate::handle::Edit::set_next_collider)
  - [`collider_manager`](crate::handle::Edit::collider_manager)
  - [`set_collider_manager`](crate::handle::Edit::set_collider_manager)

</details>

<details>
<summary>From <code>HasSpawnModifier</code></summary>

- `View`
  - [`spawn_modifier`](crate::handle::View::spawn_modifier)
- `Edit`
  - [`spawn_modifier`](crate::handle::Edit::spawn_modifier)
  - [`set_spawn_modifier`](crate::handle::Edit::set_spawn_modifier)

</details>

<details>
<summary>From <code>HasModifierObject</code></summary>

- `View`
  - [`modifier_object`](crate::handle::View::modifier_object)
- `Edit`
  - [`modifier_object`](crate::handle::Edit::modifier_object)
  - [`set_modifier_object`](crate::handle::Edit::set_modifier_object)

</details>
