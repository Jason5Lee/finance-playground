use rand::prelude::*;
use rand::rngs::SmallRng;
use statrs::distribution::{ContinuousCDF, Normal};

pub enum OptionType {
    Call,
    Put,
}

/// Financial Option, not to be confused with Rust's Option type
pub struct FinOption {
    pub underlying: f64,
    pub strike: f64,
    pub rate: f64,
    pub volatility: f64,
    pub time: f64,
    pub option_type: OptionType,
}

impl FinOption {
    pub fn price_european_black_scholes(&self) -> f64 {
        let d1 = (self.underlying.ln() - self.strike.ln()
            + (self.rate + 0.5 * self.volatility.powi(2)) * self.time)
            / (self.volatility * self.time.sqrt());
        let d2 = d1 - self.volatility * self.time.sqrt();

        let nd1 = Normal::new(0.0, 1.0).unwrap().cdf(d1);
        let nd2 = Normal::new(0.0, 1.0).unwrap().cdf(d2);
        match self.option_type {
            OptionType::Call => {
                self.underlying * nd1 - self.strike * (-self.rate * self.time).exp() * nd2
            }
            OptionType::Put => {
                self.strike * (-self.rate * self.time).exp() * (1.0 - nd2)
                    - self.underlying * (1.0 - nd1)
            }
        }
    }

    pub fn price_european_monte_carlo(&self, iterations: usize) -> f64 {
        let mut rng = SmallRng::from_entropy();
        let mut sum = 0.0;
        for _ in 0..iterations {
            let z = Normal::new(0.0, 1.0).unwrap().sample(&mut rng);
            let stock_price = self.underlying
                * ((self.rate - self.volatility.powi(2) / 2.0) * self.time
                    + self.volatility * z * self.time.sqrt())
                .exp();

            let payoff = match self.option_type {
                OptionType::Call => (stock_price - self.strike).max(0.0),
                OptionType::Put => (self.strike - stock_price).max(0.0),
            };
            sum += payoff;
        }
        let mean = sum / iterations as f64;
        mean * (-self.rate * self.time).exp()
    }
}
