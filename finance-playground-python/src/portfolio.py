import pandas as pd

stocks_data = pd.read_csv('stocks-data.csv')

class Columns:
    stock = 'Stock'
    price = 'Price'
    market_cap = 'Market Capitalization'
    equal_weight = 'Equal-weighting Weight'
    value_weight = 'Value-weighting Weight'

num_stocks = len(stocks_data)
stocks_data[Columns.equal_weight] = 1 / num_stocks

# Calculate Value-weighting
total_market_cap = stocks_data[Columns.market_cap].sum()
stocks_data[Columns.value_weight] = stocks_data[Columns.market_cap] / total_market_cap

# Format the weights to 3 decimal places
for column in [Columns.equal_weight, Columns.value_weight]:
    stocks_data[column] = stocks_data[column].apply(lambda x: round(x, 3))

# Save the updated DataFrame to a new CSV file
stocks_data.to_csv('portfolio.csv', index=False)
