use gpui::{App, Context, Entity, WeakEntity};
use crate::worktree_store::WorktreeStore;
use crate::Project;

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

    pub fn remote(
        _project_id: u64,
        _remote: Entity<remote::RemoteClient>,
        _worktree_store: Entity<WorktreeStore>,
        _project: Option<WeakEntity<Project>>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self
    }
}
