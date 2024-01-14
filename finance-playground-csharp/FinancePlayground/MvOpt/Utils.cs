namespace Jason5Lee.FinancePlayground.MvOpt;

using MathNet.Numerics.LinearAlgebra;

internal class Utils {
    public static double CalculateExpectedReturn(Vector<double> weights, Vector<double> returns, double riskFreeRate) {
        return weights.DotProduct(returns) + riskFreeRate;
    }

    public static double CalculateVariance(Vector<double> weights, Matrix<double> covariance) {
        return weights.DotProduct(covariance.Multiply(weights));
    }

    public static double CalculateSharpeRatio(double expectedReturn, double variance, double riskFreeRate) {
        return (expectedReturn - riskFreeRate) / Math.Sqrt(variance);
    }
}
