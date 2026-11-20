//! Material, alpha and stencil properties.

use super::*;

impl NiMaterialProperty {
    fields! {
        /// Flags of unknown meaning. Only saved in files from version 3.0 up to 10.0.1.2, which
        /// include Morrowind's.
        legacy_raw_flags, set_legacy_raw_flags: u16 = material_legacy_flags, set_material_legacy_flags, same, same;
    }

    /// Ambient color. Only saved in Oblivion and older files.
    pub fn ambient(&self) -> [f32; 3] {
        rgb(ffi::material_ambient(&self.0))
    }

    /// Diffuse color. Only saved in Oblivion and older files.
    pub fn diffuse(&self) -> [f32; 3] {
        rgb(ffi::material_diffuse(&self.0))
    }

    pub fn specular(&self) -> [f32; 3] {
        rgb(ffi::material_specular(&self.0))
    }

    pub fn emissive(&self) -> [f32; 3] {
        rgb(ffi::material_emissive(&self.0))
    }

    pub fn glossiness(&self) -> f32 {
        ffi::material_glossiness(&self.0)
    }

    pub fn alpha(&self) -> f32 {
        ffi::material_alpha(&self.0)
    }

    /// Multiplier for the emissive color. Only saved in Fallout 3 and later files; 1.0 otherwise.
    pub fn emissive_multiple(&self) -> f32 {
        ffi::material_emissive_multiple(&self.0)
    }

    /// Whether the emissive color isn't black (exactly zero).
    pub fn is_emissive(&self) -> bool {
        ffi::material_is_emissive(&self.0)
    }

    /// Whether the specular color isn't black (exactly zero).
    pub fn has_specular(&self) -> bool {
        ffi::material_has_specular(&self.0)
    }

    pub fn set_ambient(&mut self, color: [f32; 3]) {
        ffi::set_material_ambient(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_diffuse(&mut self, color: [f32; 3]) {
        ffi::set_material_diffuse(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_specular(&mut self, color: [f32; 3]) {
        ffi::set_material_specular(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_emissive(&mut self, color: [f32; 3]) {
        ffi::set_material_emissive(self.ffi_mut(), rgb_to_ffi(color));
    }

    pub fn set_glossiness(&mut self, glossiness: f32) {
        ffi::set_material_glossiness(self.ffi_mut(), glossiness);
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        ffi::set_material_alpha(self.ffi_mut(), alpha);
    }

    pub fn set_emissive_multiple(&mut self, multiple: f32) {
        ffi::set_material_emissive_multiple(self.ffi_mut(), multiple);
    }
}

impl NiAlphaProperty {
    /// The threshold of a new alpha property.
    pub const DEFAULT_THRESHOLD: u8 = 128;

    /// The blending and testing settings.
    pub fn flags(&self) -> AlphaFlags {
        AlphaFlags::from_bits(ffi::alpha_flags(&self.0))
    }

    /// The alpha test threshold. Pixels are tested against it with the
    /// [test function](AlphaFlags::test_function).
    pub fn threshold(&self) -> u8 {
        ffi::alpha_threshold(&self.0)
    }

    pub fn set_flags(&mut self, flags: AlphaFlags) {
        ffi::set_alpha_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        ffi::set_alpha_threshold(self.ffi_mut(), threshold);
    }
}

impl NiStencilProperty {
    fields! {
        /// Flags of unknown meaning. Only saved in files from version 3.0 up to 10.0.1.2, which
        /// include Morrowind's.
        legacy_raw_flags, set_legacy_raw_flags: u16 = stencil_legacy_flags, set_stencil_legacy_flags, same, same;
    }

    /// The packed settings (Fallout 3 and later).
    pub fn flags(&self) -> StencilFlags {
        StencilFlags::from_bits(ffi::stencil_flags(&self.0))
    }

    /// Whether stencil testing is on (Oblivion and older).
    pub fn enabled(&self) -> bool {
        ffi::stencil_enabled(&self.0)
    }

    /// The test function (Oblivion and older). `None` if the stored value isn't a test function.
    pub fn function(&self) -> Option<StencilTest> {
        StencilTest::try_from(ffi::stencil_function(&self.0)).ok()
    }

    /// The value tested against.
    pub fn reference(&self) -> u32 {
        ffi::stencil_reference(&self.0)
    }

    /// The mask applied before testing.
    pub fn mask(&self) -> u32 {
        ffi::stencil_mask(&self.0)
    }

    /// The action when the stencil test fails (Oblivion and older). `None` if the stored value
    /// isn't an action.
    pub fn fail_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_fail_action(&self.0)).ok()
    }

    /// The action when the stencil test passes but the depth test fails (Oblivion and older).
    /// `None` if the stored value isn't an action.
    pub fn z_fail_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_z_fail_action(&self.0)).ok()
    }

    /// The action when both tests pass (Oblivion and older). `None` if the stored value isn't an
    /// action.
    pub fn pass_action(&self) -> Option<StencilAction> {
        StencilAction::try_from(ffi::stencil_pass_action(&self.0)).ok()
    }

    /// Which faces are drawn (Oblivion and older). `None` if the stored value isn't a draw mode.
    pub fn draw_mode(&self) -> Option<DrawMode> {
        DrawMode::try_from(ffi::stencil_draw_mode(&self.0)).ok()
    }

    pub fn set_flags(&mut self, flags: StencilFlags) {
        ffi::set_stencil_flags(self.ffi_mut(), flags.bits());
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        ffi::set_stencil_enabled(self.ffi_mut(), enabled);
    }

    pub fn set_function(&mut self, function: StencilTest) {
        ffi::set_stencil_function(self.ffi_mut(), function.into());
    }

    pub fn set_reference(&mut self, reference: u32) {
        ffi::set_stencil_reference(self.ffi_mut(), reference);
    }

    pub fn set_mask(&mut self, mask: u32) {
        ffi::set_stencil_mask(self.ffi_mut(), mask);
    }

    pub fn set_fail_action(&mut self, action: StencilAction) {
        ffi::set_stencil_fail_action(self.ffi_mut(), action.into());
    }

    pub fn set_z_fail_action(&mut self, action: StencilAction) {
        ffi::set_stencil_z_fail_action(self.ffi_mut(), action.into());
    }

    pub fn set_pass_action(&mut self, action: StencilAction) {
        ffi::set_stencil_pass_action(self.ffi_mut(), action.into());
    }

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        ffi::set_stencil_draw_mode(self.ffi_mut(), mode.into());
    }
}

/// How a color and the color already drawn are weighted when blending.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum BlendFactor {
    /// 1.
    One = 0,
    /// 0.
    Zero = 1,
    /// The source color.
    SourceColor = 2,
    /// 1 minus the source color.
    InverseSourceColor = 3,
    /// The destination color.
    DestinationColor = 4,
    /// 1 minus the destination color.
    InverseDestinationColor = 5,
    /// The source alpha.
    SourceAlpha = 6,
    /// 1 minus the source alpha.
    InverseSourceAlpha = 7,
    /// The destination alpha.
    DestinationAlpha = 8,
    /// 1 minus the destination alpha.
    InverseDestinationAlpha = 9,
    /// The smaller of the source alpha and 1 minus the destination alpha.
    SourceAlphaSaturate = 10,
}

/// An alpha test function: how a pixel's alpha compares with the
/// [threshold](NiAlphaProperty::threshold) for the pixel to be drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum AlphaTest {
    /// Always drawn.
    Always = 0,
    /// Drawn if the alpha is less than the threshold.
    Less = 1,
    /// Drawn if they're equal.
    Equal = 2,
    /// Drawn if the alpha is less than or equal to the threshold.
    LessOrEqual = 3,
    /// Drawn if the alpha is greater than the threshold.
    Greater = 4,
    /// Drawn if they differ.
    NotEqual = 5,
    /// Drawn if the alpha is greater than or equal to the threshold.
    GreaterOrEqual = 6,
    /// Never drawn.
    Never = 7,
}

/// The packed blending and testing settings of an [`NiAlphaProperty`].
///
/// Bits:
/// - 0: alpha blending
/// - 1–4: source blend factor
/// - 5–8: destination blend factor
/// - 9: alpha testing
/// - 10–12: test function
/// - 13: no sorting
///
/// The `with_*` methods replace one setting and keep the others.
///
/// ```
/// # use nifly::shaders::{AlphaFlags, AlphaTest};
/// let flags = AlphaFlags::DEFAULT.with_blending(true).with_test_function(AlphaTest::GreaterOrEqual);
/// assert!(flags.blending() && flags.testing());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AlphaFlags(u16);

impl Default for AlphaFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl AlphaFlags {
    /// The flags of a new alpha property: blending off, factors
    /// [`SourceAlpha`](BlendFactor::SourceAlpha) and
    /// [`InverseSourceAlpha`](BlendFactor::InverseSourceAlpha), testing on with
    /// [`Greater`](AlphaTest::Greater), sorting on.
    pub const DEFAULT: Self = Self(4844);

    const BLENDING: BitField = (0, 1);
    const SOURCE: BitField = (1, 4);
    const DESTINATION: BitField = (5, 4);
    const TESTING: BitField = (9, 1);
    const TEST: BitField = (10, 3);
    const NO_SORTING: BitField = (13, 1);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    fn field(self, field: BitField) -> u32 {
        get_bits(self.0, field)
    }

    fn with_field(self, field: BitField, value: u32) -> Self {
        Self(set_bits(self.0, field, value))
    }

    /// Whether alpha blending is on.
    pub fn blending(self) -> bool {
        self.field(Self::BLENDING) != 0
    }

    /// The factor for the color being drawn. `None` if the bits hold 11 or more.
    pub fn source_blend(self) -> Option<BlendFactor> {
        BlendFactor::try_from(self.field(Self::SOURCE)).ok()
    }

    /// The factor for the color already drawn. `None` if the bits hold 11 or more.
    pub fn destination_blend(self) -> Option<BlendFactor> {
        BlendFactor::try_from(self.field(Self::DESTINATION)).ok()
    }

    /// Whether alpha testing is on.
    pub fn testing(self) -> bool {
        self.field(Self::TESTING) != 0
    }

    pub fn test_function(self) -> AlphaTest {
        AlphaTest::try_from(self.field(Self::TEST)).expect("3 bits are always a test function")
    }

    /// Whether the shape is drawn without being sorted by depth.
    pub fn no_sorting(self) -> bool {
        self.field(Self::NO_SORTING) != 0
    }

    #[must_use]
    pub fn with_blending(self, on: bool) -> Self {
        self.with_field(Self::BLENDING, on.into())
    }

    #[must_use]
    pub fn with_source_blend(self, factor: BlendFactor) -> Self {
        self.with_field(Self::SOURCE, factor.into())
    }

    #[must_use]
    pub fn with_destination_blend(self, factor: BlendFactor) -> Self {
        self.with_field(Self::DESTINATION, factor.into())
    }

    #[must_use]
    pub fn with_testing(self, on: bool) -> Self {
        self.with_field(Self::TESTING, on.into())
    }

    #[must_use]
    pub fn with_test_function(self, function: AlphaTest) -> Self {
        self.with_field(Self::TEST, function.into())
    }

    #[must_use]
    pub fn with_no_sorting(self, on: bool) -> Self {
        self.with_field(Self::NO_SORTING, on.into())
    }
}

/// A stencil test function: how the reference compares with the stored stencil value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum StencilTest {
    /// Never passes.
    Never = 0,
    /// Passes if the reference is less than the stored value.
    Less = 1,
    /// Passes if they're equal.
    Equal = 2,
    /// Passes if the reference is less than or equal to the stored value.
    LessOrEqual = 3,
    /// Passes if the reference is greater than the stored value.
    Greater = 4,
    /// Passes if they differ.
    NotEqual = 5,
    /// Passes if the reference is greater than or equal to the stored value.
    GreaterOrEqual = 6,
    /// Always passes.
    Always = 7,
}

/// What happens to the stored stencil value after a test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum StencilAction {
    /// Leaves it unchanged.
    Keep = 0,
    /// Sets it to 0.
    Zero = 1,
    /// Sets it to the reference.
    Replace = 2,
    /// Adds 1.
    Increment = 3,
    /// Subtracts 1.
    Decrement = 4,
    /// Inverts its bits.
    Invert = 5,
}

/// Which faces of a shape are drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum DrawMode {
    /// The application's default: counterclockwise faces, or both.
    Default = 0,
    /// Counterclockwise (front) faces.
    CounterClockwise = 1,
    /// Clockwise (back) faces.
    Clockwise = 2,
    /// Both sides.
    Both = 3,
}

/// The packed stencil settings of Fallout 3 and later files, from
/// [`NiStencilProperty::flags`].
///
/// Bits:
/// - 0: stencil testing
/// - 1–3: fail action
/// - 4–6: z-fail action
/// - 7–9: pass action
/// - 10–11: draw mode
/// - 12–14: test function
///
/// The `with_*` methods replace one setting and keep the others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StencilFlags(u16);

impl Default for StencilFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl StencilFlags {
    /// The flags of a new stencil property: testing off, [`Greater`](StencilTest::Greater),
    /// pass action [`Increment`](StencilAction::Increment), draw mode [`Both`](DrawMode::Both).
    pub const DEFAULT: Self = Self(19840);

    const FAIL: BitField = (1, 3);
    const Z_FAIL: BitField = (4, 3);
    const PASS: BitField = (7, 3);
    const DRAW: BitField = (10, 2);
    const TEST: BitField = (12, 3);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    fn field(self, field: BitField) -> u32 {
        get_bits(self.0, field)
    }

    fn with_field(self, field: BitField, value: u32) -> Self {
        Self(set_bits(self.0, field, value))
    }

    /// Whether stencil testing is on.
    pub fn enabled(self) -> bool {
        self.0 & 1 != 0
    }

    pub fn function(self) -> StencilTest {
        StencilTest::try_from(self.field(Self::TEST)).expect("3 bits are always a test function")
    }

    /// `None` if the bits hold 6 or 7.
    pub fn fail_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::FAIL)).ok()
    }

    /// `None` if the bits hold 6 or 7.
    pub fn z_fail_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::Z_FAIL)).ok()
    }

    /// `None` if the bits hold 6 or 7.
    pub fn pass_action(self) -> Option<StencilAction> {
        StencilAction::try_from(self.field(Self::PASS)).ok()
    }

    pub fn draw_mode(self) -> DrawMode {
        DrawMode::try_from(self.field(Self::DRAW)).expect("2 bits are always a draw mode")
    }

    #[must_use]
    pub fn with_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !1) | u16::from(enabled))
    }

    #[must_use]
    pub fn with_function(self, function: StencilTest) -> Self {
        self.with_field(Self::TEST, function.into())
    }

    #[must_use]
    pub fn with_fail_action(self, action: StencilAction) -> Self {
        self.with_field(Self::FAIL, action.into())
    }

    #[must_use]
    pub fn with_z_fail_action(self, action: StencilAction) -> Self {
        self.with_field(Self::Z_FAIL, action.into())
    }

    #[must_use]
    pub fn with_pass_action(self, action: StencilAction) -> Self {
        self.with_field(Self::PASS, action.into())
    }

    #[must_use]
    pub fn with_draw_mode(self, mode: DrawMode) -> Self {
        self.with_field(Self::DRAW, mode.into())
    }
}
