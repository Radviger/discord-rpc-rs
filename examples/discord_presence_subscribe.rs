extern crate simplelog;
extern crate discord_rpc;

use std::{thread, time};
use simplelog::*;
use discord_rpc::{
    Client as DiscordRPC,
    models::Event,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(610434593505542149);

    drpc.start();

    drpc.subscribe(Event::ActivityJoin, |j| j
        .secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivitySpectate, |s| s
        .secret("123456"))
        .expect("Failed to subscribe to event");

    drpc.subscribe(Event::ActivityJoinRequest, |s| s)
        .expect("Failed to subscribe to event");

    drpc.unsubscribe(Event::ActivityJoinRequest, |j| j)
        .expect("Failed to unsubscribe from event");

    drpc.set_activity(|a| {
        a.details("Играет на сервере")
            .assets(|ast| ast.small_image("logo"))
            .state("Alpha")
            .party(|p| p.size((25, 100)))
            .timestamps(|t| t.start(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()))
            .secrets(|s| s.join("SERIOUS SHIT"))
    })
        .expect("failed to update activity");

    loop { thread::sleep(time::Duration::from_millis(500)); }
}
