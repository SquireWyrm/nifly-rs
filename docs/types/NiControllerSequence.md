
# Type hierarchy

<details open>
<summary>2 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`NiSequence`]
    - **`NiControllerSequence`**

</details>

# Handle methods

<details open>
<summary>From <code>NiControllerSequence</code></summary>

- `View`
  - [`sequence_manager`](crate::handle::View::sequence_manager)
  - [`string_palette`](crate::handle::View::string_palette)
  - [`anim_notes`](crate::handle::View::anim_notes)
  - [`anim_note_arrays`](crate::handle::View::anim_note_arrays)
- `Edit`
  - [`sequence_manager`](crate::handle::Edit::sequence_manager)
  - [`set_sequence_manager`](crate::handle::Edit::set_sequence_manager)
  - [`string_palette`](crate::handle::Edit::string_palette)
  - [`set_string_palette`](crate::handle::Edit::set_string_palette)
  - [`anim_notes`](crate::handle::Edit::anim_notes)
  - [`set_anim_notes`](crate::handle::Edit::set_anim_notes)
  - [`set_anim_note_arrays`](crate::handle::Edit::set_anim_note_arrays)

</details>

<details>
<summary>From <code>NiSequence</code></summary>

- `View`
  - [`text_key_data`](crate::handle::View::text_key_data)
  - [`controlled_blocks`](crate::handle::View::controlled_blocks)
- `Edit`
  - [`text_key_data`](crate::handle::Edit::text_key_data)
  - [`set_text_key_data`](crate::handle::Edit::set_text_key_data)
  - [`set_controlled_blocks`](crate::handle::Edit::set_controlled_blocks)

</details>
