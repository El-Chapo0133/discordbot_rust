

pub struct Exponant;

impl Exponant {
	pub fn get(input: &f64, divisor: f64) -> f64 {
		return input.exp() / divisor;
	}
}


#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn assert_exponant() {
                assert_eq!(Exponant::get(&1.0), 0.1087312731383618);
                assert_eq!(Exponant::get(&2.0), 0.295562243957226);
                assert_eq!(Exponant::get(&3.0), 0.8034214769275068);
                assert_eq!(Exponant::get(&4.0), 2.1839260013257693);
                assert_eq!(Exponant::get(&5.0), 5.936526364103064);
                assert_eq!(Exponant::get(&6.0), 16.137151739709406);
        }
}