use crate::services::clipboard;
use cosmic::iced::window::Id;

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ClipboardChanged(clipboard::ClipboardEntry),
    RemoveHistory(usize),
    CopyFromHistory(usize),
    EditableInputToggled(bool),
    EditableInputChanged(String),
    EditToggled(cosmic::widget::Id),
    EditableInputSubmitted(String),
}
