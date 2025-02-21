///! 128 bit numbers
///! U128 is more efficient that u128
///! https://github.com/solana-labs/solana/issues/19549
// To fix this error, add the uint dependency to Cargo.toml:
// [dependencies]
// uint = { version = "0.9.5", default-features = false }
use uint::construct_uint;
construct_uint! {
    pub struct U128(2);
}