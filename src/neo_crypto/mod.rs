pub use base58_helper::*;
pub use error::*;
pub use hash::*;
pub use key_pair::*;
pub use keys::*;
pub use utils::*;
pub use wif::*;

mod base58_helper;
mod error;
mod hash;
mod key_pair;
mod keys;
mod utils;
mod wif;

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
