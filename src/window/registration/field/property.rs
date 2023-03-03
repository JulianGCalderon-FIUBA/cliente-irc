use std::ops::Deref;
use strum_macros::EnumString;

#[derive(EnumString)]
pub enum FieldProperty {
    Name,
    Input,
    Default,
    Password,
}
