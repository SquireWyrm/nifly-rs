
# Type hierarchy

<details>
<summary>2 supertypes, 53 subtypes</summary>

- [`NiObject`]
  - [`NiObjectNet`]
    - **`NiAvObject`**
      - [`effects::NiDynamicEffect`](crate::effects::NiDynamicEffect)
        - [`effects::NiTextureEffect`](crate::effects::NiTextureEffect)
        - [`effects::NiLight`](crate::effects::NiLight)
          - [`effects::NiAmbientLight`](crate::effects::NiAmbientLight)
          - [`effects::NiDirectionalLight`](crate::effects::NiDirectionalLight)
          - [`effects::NiPointLight`](crate::effects::NiPointLight)
            - [`effects::NiSpotLight`](crate::effects::NiSpotLight)
      - [`geometry::NiShape`](crate::geometry::NiShape)
        - [`geometry::BsTriShape`](crate::geometry::BsTriShape)
          - [`geometry::BsSubIndexTriShape`](crate::geometry::BsSubIndexTriShape)
          - [`geometry::BsMeshLodTriShape`](crate::geometry::BsMeshLodTriShape)
          - [`geometry::BsDynamicTriShape`](crate::geometry::BsDynamicTriShape)
        - [`geometry::BsGeometry`](crate::geometry::BsGeometry)
        - [`geometry::NiGeometry`](crate::geometry::NiGeometry)
          - [`geometry::NiTriBasedGeom`](crate::geometry::NiTriBasedGeom)
            - [`geometry::NiTriShape`](crate::geometry::NiTriShape)
              - [`geometry::NiScreenElements`](crate::geometry::NiScreenElements)
              - [`geometry::BsSegmentedTriShape`](crate::geometry::BsSegmentedTriShape)
            - [`geometry::NiTriStrips`](crate::geometry::NiTriStrips)
            - [`geometry::NiLines`](crate::geometry::NiLines)
            - [`geometry::BsLodTriShape`](crate::geometry::BsLodTriShape)
          - [`geometry::NiParticles`](crate::geometry::NiParticles)
            - [`geometry::NiAutoNormalParticles`](crate::geometry::NiAutoNormalParticles)
            - [`geometry::NiParticleMeshes`](crate::geometry::NiParticleMeshes)
            - [`geometry::NiRotatingParticles`](crate::geometry::NiRotatingParticles)
      - [`nodes::NiNode`](crate::nodes::NiNode)
        - [`nodes::BsFadeNode`](crate::nodes::BsFadeNode)
        - [`nodes::RootCollisionNode`](crate::nodes::RootCollisionNode)
        - [`nodes::AvoidNode`](crate::nodes::AvoidNode)
        - [`nodes::NiBsAnimationNode`](crate::nodes::NiBsAnimationNode)
        - [`nodes::NiBsParticleNode`](crate::nodes::NiBsParticleNode)
        - [`nodes::BsValueNode`](crate::nodes::BsValueNode)
        - [`nodes::BsLeafAnimNode`](crate::nodes::BsLeafAnimNode)
        - [`nodes::BsTreeNode`](crate::nodes::BsTreeNode)
        - [`nodes::BsOrderedNode`](crate::nodes::BsOrderedNode)
        - [`nodes::BsMultiBoundNode`](crate::nodes::BsMultiBoundNode)
          - [`nodes::BsDistantObjectInstancedNode`](crate::nodes::BsDistantObjectInstancedNode)
        - [`nodes::BsRangeNode`](crate::nodes::BsRangeNode)
          - [`nodes::BsDebrisNode`](crate::nodes::BsDebrisNode)
          - [`nodes::BsBlastNode`](crate::nodes::BsBlastNode)
            - [`nodes::BsDamageStage`](crate::nodes::BsDamageStage)
        - [`nodes::BsWeakReferenceNode`](crate::nodes::BsWeakReferenceNode)
        - [`nodes::BsFaceGenNiNode`](crate::nodes::BsFaceGenNiNode)
        - [`nodes::NiBillboardNode`](crate::nodes::NiBillboardNode)
        - [`nodes::NiSwitchNode`](crate::nodes::NiSwitchNode)
          - [`nodes::NiLodNode`](crate::nodes::NiLodNode)
        - [`nodes::NiBone`](crate::nodes::NiBone)
        - [`nodes::NiSortAdjustNode`](crate::nodes::NiSortAdjustNode)
        - [`nodes::BsMasterParticleSystem`](crate::nodes::BsMasterParticleSystem)
      - [`nodes::NiCamera`](crate::nodes::NiCamera)
      - [`particles::NiParticleSystem`](crate::particles::NiParticleSystem)
        - [`particles::NiMeshParticleSystem`](crate::particles::NiMeshParticleSystem)
        - [`particles::BsStripParticleSystem`](crate::particles::BsStripParticleSystem)

</details>

# Handle methods

<details open>
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
