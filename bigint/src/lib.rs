#![cfg_attr(asm_available, feature(asm))]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

extern crate rlp;
extern crate hexutil;
#[cfg(feature = "std")]
extern crate rand;
#[cfg(feature = "std")]
extern crate libc;
extern crate byteorder;

#[cfg(feature="heapsizeof")]
#[macro_use] extern crate heapsize;

mod m256;
mod mi256;
mod uint;
mod hash;
mod bytes;
mod gas;

pub type Address = H160;

pub use self::bytes::B256;
pub use self::gas::Gas;
pub use self::uint::{U128, U256, U512};
pub use self::m256::M256;
pub use self::mi256::MI256;
pub use self::hash::{H32, H64, H128, H160, H256, H264, H512, H520, H1024, H2048};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
/// Sign of an integer.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}
