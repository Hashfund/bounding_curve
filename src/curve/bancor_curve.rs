use std::ops::{Div, Mul};

use crate::safe_number::{Math, SafeNumber};

use super::calculator::{CurveCalculator, TradeDirection};

pub struct BancorCurve {
    reserve_ratio: f64,
    token_a_current_supply: f64,
    token_b_reserve_balance: f64,
}

impl BancorCurve {
    pub fn new(
        token_a_current_supply: u64,
        token_b_reserve_balance: u64,
        reserve_ratio: f64,
    ) -> Self {
        let token_a_current_supply = token_a_current_supply as f64;
        let token_b_reserve_balance = token_b_reserve_balance as f64;

        Self {
            token_a_current_supply,
            token_b_reserve_balance,
            reserve_ratio,
        }
    }
}

impl CurveCalculator for BancorCurve {
    fn calculate_initial_price(&self) -> SafeNumber {
        SafeNumber::new(
            self.token_b_reserve_balance
                .div(self.token_a_current_supply.mul(self.reserve_ratio)),
        )
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
mod bancor_curve_test {
    use std::ops::Mul;

    use crate::{
        curve::{bancor_curve::BancorCurve, calculator::CurveCalculator},
        safe_number::SafeNumber,
    };

    #[test]
    pub fn sell_mint_fraction_to_meet_maximum_cap() {
        let total_supply = 1_000_000_000.mul(10_u64.pow(6));
        let reserve_balance = 50.mul(10_u64.pow(9));
        let reserve_ratio = 0.9;

        let curve = BancorCurve::new(total_supply, reserve_balance, reserve_ratio);
        let initial_price = curve.calculate_initial_price();

        assert_eq!(
            initial_price,
            SafeNumber::new(0.00005555555555555556),
            "we are testing math accuracy"
        );

        // let sol_in = 1_000_000_000_000_000_000;
        // let token_out = curve.calculate_token_out(initial_price, sol_in, TradeDirection::BtoA);

        // assert!(token_out < total_supply, "We can't get more than the supply");
    }
}
