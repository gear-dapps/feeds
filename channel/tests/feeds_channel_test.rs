use channel_io::*;
use codec::Encode;
use gtest::{Program, System};

const CHANNEL_ID: u64 = 2;
const ROUTER_ID: u64 = 1;
const OWNER: [u8; 32] = [1; 32];
const SUBSCRIBER: [u8; 32] = [2; 32];

fn init_with_msg(sys: &System) {

    let router = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/gear_feeds_router.wasm",
    );

    let res = router.send_bytes(
        OWNER,
        "INIT",
    );

    assert!(res.log().is_empty());

    let channel = Program::current(sys);

    // ⚠️ TODO: Change the text message
    // let res = channel.send(
    //     OWNER,
    //     ChannelInit {
    //         router_contract_id: ROUTER_ID.into(),
    //     },
    // );
    // assert!(res.log().is_empty());
}

#[test]
fn add_subscriber() {
    let sys = System::new();
    sys.init_logger();
    init_with_msg(&sys);
    let router = sys.get_program(ROUTER_ID);
    let channel = sys.get_program(CHANNEL_ID);
    //let res = channel.send(OWNER, ChannelAction::Subscribe);
     // ⚠️ TODO: Change the text message
    let res = channel.send(
        OWNER,
        ChannelInit {
            router_contract_id: ROUTER_ID.into(),
        },
    );

    let res = channel.send(OWNER, ChannelAction::Subscribe);
    // ⚠️ TODO: Change the channel name and description
}

// #[test]
// fn subscribe_and_unsubscribe() {
//     let sys = System::new();
//     sys.init_logger();
//     init_with_msg(&sys);

//     let feeds_channel = sys.get_program(1);
//     // subscribes to the channel
//     feeds_channel.send(SUBSCRIBER, ChannelAction::Subscribe);

//     // ⚠️ TODO: Change the post message
//     let res = feeds_channel.send(OWNER, ChannelAction::Post("hello".to_string()));

//     // checks that the message was sent to the owner
//     // ⚠️ TODO: Change the received message
//     assert!(res.contains(&(
//         OWNER,
//         ChannelOutput::SingleMessage(Message {
//             text: "hello".to_string(),
//             timestamp: 0,
//         })
//         .encode()
//     )));

//     // checks that the message was sent to the subscriber
//     // ⚠️ TODO: Change the received message
//     assert!(res.contains(&(
//         SUBSCRIBER,
//         ChannelOutput::SingleMessage(Message {
//             text: "hello".to_string(),
//             timestamp: 0,
//         })
//         .encode()
//     )));

//     // unsubscribes from the channel
//     feeds_channel.send(SUBSCRIBER, ChannelAction::Unsubscribe);

//     let res = feeds_channel.send(OWNER, ChannelAction::Post("hello".to_string()));

//     // checks that the subscriber didn't receive the message
//     assert!(!res.contains(&(
//         SUBSCRIBER,
//         ChannelOutput::SingleMessage(Message {
//             text: "hello".to_string(),
//             timestamp: 0,
//         })
//         .encode()
//     )));
// }

// #[test]
// fn check_for_failure() {
//     let sys = System::new();
//     sys.init_logger();
//     init_with_msg(&sys);

//     let feeds_channel = sys.get_program(1);

//     // must fails since a subscriber is not the channel owner
//     let res = feeds_channel.send(SUBSCRIBER, ChannelAction::Post("hello".to_string()));
//     assert!(res.main_failed());
// }
