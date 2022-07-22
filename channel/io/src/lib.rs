#![no_std]
use gstd::{exec, msg, prelude::String, ActorId};

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
    pub owner: ActorId,
    pub text: String,
    pub timestamp: u32,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self {
            owner: msg::source(),
            text,
            timestamp: exec::block_height(),
        }
    }
}
