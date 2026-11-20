
# Type hierarchy

<details>
<summary>1 supertype, 68 subtypes</summary>

- [`objects::NiObject`](crate::objects::NiObject)
  - **`NiTimeController`**
    - [`NiLookAtController`]
    - [`NiPathController`]
    - [`NiPSysResetOnLoopCtlr`]
    - [`NiUvController`]
    - [`BsFrustumFovController`]
    - [`BsLagBoneController`]
    - [`BsProceduralLightningController`]
    - [`NiBoneLodController`]
      - [`NiBsBoneLodController`]
    - [`NiInterpController`]
      - [`NiGeomMorpherController`]
      - [`NiSingleInterpController`]
        - [`NiRollController`]
        - [`NiPoint3InterpController`]
          - [`NiMaterialColorController`]
          - [`NiLightColorController`]
        - [`NiExtraDataController`]
          - [`NiFloatExtraDataController`]
        - [`NiBoolInterpController`]
          - [`NiVisController`]
        - [`NiFloatInterpController`]
          - [`NiFlipController`]
          - [`NiTextureTransformController`]
          - [`NiLightDimmerController`]
          - [`NiLightRadiusController`]
          - [`NiAlphaController`]
            - [`BsNiAlphaPropertyTestRefController`]
          - [`BsMaterialEmittanceMultController`]
          - [`BsRefractionStrengthController`]
          - [`BsLightingShaderPropertyColorController`]
          - [`BsLightingShaderPropertyFloatController`]
          - [`BsLightingShaderPropertyUShortController`]
          - [`BsEffectShaderPropertyColorController`]
          - [`BsEffectShaderPropertyFloatController`]
        - [`BsRefractionFirePeriodController`]
        - [`NiKeyframeController`]
          - [`NiTransformController`]
        - [`NiPSysModifierCtlr`]
          - [`NiPSysModifierBoolCtlr`]
            - [`NiPSysModifierActiveCtlr`]
          - [`NiPSysModifierFloatCtlr`]
            - [`NiPSysEmitterLifeSpanCtlr`]
            - [`NiPSysEmitterSpeedCtlr`]
            - [`NiPSysEmitterInitialRadiusCtlr`]
            - [`NiPSysEmitterDeclinationCtlr`]
            - [`NiPSysGravityStrengthCtlr`]
            - [`NiPSysEmitterDeclinationVarCtlr`]
            - [`NiPSysFieldMagnitudeCtlr`]
            - [`NiPSysFieldAttenuationCtlr`]
            - [`NiPSysFieldMaxDistanceCtlr`]
            - [`NiPSysAirFieldAirFrictionCtlr`]
            - [`NiPSysAirFieldInheritVelocityCtlr`]
            - [`NiPSysAirFieldSpreadCtlr`]
            - [`NiPSysInitialRotSpeedCtlr`]
            - [`NiPSysInitialRotSpeedVarCtlr`]
            - [`NiPSysInitialRotAngleCtlr`]
            - [`NiPSysInitialRotAngleVarCtlr`]
            - [`NiPSysEmitterPlanarAngleCtlr`]
            - [`NiPSysEmitterPlanarAngleVarCtlr`]
            - [`NiPSysRotDampeningCtlr`]
          - [`particles::NiPSysEmitterCtlr`](crate::particles::NiPSysEmitterCtlr)
            - [`particles::BsPSysMultiTargetEmitterCtlr`](crate::particles::BsPSysMultiTargetEmitterCtlr)
      - [`NiMultiTargetTransformController`]
    - [`NiPSysUpdateCtlr`]
    - [`NiControllerManager`]
    - [`BhkBlendController`]
    - [`particles::NiParticleSystemController`](crate::particles::NiParticleSystemController)
      - [`particles::NiBspArrayController`](crate::particles::NiBspArrayController)

</details>

# Handle methods

<details open>
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
