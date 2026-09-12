use std::rc::Rc;
use gpui::{App, Context, IntoElement, Window, div};
use crate::TitleBar;

pub fn toggle_screen_sharing(
    _screen: anyhow::Result<Option<Rc<dyn gpui::ScreenCaptureSource>>>,
    _window: &mut Window,
    _cx: &mut App,
) {}

pub fn toggle_mute(_cx: &mut App) {}

pub fn toggle_deafen(_cx: &mut App) {}

impl TitleBar {
    pub(crate) fn render_collaborator_list(
        &self,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().id("collaborator-list")
    }
}
