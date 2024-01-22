import numpy as np
import pandas as pd
import requests
import xlsxwriter
import math

stocks = pd.read_csv('sp_500_stocks.csv')
my_columns = ['Ticker', 'Price','Market Capitalization', 'Number Of Shares to Buy']
final_dataframe = pd.DataFrame(columns = my_columns)
