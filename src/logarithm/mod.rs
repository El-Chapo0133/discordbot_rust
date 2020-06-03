

pub struct Logarithm;

impl Logarithm {
	pub fn get(input: &f64) -> f64 {
		return (input.log10() + 1.0) * input;
	}
}



#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn assert_logarithm() {
                assert_eq!(Logarithm::get(&1.0), 1.0);
                assert_eq!(Logarithm::get(&4.0), 1.6020599913279624);
        }
}