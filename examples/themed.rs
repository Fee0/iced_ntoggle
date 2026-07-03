use std::num::NonZeroUsize;

use iced::widget::{button, column, container, row, slider, text};
use iced::{Alignment, Element, Fill, Font, Length, Result, Theme};
use iced_ntoggle::{Ntoggle, Selection, Style};

pub fn main() -> Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .theme(Demo::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Text(Selection),
    Glyph(Selection),
    Custom(Selection),
    ToggleTheme,
    PaddingChanged(u16),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    CornerRadiusChanged(f32),
}

struct Demo {
    text_selection: Selection,
    glyph_selection: Selection,
    custom_selection: Selection,
    dark_mode: bool,
    padding: u16,
    spacing: u16,
    text_size: u16,
    corner_radius: f32,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            text_selection: Selection::multiple_limited([0], NonZeroUsize::new(2).unwrap()),
            glyph_selection: Selection::single(1),
            custom_selection: Selection::multiple([0, 2]),
            dark_mode: true,
            padding: 8,
            spacing: 0,
            text_size: 16,
            corner_radius: 6.0,
        }
    }
}

impl Demo {
    fn theme(&self) -> Theme {
        if self.dark_mode { Theme::Dark } else { Theme::Light }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Text(selection) => self.text_selection = selection,
            Message::Glyph(selection) => self.glyph_selection = selection,
            Message::Custom(selection) => self.custom_selection = selection,
            Message::ToggleTheme => self.dark_mode = !self.dark_mode,
            Message::PaddingChanged(padding) => self.padding = padding,
            Message::SpacingChanged(spacing) => self.spacing = spacing,
            Message::TextSizeChanged(text_size) => self.text_size = text_size,
            Message::CornerRadiusChanged(corner_radius) => self.corner_radius = corner_radius,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let style = Style {
            corner_radius: self.corner_radius,
            ..Style::from_theme(&self.theme())
        };

        let text_toggle = Ntoggle::text(
            ["Draft", "Review", "Published"],
            self.text_selection.clone(),
            Message::Text,
        )
        .unwrap()
        .style(style)
        .padding(self.padding)
        .spacing(self.spacing)
        .text_size(f32::from(self.text_size));

        let glyph_toggle = Ntoggle::glyphs(
            ["◐", "●", "◯"],
            Font::MONOSPACE,
            self.glyph_selection.clone(),
            Message::Glyph,
        )
        .unwrap()
        .style(style)
        .padding(self.padding)
        .spacing(self.spacing);

        let custom_items = [
            swatch_item("Low", iced::Color::from_rgb8(0x22, 0xc5, 0x5e)),
            swatch_item("Medium", iced::Color::from_rgb8(0xf5, 0x9e, 0x0b)),
            swatch_item("High", iced::Color::from_rgb8(0xef, 0x44, 0x44)),
        ];

        let custom_toggle =
            Ntoggle::elements(custom_items, self.custom_selection.clone(), Message::Custom)
                .unwrap()
                .style(style)
                .padding(self.padding)
                .spacing(self.spacing);

        let controls = column![
            button(text(if self.dark_mode {
                "Switch to light theme"
            } else {
                "Switch to dark theme"
            }))
            .on_press(Message::ToggleTheme),
            labeled_slider(
                format!("Padding: {}", self.padding),
                slider(0..=24, self.padding, Message::PaddingChanged),
            ),
            labeled_slider(
                format!("Spacing: {}", self.spacing),
                slider(0..=16, self.spacing, Message::SpacingChanged),
            ),
            labeled_slider(
                format!("Text size: {}", self.text_size),
                slider(10..=28, self.text_size, Message::TextSizeChanged),
            ),
            labeled_slider(
                format!("Corner radius: {:.0}", self.corner_radius),
                slider(0.0..=20.0, self.corner_radius, Message::CornerRadiusChanged),
            ),
        ]
        .spacing(10);

        container(
            column![
                controls,
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

fn labeled_slider<'a>(
    label: String,
    slider: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![text(label).width(Length::Fixed(140.0)), slider.into()]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
}

fn swatch_item(label: &'static str, color: iced::Color) -> Element<'static, Message> {
    row![
        container("")
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .style(move |_theme| container::Style {
                background: Some(color.into()),
                border: iced::Border::default().rounded(6.0),
                ..container::Style::default()
            }),
        text(label),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}
