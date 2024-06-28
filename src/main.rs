use std::ops::{Div, Mul};

use bounding_curve::curve::{calculator::CurveCalculator, constant_curve::ConstantCurve};

fn main(){
    let total_supply = 1_000_000_000.mul(10_u64.pow(6)); 
    let maximum_market_cap = 50.mul(10_u64.pow(9));

    let curve = ConstantCurve::new(total_supply.div(100), maximum_market_cap);
    println!("{:?}", curve.calculate_initial_price().unwrap::<f64>());
}