use super::{AppModel, Message};
use crate::fl;
use crate::services::clipboard;
use cosmic::applet::menu_button;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget::{self, ListColumn, Widget};
use crate::services::clipboard::ClipboardEntry;

pub fn view(app: &AppModel) -> Element<'_, Message> {
    app.core
        .applet
        .icon_button("edit-copy-symbolic")
        .on_press(Message::TogglePopup)
        .into()
}

pub fn view_clipboard_entry<'a>(app: &AppModel, entry: &'a ClipboardEntry) -> Element<'a, Message> {
    match entry.content {
        clipboard::ClipboardContent::Text(ref text) => {
            let is_editable = app.editing_entry == Some(entry.widget_id.clone());
            if !is_editable {
                menu_button(widget::text::body(&entry.title))
                    .on_press(Message::CopyFromHistory(entry.widget_id.clone()))
                    .into()
            } else {
                let input_id = widget::Id::unique();
                let mut inline_input = widget::inline_input("Placeholder", &entry.title)
                    .on_input(Message::EditableInputChanged)
                    .on_submit(Message::EditableInputSubmitted)
                    .id(entry.widget_id.clone())
                    .select_on_focus(true);
                    
                inline_input.into()
            }
        },
        clipboard::ClipboardContent::Image { ref mime, ref bytes, .. } => {
            // widget::text::body(fl!("clipboard-image")).into()
            let is_editable = app.editing_entry == Some(entry.widget_id.clone());
            if !is_editable {
                menu_button(widget::text::body(&entry.title))
                    .on_press(Message::CopyFromHistory(entry.widget_id.clone()))
                    .into()
            } else {
                let input_id = widget::Id::unique();
                let mut inline_input = widget::inline_input("Placeholder", &entry.title)
                    .on_input(Message::EditableInputChanged)
                    .on_submit(Message::EditableInputSubmitted)
                    .id(entry.widget_id.clone())
                    .select_on_focus(true);

                inline_input.into()
            }
            //     clipboard::ClipboardEntry::Image {
            //         mime,
            //         bytes,
            //         thumbnail_png,
            //         ..
            //     } => {
            //         let thumb = thumbnail_png
            //             .as_ref()
            //             .map(|png| widget::image(ImageHandle::from_bytes(png.clone())));
            //
            //         let meta = widget::text::body(format!(
            //             "{} ({} KB)",
            //             mime,
            //             (bytes.len().saturating_add(1023)) / 1024
            //         ));
            //
            //         let mut col = widget::column::Column::new()
            //             .spacing(4)
            //             .align_x(Alignment::Center);
            //         if let Some(thumb) = thumb {
            //             col = col.push(thumb);
            //         }
            //         col.push(meta).into()
            //     }
        }
    }
}

pub fn create_clipboard_row<'a>(app: &AppModel, id: &cosmic::widget::Id, item: &'a ClipboardEntry) -> Element<'a, Message> {
    let label = view_clipboard_entry(&app, &item);

    // TODO : Swap out for save button if the entry is being edited
    let edit_button = match &item.content {
        clipboard::ClipboardContent::Text(clipboard_text) => {
            widget::button::icon(widget::icon::from_name("edit-symbolic").handle())
                .tooltip(fl!("edit-title"))
                .on_press(Message::EditToggled(id.clone()))
                .extra_small()
                .width(Length::Shrink)
        }
        _ => widget::button::icon(widget::icon::from_name("edit-symbolic").handle())
            .tooltip(fl!("edit-title"))
            .on_press(Message::EditToggled(id.clone()))
            .extra_small()
            .width(Length::Shrink)
    };

    let save_button = match &item.content {
        clipboard::ClipboardContent::Text(clipboard_text) => {
            widget::button::icon(widget::icon::from_name("document-save-symbolic").handle())
                .tooltip(fl!("save-title"))
                .on_press(Message::EditToggled(item.widget_id.clone()))
                .extra_small()
                .width(Length::Shrink)
        }
        _ => widget::button::icon(widget::icon::from_name("document-save-symbolic").handle())
            .tooltip(fl!("save-title"))
            .extra_small()
            .width(Length::Shrink)
    };
    let remove_button =
        widget::button::icon(widget::icon::from_name("edit-delete-symbolic").handle())
            .tooltip(fl!("remove"))
            .on_press(Message::RemoveHistory(item.widget_id.clone()))
            .extra_small()
            .width(Length::Shrink);


    widget::row::Row::new()
        .spacing(8)
        .padding([4, 0])
        .align_y(Alignment::Center)
        .push(label)
        .push(if app.editing_entry == Some(item.widget_id.clone()) { save_button } else { edit_button })
        .push(remove_button)
        .width(Length::Fill)
        .into()
}

pub fn view_window(app: &AppModel, _id: Id) -> Element<'_, Message> {
    let mut content = widget::list_column().padding([8, 0]).spacing(0);

    if app.history.is_empty() {
        return content.add(widget::text::body(fl!("empty"))).into();
    }

    let clear_history = menu_button(widget::text::body("Clear Clipboard History"))
        .on_press(Message::ClearHistory);

    let clear_history_row = widget::row::Row::new()
        .spacing(8)
        .padding([4, 0])
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .push(clear_history);

    let content = content.add(clear_history_row);

    let content = app.history.iter().fold(content, |accumulator, entry_row| {
        let (id, item) = entry_row;
        let element = accumulator.add(create_clipboard_row(&app, id, item));

        element
    });

    app.core.applet.popup_container(
        content
            .apply(widget::container)
                .padding([8, 8])
            .apply(widget::scrollable)
                .width(Length::Fill)
                .height(if app.history.len() > 5 { Length::Fixed(400.0) } else { Length::Shrink })
    ).into()
}