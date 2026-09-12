use gpui::{AnyElement, Context, ScrollHandle, Window, div, prelude::*};
use ui::Label;

use crate::SettingsWindow;

pub(crate) fn render_edit_prediction_setup_page(
    _settings_window: &SettingsWindow,
    _scroll_handle: &ScrollHandle,
    _window: &mut Window,
    _cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    div()
        .p_6()
        .child(Label::new("Edit prediction is disabled in this minimal build."))
        .into_any_element()
}
