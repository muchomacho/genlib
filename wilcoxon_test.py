import sys
import numpy as np
import pandas as pd
from scipy.stats import ranksums

# データ読み込み
if len(sys.argv) != 2:
    print("Invalid arguments")
    sys.exit(-1)
data = pd.read_csv(sys.argv[1])
data.iloc[:, 0] = pd.to_numeric(data.iloc[:, 0], errors="coerce")
data.iloc[:, 1] = pd.to_numeric(data.iloc[:, 1], errors="coerce")

# 各グループの値をまとめる
a_group = data.iloc[:, 0].values.astype(float)
a_group = a_group[np.isfinite(a_group)]
b_group = data.iloc[:, 1].values.astype(float)
b_group = b_group[np.isfinite(b_group)]

# ウィルコクソン順位和検定
result = ranksums(a_group, b_group)

# 結果表示
print("statistic = {}".format(result.statistic))
print("p-value = {}".format(result.pvalue))
