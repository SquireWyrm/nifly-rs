
# Type hierarchy

<details open>
<summary>6 supertypes, no subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`objects::NiObjectNet`](crate::objects::NiObjectNet)
    - [`objects::NiAvObject`](crate::objects::NiAvObject)
      - [`NiShape`]
        - [`NiGeometry`]
          - [`NiTriBasedGeom`]
            - **`BsLodTriShape`**

</details>

# Handle methods

<details>
<summary>From <code>NiShape</code>, after <code>upcast</code></summary>

These are on handles of exactly [`NiShape`], so [`upcast`](crate::handle::View::upcast) to it first.

- `View`
  - [`mesh_paths`](crate::handle::View::mesh_paths)
  - [`geometry_data`](crate::handle::View::geometry_data)
  - [`is_sse_compatible`](crate::handle::View::is_sse_compatible)
  - [`vertices`](crate::handle::View::vertices)
  - [`normals`](crate::handle::View::normals)
  - [`uvs`](crate::handle::View::uvs)
  - [`vertex_colors`](crate::handle::View::vertex_colors)
  - [`tangents`](crate::handle::View::tangents)
  - [`bitangents`](crate::handle::View::bitangents)
  - [`eye_data`](crate::handle::View::eye_data)
  - [`vertex_diff`](crate::handle::View::vertex_diff)
  - [`uv_diff`](crate::handle::View::uv_diff)
  - [`triangles`](crate::handle::View::triangles)
  - [`save_mesh`](crate::handle::View::save_mesh)
  - [`segments`](crate::handle::View::segments)
  - [`shader`](crate::handle::View::shader)
  - [`alpha_property`](crate::handle::View::alpha_property)
  - [`stencil_property`](crate::handle::View::stencil_property)
  - [`texturing_property`](crate::handle::View::texturing_property)
  - [`material_property`](crate::handle::View::material_property)
  - [`texture_paths`](crate::handle::View::texture_paths)
  - [`texture_slot`](crate::handle::View::texture_slot)
  - [`skin_instance`](crate::handle::View::skin_instance)
  - [`bone_ids`](crate::handle::View::bone_ids)
  - [`bone_list`](crate::handle::View::bone_list)
  - [`max_weights_per_vertex`](crate::handle::View::max_weights_per_vertex)
  - [`weights_per_vertex`](crate::handle::View::weights_per_vertex)
  - [`bone_weights`](crate::handle::View::bone_weights)
  - [`partitions`](crate::handle::View::partitions)
  - [`transform_global_to_skin`](crate::handle::View::transform_global_to_skin)
  - [`calc_transform_global_to_skin`](crate::handle::View::calc_transform_global_to_skin)
  - [`transform_skin_to_bone`](crate::handle::View::transform_skin_to_bone)
  - [`bone_bounds`](crate::handle::View::bone_bounds)
- `Edit`
  - [`geometry_data`](crate::handle::Edit::geometry_data)
  - [`set_geometry_data`](crate::handle::Edit::set_geometry_data)
  - [`set_triangles`](crate::handle::Edit::set_triangles)
  - [`load_mesh`](crate::handle::Edit::load_mesh)
  - [`set_mesh_path`](crate::handle::Edit::set_mesh_path)
  - [`triangulate`](crate::handle::Edit::triangulate)
  - [`delete`](crate::handle::Edit::delete)
  - [`clone_shape`](crate::handle::Edit::clone_shape)
  - [`reorder_triangles`](crate::handle::Edit::reorder_triangles)
  - [`calc_normals`](crate::handle::Edit::calc_normals)
  - [`calc_tangents`](crate::handle::Edit::calc_tangents)
  - [`apply_locked_normals`](crate::handle::Edit::apply_locked_normals)
  - [`set_segments`](crate::handle::Edit::set_segments)
  - [`set_vertices`](crate::handle::Edit::set_vertices)
  - [`delete_vertices`](crate::handle::Edit::delete_vertices)
  - [`set_vertex`](crate::handle::Edit::set_vertex)
  - [`set_normals`](crate::handle::Edit::set_normals)
  - [`offset`](crate::handle::Edit::offset)
  - [`scale`](crate::handle::Edit::scale)
  - [`rotate`](crate::handle::Edit::rotate)
  - [`mirror`](crate::handle::Edit::mirror)
  - [`invert_uvs`](crate::handle::Edit::invert_uvs)
  - [`set_uvs`](crate::handle::Edit::set_uvs)
  - [`set_vertex_colors`](crate::handle::Edit::set_vertex_colors)
  - [`set_tangents`](crate::handle::Edit::set_tangents)
  - [`set_bitangents`](crate::handle::Edit::set_bitangents)
  - [`set_eye_data`](crate::handle::Edit::set_eye_data)
  - [`shader`](crate::handle::Edit::shader)
  - [`alpha_property`](crate::handle::Edit::alpha_property)
  - [`stencil_property`](crate::handle::Edit::stencil_property)
  - [`texturing_property`](crate::handle::Edit::texturing_property)
  - [`material_property`](crate::handle::Edit::material_property)
  - [`assign_alpha_property`](crate::handle::Edit::assign_alpha_property)
  - [`remove_alpha_property`](crate::handle::Edit::remove_alpha_property)
  - [`delete_shader`](crate::handle::Edit::delete_shader)
  - [`set_shader`](crate::handle::Edit::set_shader)
  - [`set_alpha_property`](crate::handle::Edit::set_alpha_property)
  - [`set_texture_slot`](crate::handle::Edit::set_texture_slot)
  - [`skin_instance`](crate::handle::Edit::skin_instance)
  - [`set_skin_instance`](crate::handle::Edit::set_skin_instance)
  - [`set_bone_weights`](crate::handle::Edit::set_bone_weights)
  - [`set_bone_ids`](crate::handle::Edit::set_bone_ids)
  - [`set_bone_list`](crate::handle::Edit::set_bone_list)
  - [`replace_bone`](crate::handle::Edit::replace_bone)
  - [`update_skin_partitions`](crate::handle::Edit::update_skin_partitions)
  - [`update_partition_flags`](crate::handle::Edit::update_partition_flags)
  - [`set_vertex_weights`](crate::handle::Edit::set_vertex_weights)
  - [`clear_vertex_weights`](crate::handle::Edit::clear_vertex_weights)
  - [`create_skinning`](crate::handle::Edit::create_skinning)
  - [`delete_skinning`](crate::handle::Edit::delete_skinning)
  - [`set_partitions`](crate::handle::Edit::set_partitions)
  - [`set_default_partition`](crate::handle::Edit::set_default_partition)
  - [`delete_partitions`](crate::handle::Edit::delete_partitions)
  - [`remove_empty_partitions`](crate::handle::Edit::remove_empty_partitions)
  - [`set_transform_skin_to_bone`](crate::handle::Edit::set_transform_skin_to_bone)
  - [`set_bone_bounds`](crate::handle::Edit::set_bone_bounds)

</details>

<details>
<summary>From <code>NiAvObject</code></summary>

- `View`
  - [`collision`](crate::handle::View::collision)
  - [`parent`](crate::handle::View::parent)
  - [`properties`](crate::handle::View::properties)
- `Edit`
  - [`assign_collision`](crate::handle::Edit::assign_collision)
  - [`parent`](crate::handle::Edit::parent)
  - [`collision`](crate::handle::Edit::collision)
  - [`set_collision`](crate::handle::Edit::set_collision)
  - [`set_parent`](crate::handle::Edit::set_parent)
  - [`set_properties`](crate::handle::Edit::set_properties)
  - [`add_property`](crate::handle::Edit::add_property)

</details>

<details>
<summary>From <code>NiObjectNet</code></summary>

- `View`
  - [`controller`](crate::handle::View::controller)
  - [`controllers`](crate::handle::View::controllers)
  - [`extra_data`](crate::handle::View::extra_data)
  - [`extra_data_chain`](crate::handle::View::extra_data_chain)
- `Edit`
  - [`controller`](crate::handle::Edit::controller)
  - [`set_controller`](crate::handle::Edit::set_controller)
  - [`set_controllers`](crate::handle::Edit::set_controllers)
  - [`add_controller`](crate::handle::Edit::add_controller)
  - [`assign_extra_data`](crate::handle::Edit::assign_extra_data)
  - [`set_extra_data`](crate::handle::Edit::set_extra_data)
  - [`set_extra_data_chain`](crate::handle::Edit::set_extra_data_chain)

</details>
