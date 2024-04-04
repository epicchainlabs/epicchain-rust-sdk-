pub use config::*;
pub use constant::*;
pub use test_properties::*;

mod config;
mod constant;
mod test_properties;

pub(crate) fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
