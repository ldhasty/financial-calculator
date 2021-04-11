// use calculators::*;
fn main() {

    println!("{}", format!("{}{}", "compound interest: ",  calculators::compound_interest(5000.50f32,7.82f32,12,1)));
    println!("{}", format!("{}{}", "compound interest with contributions end of the period: ",  calculators::compound_interest_with_contributions_eop(5000f32,5f32,12,10,100f32)));
    println!("{}", format!("{}{}", "effective interest rate: ",  calculators::effective_interest(12,284.00+1.60,284.00,1.00/12.00)));



}