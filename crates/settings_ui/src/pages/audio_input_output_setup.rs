use gpui::{AnyElement, App, SharedString, Window};
use settings::{AudioInputDeviceName, AudioOutputDeviceName};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, FluentBuilder, IconPosition, IntoElement};

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile};

pub(crate) const SYSTEM_DEFAULT: &str = "System Default";

pub fn render_input_audio_device_dropdown(
    _field: SettingField<AudioInputDeviceName>,
    _file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    title: &'static str,
    description: &'static str,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let menu = ContextMenu::build(window, cx, |menu, _, _| {
        menu.toggleable_entry(SYSTEM_DEFAULT, true, IconPosition::Start, None, |_, _| {})
    });

    DropdownMenu::new("input-audio-device-dropdown", SYSTEM_DEFAULT, menu)
        .style(DropdownStyle::Outlined)
        .full_width(true)
        .aria_label(SharedString::new_static(title))
        .when(!description.is_empty(), |this| {
            this.aria_description(SharedString::new_static(description))
        })
        .into_any_element()
}

pub fn render_output_audio_device_dropdown(
    _field: SettingField<AudioOutputDeviceName>,
    _file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    title: &'static str,
    description: &'static str,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let menu = ContextMenu::build(window, cx, |menu, _, _| {
        menu.toggleable_entry(SYSTEM_DEFAULT, true, IconPosition::Start, None, |_, _| {})
    });

    DropdownMenu::new("output-audio-device-dropdown", SYSTEM_DEFAULT, menu)
        .style(DropdownStyle::Outlined)
        .full_width(true)
        .aria_label(SharedString::new_static(title))
        .when(!description.is_empty(), |this| {
            this.aria_description(SharedString::new_static(description))
        })
        .into_any_element()
}
