/// Structure containing all state related to pixel processing.
///
/// For a list of things contained within the pixel state, see:
/// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-pixel-states-with-a-stateblock
pub struct PixelState {}

impl PixelState {
    /// Creates a new pixel state structure which initially tracks no state.
    pub(super) fn empty() -> Self {
        Self {}
    }
}

impl Default for PixelState {
    fn default() -> Self {
        Self {}
    }
}
