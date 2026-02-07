use super::{AppModel, Message};
use crate::services::clipboard;
use cosmic::iced::Subscription;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use futures_util::SinkExt;
use std::time::Duration;
use cosmic::Action;
use cosmic::widget::Id;
use crate::services::clipboard::ClipboardEntry;

const MAX_HISTORY: usize = 30;

pub fn subscription(_app: &AppModel) -> Subscription<Message> {
    struct ClipboardSubscription;

    Subscription::batch(vec![Subscription::run_with_id(
        std::any::TypeId::of::<ClipboardSubscription>(),
        cosmic::iced::stream::channel(1, move |mut channel| async move {
            let mut last_seen: Option<clipboard::ClipboardFingerprint> = None;

            loop {
                tokio::time::sleep(Duration::from_millis(500)).await;

                let next = tokio::task::spawn_blocking(clipboard::read_clipboard_entry)
                    .await
                    .ok()
                    .flatten();

                let Some(next) = next else {
                    continue;
                };

                let next_fp = next.content.fingerprint();
                if last_seen.as_ref() == Some(&next_fp) {
                    continue;
                }

                last_seen = Some(next_fp);

                if channel.send(Message::ClipboardChanged(next)).await.is_err() {
                    break;
                }
            }
        }),
    )])
}

pub fn update(app: &mut AppModel, message: Message) -> Task<cosmic::Action<Message>> {
    match message {
        Message::ClipboardChanged(entry) => if let Some(value) = on_clipboard_changed(app, &entry) {
            return value;
        }
        Message::CopyFromHistory(index) => on_copy_from_history(app, index),
        Message::RemoveHistory(index) => on_remove_from_history(app, index),
        Message::EditToggled(widget_id) => if let Some(value) = on_edit_toggled(app, widget_id) {
            return value;
        }
        Message::EditableInputChanged(new_value) => {
            // TODO : Consider using a HashMap based on the widgetId for the main tracking of items
            app.history.iter_mut().for_each(|entry| {
                if let clipboard::ClipboardContent::Text(_) = &entry.content {
                    if Some(entry.widget_id.clone()) == app.editing_entry {
                        entry.title = new_value.clone();
                    }
                }
            });
        }
        Message::EditableInputSubmitted(_) => {
            app.editing_entry = None;
        }
        Message::TogglePopup => if let Some(value) = on_toggle_popup(app) {
            return value;
        }
        Message::PopupClosed(id) => {
            if app.popup.as_ref() == Some(&id) {
                app.popup = None;
            }
        }
        Message::EditableInputToggled(_) => {
            println!("Toggle edit mode for entry");
        }
    }
    Task::none()
}

fn on_toggle_popup(app: &mut AppModel) -> Option<Task<Action<Message>>> {
    return Some(if let Some(p) = app.popup.take() {
        destroy_popup(p)
    } else {
        let new_id = cosmic::iced::window::Id::unique();
        app.popup.replace(new_id);
        let popup_settings = app.core.applet.get_popup_settings(
            app.core.main_window_id().unwrap(),
            new_id,
            None,
            None,
            None,
        );
        get_popup(popup_settings)
    });
    None
}

fn on_edit_toggled(app: &mut AppModel, widget_id: Id) -> Option<Task<Action<Message>>> {
    if app.editing_entry == Some(widget_id.clone()) {
        app.editing_entry = None;
    } else {
        app.editing_entry = Some(widget_id.clone());

        return Some(cosmic::widget::text_input::focus(widget_id.clone()));
    }
    None
}

fn on_remove_from_history(app: &mut AppModel, index: usize) {
    let _ = app.history.remove(index);
}

fn on_copy_from_history(app: &mut AppModel, index: usize) {
    if let Some(entry) = app.history.get(index) {
        match &entry.content {
            clipboard::ClipboardContent::Text(clipboard_text) => {
                _ = clipboard::write_clipboard_text(&clipboard_text);
            }
            clipboard::ClipboardContent::Image { mime, bytes, .. } => {
                _ = clipboard::write_clipboard_image(&mime, &*bytes);
            }
        }
    }
}

fn on_clipboard_changed(app: &mut AppModel, entry: &ClipboardEntry) -> Option<Task<Action<Message>>> {
    if app
        .history
        .front()
        .is_some_and(|e: &clipboard::ClipboardEntry| e.content == entry.content)
    {
        return Some(Task::none());
    }

    if let clipboard::ClipboardContent::Text(clipboard_text) = &entry.content {
        if should_ignore_clipboard_entry(&clipboard_text) {
            return Some(Task::none());
        }
    }

    // Remove any existing entries that match to keep the history unique
    app.history.retain(|existing| existing.content != entry.content);
    app.history.push_front(entry.to_owned());
    while app.history.len() > MAX_HISTORY {
        app.history.pop_back();
    }
    None
}

fn should_ignore_clipboard_entry(entry: &str) -> bool {
    let trimmed = entry.trim();
    if trimmed.is_empty() {
        return true;
    }

    if trimmed.chars().all(|c| {
        c.is_ascii_digit() || matches!(c, ',' | '.' | ':' | ';' | '/' | '\\' | '_' | '-' | ' ')
    }) && trimmed.chars().count() <= 8
    {
        return true;
    }

    false
}
