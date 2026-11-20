
# Type hierarchy

<details open>
<summary>4 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiPSysModifier`]
    - [`NiPSysEmitter`]
      - [`NiPSysVolumeEmitter`]
        - **`NiPSysSphereEmitter`**

</details>

# Handle methods

<details>
<summary>From <code>HasModifierObject</code></summary>

- `View`
  - [`modifier_object`](crate::handle::View::modifier_object)
- `Edit`
  - [`modifier_object`](crate::handle::Edit::modifier_object)
  - [`set_modifier_object`](crate::handle::Edit::set_modifier_object)

</details>

<details>
<summary>From <code>NiPSysModifier</code></summary>

- `View`
  - [`particle_system`](crate::handle::View::particle_system)
- `Edit`
  - [`particle_system`](crate::handle::Edit::particle_system)
  - [`set_particle_system`](crate::handle::Edit::set_particle_system)

</details>
