#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProgramMetadata;

// 1. Define actions, events, errors and state for your metadata.
impl Metadata for ProgramMetadata {
    type Init = In<InitStruct>;
    type Handle = InOut<Actions, Result<Events, Errors>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<Query, QueryReply>;
}

// 2. Create your init Struct(Optional)
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitStruct {
    // Example:
    pub ft_program_id: ActorId,
}

// 3. Create your own Actions
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Actions {
    // Add Actions
    FirstAction,               // Example an action with a simple input
    SecondAction(String),      // Example an action with String input
    ThirdAction(u128),         // Example an action with a u128 input
    Fourthaction(CustomInput), // Example an action with a custom input
    Fifthaction {
        // Example an action with a custom input
        first_field: u128,
        second_field: Vec<ActorId>,
    },
}

// Example of a custom input for an action
#[derive(Debug, Decode, Encode,  Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CustomInput {
   pub firstfield: String,
   pub secondfield: u128,
   pub thirdfield: ActorId,
}

// 4. Create your own Events
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Events {
    // Add Events(Example)
    FirstEvent,          // Example an event with a simple input
    SecondEvent(String), // Example an event with a u128 input
    ThirdEvent(u128),    // Example an event with String input
    FourtEvent {
        first_field: ActorId,
        second_field: Vec<ActorId>, // Example an event with a custom input
    },
}

// 5. Create your own Errors
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
    // Add errors(Example)
    FirstError,
    SecondError,
    ThirdErrors,
    FourtErrors,
}

// 6. Create your own Struct
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoCustomStruct {
    pub firstfield: String,
    pub secondfield: String,
    pub thirdfield: u128,
    pub fourthfield: Vec<(ActorId, CustomInput)> ,
    pub fifthfield: Vec<(ActorId, u128)> ,
}


// 7. Create your State Querys
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Query {
    All,
    FirstField,
    SecondField,
    ThirdField,
    FourthField{ actor_id: ActorId },
    FifthField{ actor_id: ActorId }
    
}

// 8. Create your State Query Replys
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum QueryReply {
    All(IoCustomStruct),
    FirstField(String),
    SecondField(String),
    ThirdField(u128),
    FourthField(Option<CustomInput>),
    FifthField(Option<u128>)
 
}
