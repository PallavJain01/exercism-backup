#[derive(Debug)]
pub struct Duration {
	pub seconds: f64,
}

impl From<u64> for Duration {
	fn from(s: u64) -> Self {
		Duration { seconds: s as f64 }
	}
}

static EARTH_SECONDS_IN_YEAR: f64 = 31_557_600f64;

pub trait Planet {
	const ORBITAL_PERIOD: f64 = 1f64;
	fn years_during(d: &Duration) -> f64 {
		d.seconds / (EARTH_SECONDS_IN_YEAR * Self::ORBITAL_PERIOD)
	}
}

macro_rules! planets_with_orbital_period {
		($($planet:ident $period:expr),+) => {
			$(
				pub struct $planet;
				impl Planet for $planet {
					const ORBITAL_PERIOD: f64 = $period;
				}
			)+
		};
}


planets_with_orbital_period!(
	Mercury 0.2408467,
	Venus 0.61519726,
	Earth 1.0,
	Mars 1.8808158,
	Jupiter 11.862615,
	Saturn 29.447498,
	Uranus 84.016846,
	Neptune 164.79132
);