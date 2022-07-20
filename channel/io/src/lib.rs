#![no_std]
use gstd::{exec, prelude::String, ActorId};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct ChannelInit {
    pub router_contract_id: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ChannelAction {
    //  Register,
    Subscribe,
    Unsubscribe,
    Post(String),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ChannelOutput {
    SingleMessage(Message),
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Default)]
pub struct Message {
    pub text: String,
    pub timestamp: u32,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timestamp: exec::block_height(),
        }
    }
}
