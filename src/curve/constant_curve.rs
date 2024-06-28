use std::ops::Div;

use crate::safe_number::{Math, SafeNumber};

use super::calculator::{CurveCalculator, TradeDirection};

pub struct ConstantCurve {
    current_supply: f64,
    maximum_market_cap: f64,
}

impl ConstantCurve {
    pub fn new(current_supply: u64, maximum_market_cap: u64) -> Self {
        let current_supply = current_supply as f64;
        let maximum_market_cap = maximum_market_cap as f64;

        Self {
            current_supply,
            maximum_market_cap,
        }
    }
}

impl CurveCalculator for ConstantCurve {
    fn calculate_initial_price(&self) -> SafeNumber {
        SafeNumber::new(self.maximum_market_cap.div(self.current_supply))
    }

    fn calculate_token_out(
        initial_price: SafeNumber,
        amount: u64,
        trade_direction: super::calculator::TradeDirection,
    ) -> u64 {
        match trade_direction {
            TradeDirection::AtoB => initial_price.mul(amount.into()).unwrap(),
            TradeDirection::BtoA => initial_price.inverse_div(amount.into()).unwrap(),
        }
    }
}

#[cfg(test)]
mod constant_curve_test {
    use std::ops::{Div, Mul};

    use crate::curve::calculator::{CurveCalculator, TradeDirection};

    use super::ConstantCurve;

    #[test]
    pub fn sell_mint_fraction_to_meet_maximum_cap() {
        let total_supply = 1_000_000_000.mul(10_u64.pow(6));
        let sell_fraction = total_supply.div(3);
        let maximum_market_cap = 50.mul(10_u64.pow(9));

        let curve = ConstantCurve::new(sell_fraction, maximum_market_cap);
        let initial_price = curve.calculate_initial_price();

        let token_out =
            ConstantCurve::calculate_token_out(initial_price, maximum_market_cap, TradeDirection::BtoA);
        assert_eq!(
            token_out, sell_fraction,
            "we are only selling this fraction to migrate to raydium"
        );

        let sol_out = ConstantCurve::calculate_token_out(initial_price, token_out, TradeDirection::AtoB);
        assert_eq!(
            sol_out, maximum_market_cap,
            "we reached the target marketcap"
        );
    }
}
