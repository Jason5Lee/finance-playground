use rand::Rng;

/// Estimate the Standard Error of the Sharpe Ratio.
pub fn estimate_sharpe_ratio_se(returns: &[f64], num_samples: usize) -> f64 {
    let num_returns = returns.len();
    let mut rng = rand::thread_rng();
    let mut sharpe_ratios = Vec::with_capacity(num_samples);

    for _ in 0..num_samples {
        let mut sample_returns = Vec::with_capacity(num_returns);
        for _ in 0..num_returns {
            let random_index = rng.gen_range(0..num_returns);
            sample_returns.push(returns[random_index]);
        }
        let mean_return = sample_returns.iter().sum::<f64>() / num_returns as f64;
        let std_dev = (sample_returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / num_returns as f64)
            .sqrt();
        let sharpe_ratio = mean_return / std_dev;
        sharpe_ratios.push(sharpe_ratio);
    }

    let mean_sharpe_ratio = sharpe_ratios.iter().sum::<f64>() / num_samples as f64;
    let std_dev_sharpe_ratio = (sharpe_ratios
        .iter()
        .map(|r| (r - mean_sharpe_ratio).powi(2))
        .sum::<f64>()
        / num_samples as f64)
        .sqrt();
    std_dev_sharpe_ratio
}
