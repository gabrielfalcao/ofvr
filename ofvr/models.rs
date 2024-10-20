use serde::{Deserialize, Serialize};


pub struct FamilarPersona {}
pub struct UnfamiliarPersona {}

pub enum FamilarPersonaTraits {

}
pub enum UnfamilarPersonaTraits {
}

pub enum Persona {
    Familar(FamilarPersona),
    Unfamiliar(UnfamiliarPersona),
}
pub struct Message {
    source: Persona,
}
