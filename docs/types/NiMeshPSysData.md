
# Type hierarchy

<details open>
<summary>5 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`geometry::NiGeometryData`](crate::geometry::NiGeometryData)
    - [`NiParticlesData`]
      - [`NiRotatingParticlesData`]
        - [`NiPSysData`]
          - **`NiMeshPSysData`**

</details>

# Handle methods

<details open>
<summary>From <code>NiMeshPSysData</code></summary>

- `View`
  - [`mesh_node`](crate::handle::View::mesh_node)
- `Edit`
  - [`mesh_node`](crate::handle::Edit::mesh_node)
  - [`set_mesh_node`](crate::handle::Edit::set_mesh_node)

</details>

<details>
<summary>From <code>NiGeometryData</code></summary>

- `View`
  - [`additional_data`](crate::handle::View::additional_data)
- `Edit`
  - [`additional_data`](crate::handle::Edit::additional_data)
  - [`assign_additional_data`](crate::handle::Edit::assign_additional_data)
  - [`remove_additional_data`](crate::handle::Edit::remove_additional_data)
  - [`set_uv_sets`](crate::handle::Edit::set_uv_sets)
  - [`set_data_raw_flags`](crate::handle::Edit::set_data_raw_flags)
  - [`set_additional_data`](crate::handle::Edit::set_additional_data)

</details>
