
# Type hierarchy

<details>
<summary>2 supertypes, 42 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - [`BhkRefObject`]
    - **`BhkSerializable`**
      - [`BhkShape`]
        - [`BhkHeightFieldShape`]
          - [`BhkPlaneShape`]
        - [`BhkSphereRepShape`]
          - [`BhkConvexShape`]
            - [`BhkConvexVerticesShape`]
            - [`BhkBoxShape`]
            - [`BhkSphereShape`]
            - [`BhkCylinderShape`]
            - [`BhkCapsuleShape`]
          - [`BhkMultiSphereShape`]
        - [`BhkConvexListShape`]
        - [`BhkTransformShape`]
          - [`BhkConvexTransformShape`]
        - [`BhkBvTreeShape`]
          - [`BhkMoppBvTreeShape`]
        - [`BhkNiTriStripsShape`]
        - [`BhkShapeCollection`]
          - [`BhkListShape`]
          - [`HkPackedNiTriStripsData`]
          - [`BhkPackedNiTriStripsShape`]
        - [`BhkCompressedMeshShape`]
      - [`BhkLiquidAction`]
      - [`BhkOrientHingedBodyAction`]
      - [`BhkWorldObject`]
        - [`BhkPhantom`]
          - [`BhkShapePhantom`]
            - [`BhkSimpleShapePhantom`]
            - [`BhkAabbPhantom`]
        - [`BhkEntity`]
          - [`BhkRigidBody`]
            - [`BhkRigidBodyT`]
      - [`BhkConstraint`]
        - [`BhkHingeConstraint`]
        - [`BhkLimitedHingeConstraint`]
        - [`BhkBreakableConstraint`]
        - [`BhkRagdollConstraint`]
        - [`BhkStiffSpringConstraint`]
        - [`BhkPrismaticConstraint`]
        - [`BhkMalleableConstraint`]
        - [`BhkBallAndSocketConstraint`]
      - [`BhkBallSocketConstraintChain`]

</details>
