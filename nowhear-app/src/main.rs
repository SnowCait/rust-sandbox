use futures::StreamExt;
use nowhear::{MediaEvent, MediaSource, MediaSourceBuilder, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let source = MediaSourceBuilder::new().build().await?;

    // Basic Example
    // let players = source.list_players().await?;
    // println!("Active players: {players:?}");

    // if let Some(player_name) = players.first() {
    //     let info = source.get_player(player_name).await?;
    //     println!("Player: {}", info.player_name);
    //     println!("State: {:?}", info.playback_state);
    //     if let Some(track) = info.current_track {
    //         println!("Track: {} - {}", track.artist.join(", "), track.title);
    //     }
    // }

    // Event Stream Example
    let mut stream = source.event_stream().await?;

    while let Some(event) = stream.next().await {
        match event {
            MediaEvent::TrackChanged { player_name: _, track } => {
                println!("🎵 {} - {}", track.artist.join(", "), track.title);
            }
            MediaEvent::StateChanged { player_name: _, state } => {
                println!("▶️ Playback state: {:?}", state);
            }
            MediaEvent::PlayerAdded { player_name } => {
                println!("➕ Player added: {}", player_name);
            }
            MediaEvent::PlayerRemoved { player_name } => {
                println!("➖ Player removed: {}", player_name);
            }
            _=>{}
        }
    }

    Ok(())
}
