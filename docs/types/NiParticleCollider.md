
# Type hierarchy

<details open>
<summary>2 supertypes, 2 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiParticleModifier`]
    - **`NiParticleCollider`**
      - [`NiPlanarCollider`]
      - [`NiSphericalCollider`]

</details>

# Handle methods

<details>
<summary>From <code>ChainedModifier</code></summary>

- `View`
  - [`next_modifier`](crate::handle::View::next_modifier)
  - [`particle_controller`](crate::handle::View::particle_controller)
- `Edit`
  - [`next_modifier`](crate::handle::Edit::next_modifier)
  - [`set_next_modifier`](crate::handle::Edit::set_next_modifier)
  - [`particle_controller`](crate::handle::Edit::particle_controller)
  - [`set_particle_controller`](crate::handle::Edit::set_particle_controller)

</details>
