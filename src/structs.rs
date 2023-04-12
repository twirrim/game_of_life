pub type Colony = Vec<Vec<State>>;

// Should probably implement methods around this
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub struct State {
    pub neighbour_count: u8,
    pub alive: bool,
}
