pub(crate) mod boltzswap;
pub(crate) mod error;
pub(crate) mod reverseswap;

pub(crate) fn calculate_service_fee_sat(send_amount_sat: u64, fees_percentage: f64) -> u64 {
    ((send_amount_sat as f64) * fees_percentage / 100.0) as u64
}
