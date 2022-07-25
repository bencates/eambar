use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
}
