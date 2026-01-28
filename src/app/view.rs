use super::{AppModel, Message, icons};
use crate::fl;
use crate::services::clipboard;
use cosmic::applet::menu_button;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget;

pub fn view(app: &AppModel) -> Element<'_, Message> {
    app.core
        .applet
        .icon_button("edit-copy-symbolic")
        .on_press(Message::TogglePopup)
        .into()
}

pub fn view_window(app: &AppModel, _id: Id) -> Element<'_, Message> {
    let mut content = widget::list_column()
        .padding([8, 0])
        .spacing(0);

    if app.history.is_empty() {
        content = content.add(widget::text::body(fl!("empty")));
    } else {
        for (idx, item) in app.history.iter().enumerate() {
            let label: Element<'_, Message> = match item {
                clipboard::ClipboardEntry::Text(text) => {
                    widget::text::body(summarize_one_line(text)).into()
                }
                clipboard::ClipboardEntry::Image {
                    mime,
                    bytes,
                    thumbnail_png,
                    ..
                } => {
                    let thumb = thumbnail_png.as_ref().map(|png| {
                        widget::image(ImageHandle::from_bytes(png.clone()))
                            .width(Length::Fixed(40.0))
                            .height(Length::Fixed(40.0))
                    });

                    let meta = widget::text::body(format!(
                        "{} ({} KB)",
                        mime,
                        (bytes.len().saturating_add(1023)) / 1024
                    ));

                    let mut row = widget::row::Row::new().spacing(8);
                    if let Some(thumb) = thumb {
                        row = row.push(thumb);
                    }
                    row.push(meta).into()
                }
            };

            let copy_button = menu_button(label)
                .on_press(Message::CopyFromHistory(idx))
                .width(Length::Fill);
            let remove_button = widget::button::icon(icons::remove_icon())
                .tooltip(fl!("remove"))
                .on_press(Message::RemoveHistory(idx))
                .extra_small()
                .width(Length::Shrink);
            content = content.add(
                widget::row::Row::new()
                    .spacing(8)
                    .padding([4, 0])
                    .align_y(Alignment::Center)
                    .push(copy_button)
                    .push(remove_button)
                    .width(Length::Fill),
            );
        }
    }

    app.core.applet.popup_container(content).into()
}

fn summarize_one_line(text: &str) -> String {
    let mut line = text
        .lines()
        .map(|line| line.trim_start())
        .find(|line| !line.is_empty())
        .unwrap_or("")
        .trim_end()
        .to_string();
    const MAX_CHARS: usize = 60;
    if line.chars().count() > MAX_CHARS {
        line = line.chars().take(MAX_CHARS - 1).collect::<String>();
        line.push('…');
    }
    line
}
