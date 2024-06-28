use crate::safe_number::SafeNumber;

pub trait CurveCalculator {
    fn calculate_initial_price(&self) -> SafeNumber;
    fn calculate_token_out(
        initial_price: SafeNumber,
        amount: u64,
        trade_direction: TradeDirection,
    ) -> u64;
}

pub enum TradeDirection {
    AtoB,
    BtoA,
}
