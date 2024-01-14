namespace Jason5Lee.FinancePlayground.MvOpt;

using MathNet.Numerics.LinearAlgebra;

public class Portfolio {
    public Assets Assets { get; }
    public Vector<double> Weights { get; }
    public double ExpectedReturn { get; }
    public double Variance { get; }
    public double SharpeRatio { get; }
    public double RiskFreeRate { get; }

    public Portfolio(Assets assets, Vector<double> weights, double riskFreeRate)
    {
        if (weights.Count != assets.Returns.Count)
            throw new ArgumentException("Weights and returns must have the same dimension.");
        
        Assets = assets;
        Weights = weights;
        ExpectedReturn = Utils.CalculateExpectedReturn(Weights, Assets.Returns, RiskFreeRate);
        Variance = Utils.CalculateVariance(Weights, Assets.Covariance);
        RiskFreeRate = riskFreeRate;
        SharpeRatio = Utils.CalculateSharpeRatio(ExpectedReturn, Variance, RiskFreeRate);
    }
}
