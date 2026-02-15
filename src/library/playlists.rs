use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::audio::TrackId;

#[derive(
    Clone, Copy, Hash, Eq, PartialEq,
    Serialize, Deserialize, Debug
)]
pub struct PlaylistId(pub Uuid);

#[derive(Serialize, Deserialize, Clone)]
pub enum PlaylistKind {
    User,
    Folder,
    Generated,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub id: PlaylistId,
    pub name: String,
    pub kind: PlaylistKind,
    pub tracks: Vec<TrackId>,
}