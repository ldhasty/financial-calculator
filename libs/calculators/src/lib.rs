pub fn compound_interest(investment_amount: f32, interest_rate: f32, compound_frequency: i32, total_time: i32) -> f32 {
    // P (1 + r/n) ^(nt)
    investment_amount * (1f32 + ((interest_rate / 100f32) / compound_frequency as f32)).powi(compound_frequency * total_time)
}

pub fn compound_interest_with_contributions_eop(investment_amount: f32, interest_rate: f32, compound_frequency: i32, total_time: i32, contribution: f32) -> f32 {

    // Total = [ Compound interest for principal ] + [ Future value of a series ]
    // Total = [ P(1+r/n)^(nt) ] + [ PMT × (((1 + r/n)^(nt) - 1) / (r/n)) ]
    // Total = [ 5000 (1 + 0.05 / 12) ^ (12 × 10) ] + [ 100 × (((1 + 0.00416)^(12 × 10) - 1) / (0.00416)) ]
    // Total = [ 5000 (1.00416) ^ (120) ] + [ 100 × (1.00416)^(120) - 1) / 0.00416) ]
    // Total = [ 8235.05 ] + [ 100 × (0.647009497690848 / 0.00416) ]
    // Total = [ 8235.05 ] + [ 15528.23 ]
    // Total = [ $23,763.28 ]

    let decimal_interest_rate = interest_rate / 100f32;

    compound_interest(investment_amount, interest_rate, compound_frequency, total_time) +
        (contribution * (((1f32 + decimal_interest_rate / compound_frequency as f32).powi(compound_frequency * total_time)) - 1f32) / (decimal_interest_rate / compound_frequency as f32))
}

pub fn effective_interest(compound_frequency: i32, current_value: f32, amount_invested: f32, total_time: f32) -> f32 {
    // Calculate rate of interest in decimal, solve for r
    //
    // r = n[(A/P)^1/nt - 1]
    //r=n*((A/P)^(1/(n*t)) -1)
    compound_frequency as f32 * ((current_value / amount_invested).powf(1f32 / (compound_frequency as f32 * total_time)) - 1f32) * 100f32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
