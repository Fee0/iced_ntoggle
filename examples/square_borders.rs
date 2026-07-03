use std::num::NonZeroUsize;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Font, Length, Result};
use iced_ntoggle::{Ntoggle, Selection, Style};

pub fn main() -> Result {
    iced::application(Demo::default, Demo::update, Demo::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Text(Selection),
    Glyph(Selection),
    Custom(Selection),
}

struct Demo {
    text_selection: Selection,
    glyph_selection: Selection,
    custom_selection: Selection,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            text_selection: Selection::multiple_limited([0], NonZeroUsize::new(2).unwrap()),
            glyph_selection: Selection::single(1),
            custom_selection: Selection::multiple([0, 2]),
        }
    }
}

fn square_style() -> Style {
    Style {
        corner_radius: 0.0,
        ..Style::default()
    }
}

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::Text(selection) => self.text_selection = selection,
            Message::Glyph(selection) => self.glyph_selection = selection,
            Message::Custom(selection) => self.custom_selection = selection,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let text_toggle = Ntoggle::text(
            ["Draft", "Review", "Published"],
            self.text_selection.clone(),
            Message::Text,
        )
        .unwrap()
        .style(square_style());

        let glyph_toggle = Ntoggle::glyphs(
            ["◐", "●", "◯"],
            Font::MONOSPACE,
            self.glyph_selection.clone(),
            Message::Glyph,
        )
        .unwrap()
        .style(square_style())
        .padding(10);

        let custom_items = [
            swatch_item("Low", iced::Color::from_rgb8(0x22, 0xc5, 0x5e)),
            swatch_item("Medium", iced::Color::from_rgb8(0xf5, 0x9e, 0x0b)),
            swatch_item("High", iced::Color::from_rgb8(0xef, 0x44, 0x44)),
        ];

        let custom_toggle =
            Ntoggle::elements(custom_items, self.custom_selection.clone(), Message::Custom)
                .unwrap()
                .style(square_style())
                .spacing(4);

        container(
            column![
                text("Text states").size(18),
                text_toggle,
                text("Font glyph states").size(18),
                glyph_toggle,
                text("Custom element states").size(18),
                custom_toggle,
            ]
            .spacing(14),
        )
        .padding(24)
        .width(Fill)
        .height(Fill)
        .into()
    }
}

fn swatch_item(label: &'static str, color: iced::Color) -> Element<'static, Message> {
    row![
        container("")
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .style(move |_theme| container::Style {
                background: Some(color.into()),
                border: iced::Border::default(),
                ..container::Style::default()
            }),
        text(label),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}
