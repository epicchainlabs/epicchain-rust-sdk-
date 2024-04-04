pub use binary_decoder::*;
pub use binary_encoder::*;
pub use encode::*;
pub use error::*;

mod binary_decoder;
mod binary_encoder;
mod encode;
mod error;

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
