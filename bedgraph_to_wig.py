"""
    This script accepts a bedGraph format file and convert it into a wig file
    The region intervals of bedgraph file must be the same

    Usage: python bedgraph_to_wig.py [input bedgraph file] [interval] [output wig file]

"""

import sys

if len(sys.argv) != 4:
    print("Invalid argument number")
    sys.exit(-1)

f = open(sys.argv[1], "r")

track_option = None
browser_option = []
data = {}
current_chrom = None
for line in f:
    line = line.strip().split()
    if line[0] == "track":
        track_option = " ".join(line).replace("bedGraph", "wiggle_0")
    if line[0] == "browser":
        header.append(" ".join(line))
    elif "chr" in line[0]:
        if line[0] != current_chrom:
            data[line[0]] = [" ".join([str(int(line[1]) + 1), line[3]])]
            current_chrom = line[0]
        else:
            data[line[0]].append(" ".join([str(int(line[1]) + 1), line[3]]))
f.close()

f = open(sys.argv[3], "w")
f.write(track_option + "\n")
for option in browser_option:
    f.write(option + "\n")
for chrom, values in data.items():
    f.write("variableStep chrom={0} span={1}\n".format(chrom, sys.argv[2]))
    for v in values:
        f.write(v + "\n")
f.close()


