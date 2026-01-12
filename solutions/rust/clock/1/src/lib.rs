use std::fmt::Display;

	#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl Clock {
	pub fn new(hours: i32, minutes: i32) -> Self {
		Self {
			hours,
			minutes
		}
		.normalize()
		.clone()
	}
// 10:-27
	pub fn add_minutes(&self, minutes: i32) -> Self {
		Self {
			hours: self.hours,
			minutes: self.minutes + minutes,
		}
			.normalize()
			.clone()
	}

	// normalize negative/60+ minutes to proper Clock
	fn normalize(&mut self) -> &Self {

		// first pass for checking minutes under/over flow
		if self.minutes < 0 {
			let min = self.minutes * -1;
			// count how many times it goves below 60 (hours)
			let hours_back = min / 60 + 1; // 0 if min: [0, 60], >0 if min: [61, +]

			self.hours -= hours_back;
			self.minutes = 60 - (min % 60); // normalize min to fall in range [0, 60]
		}
		// if minutes is more than 60
		if self.minutes >= 60 {
			// count how many times it goes above 60 (hours)
			let hours_forward = self.minutes / 60;

			self.hours += hours_forward;
			self.minutes %= 60;
		}


		// second pass for checking hours under/over flow
		// if hours is negative
		if self.hours < 0 {
			self.hours = 24 - ((self.hours * -1) % 24);
		}

		// if hours is more than 24
		if self.hours >= 24 {
			self.hours %= 24;
		};

		self
	}
}

impl Display for Clock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:02}:{:02}", self.hours, self.minutes)
	}
}