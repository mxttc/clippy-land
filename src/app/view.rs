use super::{AppModel, Message};
use crate::fl;
use crate::services::clipboard;
use cosmic::applet::menu_button;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::iced::{Alignment, Length, window::Id, Padding, Pixels};
use cosmic::iced_core::text::Wrapping;
use cosmic::widget::{row, column, Column};
use cosmic::prelude::*;
use cosmic::widget::{self, text_input, ListColumn, Widget};
use cosmic::widget::icon::Handle;
use crate::services::clipboard::ClipboardEntry;

pub fn view(app: &AppModel) -> Element<'_, Message> {
    app.core
        .applet
        .icon_button("edit-paste-symbolic")
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
                let mut inline_input = widget::inline_input("Name entry", &entry.title)
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
                let mut inline_input = widget::inline_input("Name entry", &entry.title)
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

    let (pin_button_label, pin_button_icon) = if item.pinned {
        (fl!("unpin-item"), widget::icon::from_name("pin-symbolic").handle())
    } else {
        (fl!("pin-item"), widget::icon::from_name("preferences-default-applications-symbolic").handle())
    };

    let pin_button = widget::button::icon(pin_button_icon)
        .tooltip(pin_button_label)
        .on_press(Message::TogglePinEntry(id.clone()))
        .extra_small()
        .width(Length::Shrink);

    let edit_button = widget::button::icon(widget::icon::from_name("edit-symbolic").handle())
        .tooltip(fl!("edit-title"))
        .on_press(Message::EditToggled(id.clone()))
        .extra_small()
        .width(Length::Shrink);

    let save_button = widget::button::icon(widget::icon::from_name("document-save-symbolic").handle())
        .tooltip(fl!("save-title"))
        .on_press(Message::EditToggled(item.widget_id.clone()))
        .extra_small()
        .width(Length::Shrink);

    let remove_button =
        widget::button::icon(widget::icon::from_name("list-remove-symbolic").handle())
            .tooltip(fl!("remove"))
            .on_press(Message::RemoveHistory(item.widget_id.clone()))
            .extra_small()
            .width(Length::Shrink);

    widget::row::Row::new()
        .spacing(8)
        .padding([4, 0])
        .align_y(Alignment::Center)
        .push(label)
        .push(pin_button)
        .push(if app.editing_entry == Some(item.widget_id.clone()) { save_button } else { edit_button })
        .push(remove_button)
        .into()
}

pub fn view_window(app: &AppModel, _id: Id) -> Element<'_, Message> {
    let search_box = text_input::search_input(fl!("search-entries"), &app.search_filter)
        .always_active()
        .on_input(Message::SearchInputChanged)
        .on_paste(Message::SearchInputChanged)
        .on_clear(Message::SearchInputChanged("".to_string()));

    let settings = widget::button::icon(widget::icon::from_name("emblem-system-symbolic").handle())
        .tooltip("Modify Settings")
        .on_press(Message::ClearHistory); // TODO: Add message for settings gear

    let clear_all = widget::button::icon(widget::icon::from_name("edit-delete-symbolic").handle())
        .tooltip("Clear Clipboard History")
        .on_press(Message::ClearHistory);

    let top_row = widget::row().padding([2,0]).spacing(8)
        .push(settings)
        .push(search_box.width(Length::Fill))
        .push(clear_all); // row![search_box, settings_gear].padding([8, 0]).spacing(8); // widget::row().padding([8, 0]).spacing(8);

    let mut pinned_rows: Column<Message> = widget::column().into();
    for (id, item) in &app.pinned_clipboard_entries {
        if app.search_filter.is_empty() || item.title.contains(app.search_filter.as_str()) {
            pinned_rows = pinned_rows.push(create_clipboard_row(&app, &id, &item));
        }
    }

    let mut unpinned_rows: Column<Message> = widget::column().into();
    for (id, item) in &app.clipboard_entries {
        if app.search_filter.is_empty() || item.title.contains(app.search_filter.as_str()) {
            unpinned_rows = unpinned_rows.push(create_clipboard_row(&app, &id, &item));
        }
    }

    let total_clipboard_items = app.clipboard_entries.len() + app.pinned_clipboard_entries.len();
    let empty_label = (total_clipboard_items == 0).then_some(
        widget::container(
            widget::text::text(fl!("empty")))
                .center(Length::Fill)
                .height(Length::Shrink)
                .padding(Padding::from(8)
        )
    );

    let mut display = widget::column().padding(Padding::from(8)).spacing(0)
        .push(top_row)
        .push_maybe(empty_label)
        .push(pinned_rows)
        .push(unpinned_rows)
        .apply(widget::scrollable)
        // .width(Length::Fixed(800f32))
        .height(if total_clipboard_items > 5 { Length::Fixed(400.0) } else { Length::Shrink });

    app.core.applet.popup_container(display)
        .min_width(700f32)
        .max_width(800f32)
        .into()
}