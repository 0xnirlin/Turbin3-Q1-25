use anchor_lang::prelude::*;

#[error_code]
pub enum AmlError {
    #[msg("Invalid LP amount provided")]
    InvalidLpAmount,
    #[msg("Pool is currently locked")]
    PoolLocked,
    #[msg("Max A")]
    MaxA,
    #[msg("Max B")]
    MaxB,
}
