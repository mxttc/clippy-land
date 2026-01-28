use crate::services::clipboard;
use cosmic::iced::window::Id;
use std::collections::VecDeque;

/// The application model stores app-specific state used to describe its interface
#[derive(Default)]
pub struct AppModel {
    pub(super) core: cosmic::Core,
    pub(super) popup: Option<Id>,
    /// Latest clipboard entries, newest-first.
    pub(super) history: VecDeque<clipboard::ClipboardEntry>,
}
