pub use error::*;
pub use script::*;
pub use transaction::*;
pub use utils::*;

mod error;
mod script;
mod transaction;
mod utils;

pub fn add(left: usize, right: usize) -> usize {
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
