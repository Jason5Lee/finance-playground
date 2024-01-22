use finance_playground_rust::option_pricing::{FinOption, OptionType};

fn main() {
    let option = FinOption {
        underlying: 50.0,
        strike: 50.0,
        rate: 0.1,
        volatility: 0.3,
        time: 0.25,
        option_type: OptionType::Call,
    };
    println!("Black-Scholes: {}", option.price_european_black_scholes());
    println!(
        "Monte Carlo: {}",
        option.price_european_monte_carlo(10000000)
    );
}
