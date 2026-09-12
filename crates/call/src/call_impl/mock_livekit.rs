use collections::HashMap;
use gpui::{App, Context, Empty, EventEmitter, IntoElement, Render, Window};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TrackSid(pub String);

impl TrackSid {
    pub fn new(sid: impl Into<String>) -> Self {
        Self(sid.into())
    }
}

#[derive(Clone, Debug)]
pub struct RemoteAudioTrack;

impl RemoteAudioTrack {
    pub fn sid(&self) -> TrackSid {
        TrackSid(String::new())
    }
}

#[derive(Clone, Debug)]
pub struct RemoteVideoTrack;

impl RemoteVideoTrack {
    pub fn sid(&self) -> TrackSid {
        TrackSid(String::new())
    }
}

#[derive(Clone, Debug)]
pub enum RemoteTrack {
    Audio(RemoteAudioTrack),
    Video(RemoteVideoTrack),
}

#[derive(Clone, Debug, Default)]
pub struct AudioStream;

impl AudioStream {
    pub fn remote_playback_stats(&self) -> Option<RemoteAudioPlaybackStats> {
        None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Poor,
    Lost,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct RemoteAudioPlaybackStats {
    pub frames_received: u64,
    pub frames_dropped: u64,
    pub queue_underflows: u64,
    pub current_queue_depth: u64,
    pub maximum_queue_depth: u64,
}

#[derive(Clone, Debug, Default)]
pub struct SessionStats {
    pub publisher_stats: Vec<RtcStats>,
    pub subscriber_stats: Vec<RtcStats>,
}

#[derive(Clone, Debug, Default)]
pub struct RtcStats;

pub struct RemoteVideoTrackView;

impl RemoteVideoTrackView {
    pub fn new(_track: RemoteVideoTrack, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for RemoteVideoTrackView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(Debug)]
pub enum RemoteVideoTrackViewEvent {
    Close,
}

impl EventEmitter<RemoteVideoTrackViewEvent> for RemoteVideoTrackView {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ParticipantIdentity(pub String);

#[derive(Clone, Debug)]
pub struct LocalTrackPublication;

impl LocalTrackPublication {
    pub fn sid(&self) -> TrackSid {
        TrackSid(String::new())
    }
    pub fn mute(&self, _cx: &App) {}
    pub fn unmute(&self, _cx: &App) {}
}

#[derive(Debug)]
pub enum RoomEvent {
    TrackSubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: Arc<RemoteParticipant>,
    },
    TrackUnsubscribed {
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: Arc<RemoteParticipant>,
    },
    ActiveSpeakersChanged {
        speakers: Vec<ParticipantIdentity>,
    },
    TrackMuted {
        participant: Arc<RemoteParticipant>,
        publication: RemoteTrackPublication,
    },
    TrackUnmuted {
        participant: Arc<RemoteParticipant>,
        publication: RemoteTrackPublication,
    },
    LocalTrackUnpublished {
        publication: LocalTrackPublication,
    },
    LocalTrackPublished {
        publication: LocalTrackPublication,
    },
    Disconnected {
        reason: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub struct RemoteTrackPublication;

impl RemoteTrackPublication {
    pub fn is_audio(&self) -> bool {
        false
    }
    pub fn is_muted(&self) -> bool {
        false
    }
    pub fn set_enabled(&self, _enabled: bool, _cx: &App) {}
    pub fn sid(&self) -> TrackSid {
        TrackSid(String::new())
    }
}

pub struct Room {
    connection_state: ConnectionState,
}

impl Room {
    pub fn new() -> Self {
        Self {
            connection_state: ConnectionState::Disconnected,
        }
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.connection_state
    }

    pub fn local_participant(&self) -> LocalParticipant {
        LocalParticipant
    }

    pub fn remote_participants(&self) -> HashMap<ParticipantIdentity, Arc<RemoteParticipant>> {
        HashMap::default()
    }

    pub fn play_remote_audio_track(
        &self,
        _track: &RemoteAudioTrack,
        _cx: &App,
    ) -> anyhow::Result<AudioStream> {
        Ok(AudioStream)
    }
}

pub struct LocalParticipant;

impl LocalParticipant {
    pub async fn unpublish_track(&self, _sid: TrackSid, _cx: &App) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct RemoteParticipant;

impl RemoteParticipant {
    pub fn identity(&self) -> ParticipantIdentity {
        ParticipantIdentity(String::new())
    }
}
