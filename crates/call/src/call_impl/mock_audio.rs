use gpui::App;

pub struct Audio;

impl Audio {
    pub fn play_sound(_: Sound, _: &mut App) {}
    pub fn end_call(_: &mut App) {}
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    Joined,
    GuestJoined,
    Leave,
    Mute,
    Unmute,
    StartScreenshare,
    StopScreenshare,
    AgentDone,
}
