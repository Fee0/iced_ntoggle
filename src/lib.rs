pub mod error;
pub mod style;

use std::num::NonZeroUsize;

pub use error::NtoggleError;
use iced::widget::{Button, Row, Text};
use iced::{Element, Font, Length};
pub use style::{SegmentPosition, Style};

const MIN_STATES: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    Single(usize),
    Multiple {
        selected: Vec<usize>,
        max_selected: Option<NonZeroUsize>,
    },
}

impl Selection {
    pub fn single(index: usize) -> Self {
        Self::Single(index)
    }

    pub fn multiple(selected: impl Into<Vec<usize>>) -> Self {
        Self::Multiple {
            selected: selected.into(),
            max_selected: None,
        }
    }

    pub fn multiple_limited(selected: impl Into<Vec<usize>>, max_selected: NonZeroUsize) -> Self {
        Self::Multiple {
            selected: selected.into(),
            max_selected: Some(max_selected),
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        match self {
            Self::Single(selected) => *selected == index,
            Self::Multiple { selected, .. } => selected.contains(&index),
        }
    }

    pub fn next_after_press(&self, index: usize) -> Self {
        match self {
            Self::Single(_) => Self::Single(index),
            Self::Multiple {
                selected,
                max_selected,
            } => {
                let mut next = selected.clone();

                if let Some(position) = next.iter().position(|selected| *selected == index) {
                    next.remove(position);
                } else {
                    next.push(index);
                }

                next.sort_unstable();
                next.dedup();

                Self::Multiple {
                    selected: next,
                    max_selected: *max_selected,
                }
            }
        }
    }

    pub fn can_add_more(&self) -> bool {
        match self {
            Self::Single(_) => true,
            Self::Multiple {
                selected,
                max_selected,
            } => max_selected.is_none_or(|max| selected.len() < max.get()),
        }
    }

    pub fn validate(&self, len: usize) -> Result<(), NtoggleError> {
        if len < MIN_STATES {
            return Err(NtoggleError::TooFewStates {
                min: MIN_STATES,
                actual: len,
            });
        }

        match self {
            Self::Single(index) => validate_index(*index, len),
            Self::Multiple {
                selected,
                max_selected,
            } => {
                if matches!(max_selected, Some(max) if max.get() == 0) {
                    return Err(NtoggleError::MaxSelectedZero);
                }

                let mut seen = Vec::with_capacity(selected.len());

                for index in selected {
                    validate_index(*index, len)?;

                    if seen.contains(index) {
                        return Err(NtoggleError::DuplicateSelection { index: *index });
                    }

                    seen.push(*index);
                }

                if let Some(max_selected) = max_selected {
                    let max = max_selected.get();

                    if selected.len() > max {
                        return Err(NtoggleError::TooManySelected {
                            max,
                            actual: selected.len(),
                        });
                    }
                }

                Ok(())
            }
        }
    }
}

pub struct Ntoggle<'a, Message> {
    items: Vec<Element<'a, Message>>,
    selection: Selection,
    on_change: Box<dyn Fn(Selection) -> Message + 'a>,
    style: Style,
    padding: u16,
    spacing: u16,
    width: Length,
    height: Length,
}

impl<'a, Message: Clone + 'a> Ntoggle<'a, Message> {
    pub fn text(
        labels: impl IntoIterator<Item = impl Into<String>>,
        selection: Selection,
        on_change: impl Fn(Selection) -> Message + 'a,
    ) -> Result<Self, NtoggleError> {
        let items: Vec<Element<'a, Message>> = labels
            .into_iter()
            .map(|label| Element::from(Text::new(label.into())))
            .collect();

        Self::elements(items, selection, on_change)
    }

    pub fn glyphs(
        glyphs: impl IntoIterator<Item = impl Into<String>>,
        font: Font,
        selection: Selection,
        on_change: impl Fn(Selection) -> Message + 'a,
    ) -> Result<Self, NtoggleError> {
        let items: Vec<Element<'a, Message>> = glyphs
            .into_iter()
            .map(|glyph| Element::from(Text::new(glyph.into()).font(font)))
            .collect();

        Self::elements(items, selection, on_change)
    }

    pub fn elements(
        items: impl IntoIterator<Item = Element<'a, Message>>,
        selection: Selection,
        on_change: impl Fn(Selection) -> Message + 'a,
    ) -> Result<Self, NtoggleError> {
        let items = items.into_iter().collect::<Vec<_>>();
        selection.validate(items.len())?;

        Ok(Self {
            items,
            selection,
            on_change: Box::new(on_change),
            style: Style::default(),
            padding: 8,
            spacing: 0,
            width: Length::Shrink,
            height: Length::Shrink,
        })
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn into_element(self) -> Element<'a, Message> {
        let Self {
            items,
            selection,
            on_change,
            style,
            padding,
            spacing,
            width,
            height,
        } = self;
        let len = items.len();

        // With no gap between segments, overlap them by the border width so the
        // shared edge renders as a single line instead of two borders stacked
        // side by side.
        let row_spacing = if spacing == 0 {
            -style.normal.border.width
        } else {
            f32::from(spacing)
        };

        let row = items.into_iter().enumerate().fold(
            Row::new()
                .spacing(row_spacing)
                .width(width)
                .height(height),
            |row, (index, item)| {
                let is_selected = selection.contains(index);
                let is_disabled = !is_selected && !selection.can_add_more();
                let next_selection = selection.next_after_press(index);
                let style_for_segment = style;
                let position = segment_position(index, len);

                let button = Button::new(item)
                    .padding(padding)
                    .style(move |_theme, status| {
                        style_for_segment
                            .segment(is_selected, status, position)
                            .into_button_style()
                    });

                if is_disabled {
                    row.push(button)
                } else {
                    row.push(button.on_press((on_change)(next_selection)))
                }
            },
        );

        row.into()
    }
}

fn segment_position(index: usize, len: usize) -> SegmentPosition {
    if len == 1 {
        SegmentPosition::Only
    } else if index == 0 {
        SegmentPosition::First
    } else if index + 1 == len {
        SegmentPosition::Last
    } else {
        SegmentPosition::Middle
    }
}

impl<'a, Message: Clone + 'a> From<Ntoggle<'a, Message>> for Element<'a, Message> {
    fn from(ntoggle: Ntoggle<'a, Message>) -> Self {
        ntoggle.into_element()
    }
}

fn validate_index(index: usize, len: usize) -> Result<(), NtoggleError> {
    if index >= len {
        Err(NtoggleError::SelectionOutOfBounds { index, len })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_minimum_number_of_states() {
        assert_eq!(
            Selection::single(0).validate(1),
            Err(NtoggleError::TooFewStates { min: 2, actual: 1 })
        );
    }

    #[test]
    fn validates_single_selection_bounds() {
        assert_eq!(
            Selection::single(2).validate(2),
            Err(NtoggleError::SelectionOutOfBounds { index: 2, len: 2 })
        );
    }

    #[test]
    fn validates_multi_selection_bounds() {
        assert_eq!(
            Selection::multiple([0, 3]).validate(3),
            Err(NtoggleError::SelectionOutOfBounds { index: 3, len: 3 })
        );
    }

    #[test]
    fn validates_duplicate_multi_selection() {
        assert_eq!(
            Selection::multiple([1, 1]).validate(3),
            Err(NtoggleError::DuplicateSelection { index: 1 })
        );
    }

    #[test]
    fn validates_limited_multi_selection() {
        assert_eq!(
            Selection::multiple_limited([0, 1], NonZeroUsize::new(1).unwrap()).validate(3),
            Err(NtoggleError::TooManySelected { max: 1, actual: 2 })
        );
    }

    #[test]
    fn single_selection_moves_to_pressed_index() {
        assert_eq!(
            Selection::single(0).next_after_press(2),
            Selection::single(2)
        );
    }

    #[test]
    fn multi_selection_adds_pressed_index() {
        assert_eq!(
            Selection::multiple([0]).next_after_press(2),
            Selection::multiple([0, 2])
        );
    }

    #[test]
    fn multi_selection_removes_pressed_index() {
        assert_eq!(
            Selection::multiple([0, 2]).next_after_press(0),
            Selection::multiple([2])
        );
    }

    #[test]
    fn limited_multi_selection_knows_when_it_is_full() {
        assert!(!Selection::multiple_limited([0, 2], NonZeroUsize::new(2).unwrap()).can_add_more());
    }

    #[test]
    fn resolves_segment_positions() {
        assert_eq!(segment_position(0, 1), SegmentPosition::Only);
        assert_eq!(segment_position(0, 3), SegmentPosition::First);
        assert_eq!(segment_position(1, 3), SegmentPosition::Middle);
        assert_eq!(segment_position(2, 3), SegmentPosition::Last);
    }
}
