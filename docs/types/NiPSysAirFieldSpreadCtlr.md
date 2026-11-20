
# Type hierarchy

<details open>
<summary>6 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiTimeController`]
    - [`NiInterpController`]
      - [`NiSingleInterpController`]
        - [`NiPSysModifierCtlr`]
          - [`NiPSysModifierFloatCtlr`]
            - **`NiPSysAirFieldSpreadCtlr`**

</details>

# Handle methods

<details>
<summary>From <code>HasInterpolator</code></summary>

- `View`
  - [`interpolator`](crate::handle::View::interpolator)
- `Edit`
  - [`interpolator`](crate::handle::Edit::interpolator)
  - [`set_interpolator`](crate::handle::Edit::set_interpolator)

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
