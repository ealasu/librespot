use std::env;
use std::io;
use std::fs::File;
use tokio_core::reactor::Core;
use env_logger;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::playback::config::PlayerConfig;
use librespot::playback::config::Bitrate;

use librespot::playback::audio_backend;
use librespot::playback::player::Player;

fn main() {
    env_logger::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    let track = SpotifyId::from_base62(&args[3]).unwrap();

    let backend = audio_backend::find(None).unwrap();

    println!("Connecting ..");
    let session = core
        .run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    let (mut player, _) = Player::new(player_config, session.clone(), None, move || {
        (backend)(None)
    });

    player.load(track, true, 0);

    println!("Playing...");
    core.run(player.get_end_of_track_future()).unwrap();

    println!("Done");
}
