use crate::services::clipboard;
use cosmic::widget::Id;
// use cosmic::iced::window::Id;
use std::collections::VecDeque;
use indexmap::IndexMap;
use crate::services::clipboard::ClipboardEntry;

/// The application model stores app-specific state used to describe its interface
#[derive(Default)]
pub struct AppModel {
    pub(super) core: cosmic::Core,
    pub(super) popup: Option<cosmic::iced::window::Id>,
    /// Latest clipboard entries, newest-first.
    pub(super) history: IndexMap<Id, ClipboardEntry>, //<clipboard::ClipboardEntry>,
    pub(super) editing_entry: Option<Id>,
}