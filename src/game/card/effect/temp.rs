use crate::prelude::*;

#[derive(Event, Debug)]
pub struct TempChangeAction {
    pub change: i8,
}
