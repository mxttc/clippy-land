use crate::services::clipboard;
use cosmic::iced::window::Id;
use cosmic::iced_core;

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(iced_core::window::Id),
    ClipboardChanged(clipboard::ClipboardEntry),
    ClearHistory,
    RemoveHistory(cosmic::widget::Id), // TODO: Verify remove history still works after this change
    CopyFromHistory(cosmic::widget::Id),
    TogglePinEntry(cosmic::widget::Id),
    SearchInputToggled(bool),
    SearchInputChanged(String),
    EditableInputToggled(bool),
    EditableInputChanged(String),
    EditToggled(cosmic::widget::Id),
    EditableInputSubmitted(String),
}
