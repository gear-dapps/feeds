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

    let res = router.send_bytes(OWNER, "INIT");

    assert!(res.log().is_empty());

    let channel = Program::current(sys);

    //⚠️ TODO: Change the text message
    let res = channel.send(
        OWNER,
        ChannelInit {
            router_contract_id: ROUTER_ID.into(),
        },
    );
    assert!(res.log().is_empty());
}

#[test]
fn add_subscriber() {
    let sys = System::new();
    sys.init_logger();
    init_with_msg(&sys);
    let channel = sys.get_program(CHANNEL_ID);
    //   let res = channel.send(OWNER, ChannelAction::Register);

    //let res = channel.send(OWNER, ChannelAction::Subscribe);
    // ⚠️ TODO: Change the channel name and description
}
