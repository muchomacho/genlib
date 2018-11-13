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
        return 1.0
    statistic = (p1 - p2) / np.sqrt(pooled_p * (1.0 - pooled_p) * (1.0 / n1 + 1.0 / n2))
    prop = norm.cdf(statistic)

    if kind == "two-sided":
        if prop > 0.5:
            return 2.0 * (1.0 - prop)
        else:
            return 2.0 * prop
    elif kind == "greater":
        return 1.0 - prop
    else:
        print("Unknown option.")
        return np.nan

if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("Invalid arguments.")
        print("Usage: python extract_diff_genes.py <gene_read_count1> <gene_read_count2> <p-value> <output>")
        sys.exit(1)

    gene_count_file1 = sys.argv[1]
    gene_count_file2 = sys.argv[2]
    threshold = float(sys.argv[3])
    out = sys.argv[4]
    if ".csv" not in out:
        out += ".csv"

    with open(gene_count_file1, 'r') as f:
        gene_counts1 = dict()
        for line in f:
            vals = line.strip().split()
            gene_counts1[vals[0]] = int(vals[1])
        genes1 = set(gene_counts1.keys())
        total_reads1 = sum(gene_counts1.values())
    with open(gene_count_file2, 'r') as f:
        gene_counts2 = dict()
        for line in f:
            vals = line.strip().split()
            gene_counts2[vals[0]] = int(vals[1])
        genes2 = set(gene_counts2.keys())
        total_reads2 = sum(gene_counts2.values())
    intersect_genes = genes1 & genes2
    total_test = len(intersect_genes)

    results = []
    for gene in intersect_genes:
        val1 = gene_counts1[gene]
        val2 = gene_counts2[gene]
        pval = prop_test(val1, total_reads1, val2, total_reads2)
        adjusted_pval = pval * total_test
        results.append([gene, "greater" if val1 < val2 else "less", pval, adjusted_pval])
    results = pd.DataFrame(results, columns=["gene", "type", "pval", "adjusted_pval"])

    significant_regions = results[results.adjusted_pval < threshold].sort_values(by=["adjusted_pval"])
    significant_regions.to_csv(out, index=False)

    results.to_csv(re.sub("\.\w+", "_stats.csv", out), index=False)
