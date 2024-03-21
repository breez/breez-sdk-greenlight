pub(crate) mod boltzswap;
pub(crate) mod error;
pub(crate) mod reverseswap;

pub(crate) fn calculate_service_fee_sat(send_amount_sat: u64, fees_percentage: f64) -> u64 {
    ((send_amount_sat as f64) * fees_percentage / 100.0).ceil() as u64
}

#[cfg(test)]
mod tests {
    use crate::swap_out::calculate_service_fee_sat;

    #[test]
    fn test_calculate_service_fee_sat() {
        // Round values, so rounding up plays no role
        assert_eq!(250, calculate_service_fee_sat(50_000, 0.5));
        assert_eq!(300, calculate_service_fee_sat(50_000, 0.6));
        assert_eq!(750, calculate_service_fee_sat(100_000, 0.75));

        // Odd values, where rounding up kicks in
        assert_eq!(251, calculate_service_fee_sat(50_001, 0.5));
        assert_eq!(301, calculate_service_fee_sat(50_001, 0.6));
        assert_eq!(751, calculate_service_fee_sat(100_001, 0.75));
    }
}
