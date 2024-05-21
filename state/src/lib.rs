#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    // Add your State functions

    pub fn state(state: State) -> CustomStruct {
        state
    }
}
