#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
	Equal,
	Sublist,
	Superlist,
	Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {

	if first_list.len() == second_list.len() {
		if first_list == second_list {
			Comparison::Equal
		} else {
			Comparison::Unequal
		}
	} else if check_window_fits(first_list, second_list) {
		Comparison::Sublist
	} else if check_window_fits(second_list, first_list) {
		Comparison::Superlist
	} else {
		Comparison::Unequal
	}
}

fn check_window_fits(window: &[i32], frame: &[i32]) -> bool {

	if window.is_empty() {
		return true;
	}

	let frame_windows = frame.windows(window.len());

	for frame_window in frame_windows {
		if window == frame_window {
			return true;
		}
	}

	false
}