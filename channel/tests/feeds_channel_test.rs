use channel_io::*;
use codec::Encode;
use gstd::{ActorId, BTreeSet};
use gtest::{Log, Program, System};
use router_io::*;

const CHANNEL_ID: u64 = 2;
const ROUTER_ID: u64 = 1;
const OWNER: [u8; 32] = [1; 32];
const SUBSCRIBERS: &[u64] = &[10, 11, 12, 13, 14];

fn init_router(sys: &System) {
    let router = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/gear_feeds_router.wasm",
    );

    let res = router.send_bytes(OWNER, "INIT");

    assert!(res.log().is_empty());
}
fn init_channel(sys: &System) {
    let channel = Program::current(sys);

    let res = channel.send(
        OWNER,
        ChannelInit {
            router_contract_id: ROUTER_ID.into(),
        },
    );
    let log = Log::builder()
        .dest(OWNER)
        .payload(ChannelOutput::SubscriberAdded(OWNER.into()));
    assert!(res.contains(&log));
}

#[test]
fn channels_initialization() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    init_router(&sys);

    // upload and init 2 channels
    init_channel(&sys);
    init_channel(&sys);

    // check that channels were registered at router contract
    let router = sys.get_program(ROUTER_ID);
    let mut expected_channels: Vec<Channel> = Vec::new();

    // first channel info
    let mut channel = Channel {
        id: 2.into(),
        name: String::from("Channel-Coolest-Name"),
        owner_id: OWNER.into(),
        description: String::from("Channel-Coolest-Description"),
    };
    // read info about that channel from the router contract
    let channel_info: RouterStateReply = router
        .meta_state(&RouterState::Channel(channel.id))
        .expect("Meta_state failed");
    assert_eq!(channel_info, RouterStateReply::Channel(channel.clone()));

    expected_channels.push(channel.clone());
    // change id to get the second channel info
    channel.id = 3.into();
    let channel_info: RouterStateReply = router
        .meta_state(&RouterState::Channel(channel.id))
        .expect("Meta_state failed");
    assert_eq!(channel_info, RouterStateReply::Channel(channel.clone()));

    expected_channels.push(channel);

    // read state from the router contract
    let channels: RouterStateReply = router
        .meta_state(&RouterState::AllChannels)
        .expect("Meta_state failed");
    assert_eq!(channels, RouterStateReply::AllChannels(expected_channels));

    // check that OWNER subscribes to 2 channels
    let mut expected_channels: BTreeSet<ActorId> = BTreeSet::new();
    expected_channels.insert(2.into());
    expected_channels.insert(3.into());

    let channels: RouterStateReply = router
        .meta_state(RouterState::SubscribedToChannels(OWNER.into()))
        .expect("Meta_state failed");
    assert_eq!(
        channels,
        RouterStateReply::SubscribedToChannels(expected_channels)
    );
}

#[test]
fn subscriptions() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    init_router(&sys);

    // upload and init a channel
    init_channel(&sys);
    let channel = sys.get_program(CHANNEL_ID);
    let router = sys.get_program(ROUTER_ID);
    let channel_id: ActorId = CHANNEL_ID.into();
    // add subscribers
    for subscriber in SUBSCRIBERS {
        let res = channel.send(*subscriber, ChannelAction::Subscribe);
        let log = Log::builder()
            .dest(*subscriber)
            .payload(ChannelOutput::SubscriberAdded((*subscriber).into()));
        assert!(res.contains(&log));
        // check a subscription in the router contract
        let subscribed_to_channels: RouterStateReply = router
            .meta_state(RouterState::SubscribedToChannels((*subscriber).into()))
            .expect("Meta_state failed");
        
        assert_eq!(
            subscribed_to_channels,
            RouterStateReply::SubscribedToChannels(BTreeSet::from([channel_id]))
        );
    }

    // must fail since already subscribed to the channel
    let res = channel.send(SUBSCRIBERS[0], ChannelAction::Subscribe);
    assert!(res.main_failed());

    // unsubscribe
    let res = channel.send(SUBSCRIBERS[1], ChannelAction::Unsubscribe);
    let log = Log::builder()
        .dest(SUBSCRIBERS[1])
        .payload(ChannelOutput::SubscriberRemoved(SUBSCRIBERS[1].into()));
    assert!(res.contains(&log));
    // check a subscription in the router contract
    let subscribed_to_channels: RouterStateReply = router
        .meta_state(RouterState::SubscribedToChannels(SUBSCRIBERS[1].into()))
        .expect("Meta_state failed");
    
    assert_eq!(
        subscribed_to_channels,
        RouterStateReply::SubscribedToChannels(BTreeSet::new())
    );

    // must fail since a sender does not subscribe to channel
    let res = channel.send(SUBSCRIBERS[1], ChannelAction::Unsubscribe);
    assert!(res.main_failed());
}

#[test]
fn post() {
    let sys = System::new();
    sys.init_logger();

    // upload and init a router program
    init_router(&sys);

    // upload and init a channel
    init_channel(&sys);
    let channel = sys.get_program(CHANNEL_ID);
    let res = channel.send(OWNER, ChannelAction::Post(String::from("Hello")));

    // check messages in contract
    let mut expected_messages: Vec<Message> = Vec::new();
    let mut message = Message {
        text: String::from("Channel \"Channel-Coolest-Name\" was created"),
        timestamp: 0
    };
    expected_messages.push(message.clone());
    message.text = String::from("Hello");
    expected_messages.push(message.clone());

    let messages: Vec<Message> = channel.meta_state(()).expect("Meta_state failed");
    assert_eq!(expected_messages, messages);

    // check log
    let log = Log::builder().dest(OWNER).payload(ChannelOutput::MessagePosted(message.clone()));
    assert!(res.contains(&log));
    let log = Log::builder().dest(OWNER).payload(ChannelOutput::SingleMessage(message));
    assert!(res.contains(&log));
}