use super::TrackId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct PlaylistId(pub Uuid);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PlaylistKind {
    User,
    Folder,
    Generated,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Playlist {
    pub id: PlaylistId,
    pub name: String,
    pub kind: PlaylistKind,
    pub tracks: Vec<TrackId>,
}
