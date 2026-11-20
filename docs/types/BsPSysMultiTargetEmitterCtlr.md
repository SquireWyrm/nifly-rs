
# Type hierarchy

<details open>
<summary>6 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`animation::NiTimeController`](crate::animation::NiTimeController)
    - [`animation::NiInterpController`](crate::animation::NiInterpController)
      - [`animation::NiSingleInterpController`](crate::animation::NiSingleInterpController)
        - [`animation::NiPSysModifierCtlr`](crate::animation::NiPSysModifierCtlr)
          - [`NiPSysEmitterCtlr`]
            - **`BsPSysMultiTargetEmitterCtlr`**

</details>

# Handle methods

<details open>
<summary>From <code>BsPSysMultiTargetEmitterCtlr</code></summary>

- `View`
  - [`master_particle_system`](crate::handle::View::master_particle_system)
- `Edit`
  - [`master_particle_system`](crate::handle::Edit::master_particle_system)
  - [`set_master_particle_system`](crate::handle::Edit::set_master_particle_system)

</details>

<details>
<summary>From <code>HasInterpolator</code></summary>

- `View`
  - [`interpolator`](crate::handle::View::interpolator)
- `Edit`
  - [`interpolator`](crate::handle::Edit::interpolator)
  - [`set_interpolator`](crate::handle::Edit::set_interpolator)

</details>

<details>
<summary>From <code>NiPSysEmitterCtlr</code></summary>

- `View`
  - [`emitter_data`](crate::handle::View::emitter_data)
  - [`visibility_interpolator`](crate::handle::View::visibility_interpolator)
- `Edit`
  - [`emitter_data`](crate::handle::Edit::emitter_data)
  - [`set_emitter_data`](crate::handle::Edit::set_emitter_data)
  - [`visibility_interpolator`](crate::handle::Edit::visibility_interpolator)
  - [`set_visibility_interpolator`](crate::handle::Edit::set_visibility_interpolator)

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
