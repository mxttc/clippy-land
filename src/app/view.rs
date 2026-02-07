use super::{AppModel, Message};
use crate::fl;
use crate::services::clipboard;
use cosmic::applet::menu_button;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget::{self, Widget};
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
                widget::text::body(&entry.title).into()
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
            widget::text::body(fl!("clipboard-image")).into()
        }
    }
}

pub fn view_window(app: &AppModel, _id: Id) -> Element<'_, Message> {
    let mut content = widget::list_column().padding([8, 0]).spacing(0);

    if app.history.is_empty() {
        content = content.add(widget::text::body(fl!("empty")));
    } else {
        for (idx, item) in app.history.iter().enumerate() {
            // let label: Element<'_, Message> = match item {
            //     clipboard::ClipboardEntry::ClipboardContent::Text(clipboard_text) => {
            //         let is_editable = app.editing_entry == Some(clipboard_text.widget_id.clone());
            //         if !is_editable {
            //             widget::text::body(&clipboard_text.title).into()
            //         } else {
            //             let input_id = widget::Id::unique();
            //             let mut inline_input = widget::inline_input("Placeholder", &clipboard_text.title)
            //                 .on_input(Message::EditableInputChanged)
            //                 .on_submit(Message::EditableInputSubmitted)
            //                 .id(clipboard_text.widget_id.clone())
            //                 .select_on_focus(true);
            //
            //             inline_input.into()
            //         }
            //
            //     }
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
            // };

            let label = view_clipboard_entry(&app, item);

            // TODO : Swap out for save button if the entry is being edited
            let edit_button = match &item.content {
                clipboard::ClipboardContent::Text(clipboard_text) => {
                    widget::button::icon(widget::icon::from_name("edit-symbolic").handle())
                        .tooltip(fl!("edit-title"))
                        .on_press(Message::EditToggled(item.widget_id.clone()))
                        .extra_small()
                        .width(Length::Shrink)
                }
                _ => widget::button::icon(widget::icon::from_name("edit-symbolic").handle())
                    .tooltip(fl!("edit-title"))
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
        
        // TODO : Change RemoveHistory to work with widgetId
            let remove_button =
                widget::button::icon(widget::icon::from_name("edit-delete-symbolic").handle())
                    .tooltip(fl!("remove"))
                    .on_press(Message::RemoveHistory(idx))
                    .extra_small()
                    .width(Length::Shrink);
            content = content.add(
                widget::row::Row::new()
                    .spacing(8)
                    .padding([4, 0])
                    .align_y(Alignment::Center)
                    .push(label)
                    .push(edit_button)
                    .push(remove_button)
                    .width(Length::Fill),
            );
        }
    }

    // Add a fixed height with scrolling when there are many items
    let content = if app.history.len() > 5 {
        widget::scrollable(content)
            .width(Length::Fill)
            .height(Length::Fixed(400.0))
    } else {
        widget::scrollable(content)
            .width(Length::Fill)
            .height(Length::Shrink)
    };

    let content = widget::container(content).padding([8, 8]);

    app.core.applet.popup_container(content).into()
}