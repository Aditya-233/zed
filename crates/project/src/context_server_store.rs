use crate::Project;
use crate::worktree_store::WorktreeStore;
use gpui::{App, Context, Entity, WeakEntity};

pub fn init(_cx: &mut App) {}

pub struct ContextServerStore;

impl ContextServerStore {
    pub fn local(
        _worktree_store: Entity<WorktreeStore>,
        _project: Option<WeakEntity<Project>>,
        _is_collab: bool,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self
    }
}
