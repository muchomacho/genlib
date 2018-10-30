import os
import re
import sys
import numpy as np
import pandas as pd
from scipy.stats import norm

def prop_test(t1, n1, t2, n2, kind="two-sided"):
    p1 = float(t1 / n1)
    p2 = float(t2 / n2)
    pooled_p = float((t1 + t2) / (n1 + n2))

    if pooled_p == 0.0 or pooled_p == 1.0:
        return 0.0
    statistic = (p1 - p2) / np.sqrt(pooled_p * (1.0 - pooled_p) * (1.0 / n1 + 1.0 / n2))
    prop = norm.cdf(statistic)

    if kind == "two-sided":
        if prop > 0.5:
            return 1.0 - 2.0 * (1.0 - prop)
        else:
            return 1.0 - 2.0 * (1.0 - prop)
    elif kind == "greater":
        return 1.0 - prop
    else:
        print("Unknown option.")
        return np.nan

if __name__ == "__main__":
    if len(sys.argv) != 7:
        print("Invalid arguments.")
        print("Usage: python extract_diff_region.py <dir1> <dir2> <genome_reference> <resolution> <p-value> <output>")
        sys.exit(1)

    dir1 = sys.argv[1]
    dir2 = sys.argv[2]
    ref = sys.argv[3]
    if ref not in ("hg19", "hg38"):
        print("Unknown reference.")
        sys.exit(1)
    res = int(sys.argv[4])
    threshold = float(sys.argv[5])
    out = sys.argv[6]

    chrom_length = []
    with open("{}/data/{}_chrom_size.txt".format(os.path.dirname(sys.argv[0]), ref), "r") as f:
        chrom_length = [int(line.strip().split("\t")[1]) for line in f]
    chrom_names = ["chr" + str(i) for i in range(1, 23)] + ["chrX", "chrY"]

    total_test = 0
    total_reads1 = 0
    total_reads2 = 0
    counts1 = []
    counts2 = []
    for i in range(1, 25):
        with open("{}/{}.txt".format(dir1, chrom_names[i]), "r") as f:
            count = np.array([int(x.strip()) for x in f])
            total_reads1 += np.sum(count)
            counts1.append(count)
        with open("{}/{}.txt".format(dir2, chrom_names[i]), "r") as f:
            count = np.array([int(x.strip()) for x in f])
            total_reads2 += np.sum(count)
            counts2.append(count)
        assert len(counts1[i - 1]) == len(counts2[i - 1]), "the lengths of two array must be equal."
        total_test += len(counts1[i - 1])
    
    results = []
    for i in range(1, 25):
        for j in range(len(counts1[i])):
            pval = prop_test(counts1[i][j], total_reads1, counts2[i][j], total_reads2)
            adjusted_pval = p-val / total_test
            diff.append([chrom_names[i], j * resolution, min((j + 1) * resolution, chrom_length[i]), pval, adjusted_pval])
    results = pd.DataFrame(diff, columns=["chr", "start", "end", "pval", "adjusted_pval"])

    significant_results = results.loc[results.adjusted_pval < threshold, ["chr", "start", "end"]]
    significant_results.to_csv(out, index=False, header=False, sep="\t")

    results.to_csv(re.sub("\.\w+", "", out) + "_stats.csv", index=False)
