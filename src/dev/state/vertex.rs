/// Structure containing all state related to vertex processing.
///
/// For a list of things contained within the vertex state, see:
/// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-vertex-states-with-a-stateblock
pub struct VertexState {}

impl VertexState {
    /// Creates a new vertex state structure which initially tracks no state.
    pub(super) fn empty() -> Self {
        Self {}
    }
}

impl Default for VertexState {
    fn default() -> Self {
        Self {}
    }
}
