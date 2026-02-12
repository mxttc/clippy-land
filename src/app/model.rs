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
    pub(super) pinned_clipboard_entries: IndexMap<Id, ClipboardEntry>,
    pub(super) clipboard_entries: IndexMap<Id, ClipboardEntry>, //<clipboard::ClipboardEntry>,
    pub(super) search_filter: String,
    pub(super) editing_entry: Option<Id>,
}