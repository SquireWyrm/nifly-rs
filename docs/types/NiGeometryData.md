
# Type hierarchy

<details open>
<summary>1 supertype, 13 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - **`NiGeometryData`**
    - [`NiTriBasedGeomData`]
      - [`NiTriShapeData`]
        - [`NiScreenElementsData`]
      - [`NiTriStripsData`]
    - [`NiLinesData`]
    - [`BsGeometryMeshData`]
    - [`particles::NiParticlesData`](crate::particles::NiParticlesData)
      - [`particles::NiAutoNormalParticlesData`](crate::particles::NiAutoNormalParticlesData)
      - [`particles::NiRotatingParticlesData`](crate::particles::NiRotatingParticlesData)
        - [`particles::NiParticleMeshesData`](crate::particles::NiParticleMeshesData)
        - [`particles::NiPSysData`](crate::particles::NiPSysData)
          - [`particles::NiMeshPSysData`](crate::particles::NiMeshPSysData)
          - [`particles::BsStripPSysData`](crate::particles::BsStripPSysData)

</details>

# Handle methods

<details open>
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
