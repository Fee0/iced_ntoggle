use iced::theme::palette::Pair;
use iced::widget::button;
use iced::{Background, Border, Color, Shadow, Theme, border};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentPosition {
    Only,
    First,
    Middle,
    Last,
}

impl SegmentPosition {
    pub fn radius(self, corner_radius: f32) -> border::Radius {
        match self {
            Self::Only => border::Radius::new(corner_radius),
            Self::First => border::Radius::default().left(corner_radius),
            Self::Middle => border::Radius::default(),
            Self::Last => border::Radius::default().right(corner_radius),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub background: Option<Background>,
    pub text_color: Color,
    pub border: Border,
    pub shadow: Shadow,
    pub snap: bool,
}

impl Segment {
    pub fn into_button_style(self) -> button::Style {
        button::Style {
            background: self.background,
            text_color: self.text_color,
            border: self.border,
            shadow: self.shadow,
            snap: self.snap,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub corner_radius: f32,
    pub normal: Segment,
    pub hovered: Segment,
    pub pressed: Segment,
    pub selected: Segment,
    pub selected_hovered: Segment,
    pub disabled: Segment,
}

impl Style {
    pub fn segment(
        self,
        selected: bool,
        status: button::Status,
        position: SegmentPosition,
    ) -> Segment {
        let mut segment = match (selected, status) {
            (_, button::Status::Disabled) => self.disabled,
            (true, button::Status::Hovered) => self.selected_hovered,
            (true, button::Status::Pressed) => self.pressed,
            (true, _) => self.selected,
            (false, button::Status::Hovered) => self.hovered,
            (false, button::Status::Pressed) => self.pressed,
            (false, _) => self.normal,
        };

        segment.border.radius = position.radius(self.corner_radius);
        segment
    }

    /// Derives a [`Style`] from an [`iced::Theme`]'s extended palette, so the
    /// widget can match the app's light/dark/custom theme instead of the
    /// fixed light palette used by [`Style::default`].
    pub fn from_theme(theme: &Theme) -> Self {
        let palette = theme.extended_palette();

        let border = Border::default()
            .width(1.0)
            .color(palette.background.strong.color);

        let segment_from_pair = |pair: Pair| Segment {
            background: Some(Background::Color(pair.color)),
            text_color: pair.text,
            border,
            shadow: Shadow::default(),
            snap: true,
        };

        let mut disabled = segment_from_pair(palette.background.base);
        disabled.background = disabled.background.map(|bg| bg.scale_alpha(0.5));
        disabled.text_color = disabled.text_color.scale_alpha(0.5);

        Self {
            corner_radius: 6.0,
            normal: segment_from_pair(palette.background.base),
            hovered: segment_from_pair(palette.background.weak),
            pressed: segment_from_pair(palette.background.strong),
            selected: segment_from_pair(palette.primary.base),
            selected_hovered: segment_from_pair(palette.primary.strong),
            disabled,
        }
    }

    /// Returns the largest border width across all six [`Segment`] states.
    ///
    /// Used to size the negative row spacing that makes adjacent segments'
    /// shared edges overlap into a single line; using the maximum (instead
    /// of assuming `normal` represents every state) avoids a visible
    /// double-border seam even when e.g. `selected` has a thicker border
    /// than `normal`.
    pub(crate) fn max_border_width(&self) -> f32 {
        [
            self.normal.border.width,
            self.hovered.border.width,
            self.pressed.border.width,
            self.selected.border.width,
            self.selected_hovered.border.width,
            self.disabled.border.width,
        ]
        .into_iter()
        .fold(0.0_f32, f32::max)
    }
}

impl Default for Style {
    fn default() -> Self {
        let border = Border::default()
            .width(1.0)
            .color(Color::from_rgb8(0x94, 0xa3, 0xb8));

        Self {
            corner_radius: 6.0,
            normal: Segment {
                background: Some(Color::WHITE.into()),
                text_color: Color::from_rgb8(0x0f, 0x17, 0x2a),
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            hovered: Segment {
                background: Some(Color::from_rgb8(0xf1, 0xf5, 0xf9).into()),
                text_color: Color::from_rgb8(0x0f, 0x17, 0x2a),
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            pressed: Segment {
                background: Some(Color::from_rgb8(0xcb, 0xd5, 0xe1).into()),
                text_color: Color::from_rgb8(0x0f, 0x17, 0x2a),
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            selected: Segment {
                background: Some(Color::from_rgb8(0x25, 0x63, 0xeb).into()),
                text_color: Color::WHITE,
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            selected_hovered: Segment {
                background: Some(Color::from_rgb8(0x1d, 0x4e, 0xd8).into()),
                text_color: Color::WHITE,
                border,
                shadow: Shadow::default(),
                snap: true,
            },
            disabled: Segment {
                background: Some(Color::from_rgb8(0xe2, 0xe8, 0xf0).into()),
                text_color: Color::from_rgb8(0x94, 0xa3, 0xb8),
                border,
                shadow: Shadow::default(),
                snap: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_segment_rounds_left_corners() {
        assert_eq!(
            SegmentPosition::First.radius(6.0),
            border::Radius::default().left(6.0)
        );
    }

    #[test]
    fn middle_segment_has_square_corners() {
        assert_eq!(
            SegmentPosition::Middle.radius(6.0),
            border::Radius::default()
        );
    }

    #[test]
    fn last_segment_rounds_right_corners() {
        assert_eq!(
            SegmentPosition::Last.radius(6.0),
            border::Radius::default().right(6.0)
        );
    }

    #[test]
    fn max_border_width_picks_largest_across_segments() {
        let mut style = Style::default();
        style.selected.border.width = 3.0;

        assert_eq!(style.max_border_width(), 3.0);
    }
}
