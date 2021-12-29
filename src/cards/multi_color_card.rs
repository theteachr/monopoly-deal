use crate::color::{colored_text, Color, MultiColor};
use std::fmt;

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub struct MultiColorCard {
	text: &'static str,
	value: u8,
	available_colors: MultiColor,
	selected_color: Option<Color>,
}

impl MultiColorCard {
	pub fn new(text: &'static str, value: u8, colors: MultiColor) -> Self {
		Self {
			text,
			value,
			available_colors: colors,
			selected_color: None,
		}
	}

	pub fn set_color(&mut self, color: Color) {
		self.selected_color = Some(color);
	}

	pub fn colors(&self) -> Vec<Color> {
		self.available_colors.colors()
	}
}

impl fmt::Display for MultiColorCard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let color = match self.selected_color {
			Some(c) => c,
			None => Color::White,
		};

		write!(
			f,
			"{} {}",
			colored_text(self.text, color),
			self.available_colors,
		)
	}
}
