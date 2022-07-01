use gstd::{
    prelude::{String, Vec},
    ActorId,
};

use crate::{Message, Channel, CHANNEL};
use circular_buffer::CircularBuffer;
use codec::Encode;

#[derive(Clone)]
pub struct State {
    owner_id: ActorId,
    name: String,
    description: String,
    subscribers: Vec<ActorId>,
    messages: CircularBuffer<Message>,
}


#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let channel = CHANNEL.get_or_insert(Default::default());
    let messages: Vec<Message> = crate::STATE
        .messages
        .clone()
        .map(|v| v.into_iter().collect())
        .unwrap_or_default();
    let encoded = messages.encode();
    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
