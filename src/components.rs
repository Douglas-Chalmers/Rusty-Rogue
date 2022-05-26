pub use crate::prelude::*;
use crate::{ColorPair, FontCharType};

#[derive(Clone, Copy, Debug, PartialEq)]

pub struct Render{
    pub color: ColorPair,
    pub glyph: FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

