pub trait ToFloat {
	fn to_float(&self) -> f64;
}

impl ToFloat for u32 {
	fn to_float(&self) -> f64 {
		*self as f64
	}
}

impl ToFloat for i32 {
	fn to_float(&self) -> f64 {
		*self as f64
	}
}

pub struct Rational<T: ToFloat> {
	pub numerator: T,
	pub denominator: T,
}

impl<T: ToFloat> ToFloat for Rational<T> {
	fn to_float(&self) -> f64 {
		self.numerator.to_float() / self.denominator.to_float()
	}
}

pub struct CurveLookupTable {
	table: Vec<u16>,
}

impl CurveLookupTable {
	pub fn from_sony_tone_table(values: [u16; 4]) -> CurveLookupTable {
		let mut sony_curve = [0, 0, 0, 0, 0, 4095];
		for i in 0..4 {
			sony_curve[i + 1] = values[i] >> 2 & 0xfff;
		}

		let mut table = vec![0_u16; (sony_curve[5] + 1).into()];
		for i in 0..5 {
			for j in (sony_curve[i] + 1)..=sony_curve[i + 1] {
				table[j as usize] = table[(j - 1) as usize] + (1 << i);
			}
		}

		CurveLookupTable { table }
	}

	pub fn get(&self, x: usize) -> u16 {
		self.table[x]
	}
}
