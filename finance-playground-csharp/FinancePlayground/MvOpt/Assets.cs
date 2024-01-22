namespace Jason5Lee.FinancePlayground.MvOpt;

using MathNet.Numerics.LinearAlgebra;

public class Assets
{
    public Vector<double> Returns { get; }
    public Matrix<double> Covariance { get; }
    public double RiskFreeRate { get; }

    public Assets(Vector<double> returns, Matrix<double> covariance, double riskFreeRate)
    {
        if (returns.Count != covariance.RowCount || returns.Count != covariance.ColumnCount)
            throw new ArgumentException("Returns and covariance matrix must have the same dimension.");
        
        Returns = returns;
        Covariance = covariance;
        RiskFreeRate = riskFreeRate;
    }

    /// <summary>
    /// Returns the mean-variance optimal portfolio with given risk aversion parameter.
    /// </summary>
    /// <param name="riskAversion">The risk aversion</param>
    /// <returns>The portfolio</returns>
   public Portfolio MvOptimize(double riskAversion)
    {
        if (riskAversion <= 0)
            throw new ArgumentException("Gamma must be greater than 0.");

        var excessReturns = Returns.Subtract(RiskFreeRate);
        var invCovariance = Covariance.Inverse();

        // Calculate optimal weights
        var weights = invCovariance.Multiply(excessReturns).Divide(riskAversion);

        return new Portfolio(this, weights, RiskFreeRate);
    }
}
