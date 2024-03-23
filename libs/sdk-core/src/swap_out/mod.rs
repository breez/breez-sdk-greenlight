pub(crate) mod boltzswap;
pub(crate) mod error;
pub(crate) mod reverseswap;

/// Calculate the service fee from the `invoice_amount_sat`.
///
/// The fee is a percentage of the invoice amount, rounded up.
pub(crate) fn get_service_fee_sat(invoice_amount_sat: u64, fees_percentage: f64) -> u64 {
    ((invoice_amount_sat as f64) * fees_percentage / 100.0).ceil() as u64
}

/// Calculate the `invoice_amount_sat` from `invoice_amount_and_service_fee`.
///
/// This calculates the initial amount going in reverse, e.g. from the resulting sum.
pub(crate) fn get_invoice_amount_sat(
    invoice_amount_and_service_fee: u64,
    fees_percentage: f64,
) -> u64 {
    (invoice_amount_and_service_fee as f64 * 100.0 / (100.0 + fees_percentage)) as u64
}

#[cfg(test)]
mod tests {
    use crate::swap_out::{get_invoice_amount_sat, get_service_fee_sat};

    #[test]
    fn test_get_service_fee_sat() {
        // Round values, so rounding up plays no role
        assert_eq!(250, get_service_fee_sat(50_000, 0.5));
        assert_eq!(300, get_service_fee_sat(50_000, 0.6));
        assert_eq!(750, get_service_fee_sat(100_000, 0.75));

        // Odd values, where rounding up kicks in
        assert_eq!(251, get_service_fee_sat(50_001, 0.5));
        assert_eq!(301, get_service_fee_sat(50_001, 0.6));
        assert_eq!(751, get_service_fee_sat(100_001, 0.75));
    }

    #[test]
    fn test_get_invoice_amount_sat() {
        // Round values, so rounding up plays no role
        test_fee_calculation_in_both_directions(50_000, 0.5);
        test_fee_calculation_in_both_directions(51_000, 0.5);
        test_fee_calculation_in_both_directions(52_000, 0.5);
        test_fee_calculation_in_both_directions(53_000, 0.5);
        test_fee_calculation_in_both_directions(54_000, 0.5);
        test_fee_calculation_in_both_directions(60_000, 0.6);
        test_fee_calculation_in_both_directions(100_000, 0.75);

        // Odd values, where the rounding up in the service fee calculation kicks in
        test_fee_calculation_in_both_directions(50_001, 0.5);
        test_fee_calculation_in_both_directions(50_999, 0.5);
        test_fee_calculation_in_both_directions(51_001, 0.5);
        test_fee_calculation_in_both_directions(52_001, 0.5);
        test_fee_calculation_in_both_directions(53_001, 0.5);
        test_fee_calculation_in_both_directions(54_001, 0.5);
        test_fee_calculation_in_both_directions(60_001, 0.6);
        test_fee_calculation_in_both_directions(75_001, 0.75);
    }

    fn test_fee_calculation_in_both_directions(invoice_amount_sat: u64, fees_percentage: f64) {
        let service_fee_sat = get_service_fee_sat(invoice_amount_sat, fees_percentage);
        assert_eq!(
            invoice_amount_sat,
            get_invoice_amount_sat(invoice_amount_sat + service_fee_sat, fees_percentage)
        );
    }
}
