mod feature_flags;

pub(crate) use feature_flags::render_feature_flags_page;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkillCreatorOpenMode {
    Form,
    Url { initial_url: Option<String> },
}

pub(crate) fn skill_url_from_clipboard(_cx: &gpui::App) -> Option<String> {
    None
}

