import pandas as pd
import numpy as np


df = pd.DataFrame(columns=["type", "client", "tx", "amount"])

type = np.random.choice(["deposit", "withdrawl", "dispute"], size=10000, p=[0.55, 0.40, 0.05])
df["type"] = type

print(df.head())
