use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {

	#[allow(unused)]
	let gigasecond: Duration = Duration::seconds(1_000_000_000);

	let end = start.checked_add(gigasecond);

	end.unwrap()
}
