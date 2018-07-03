library(Homo.sapiens)
library(dplyr)
coord <- read.table('/Users/toshiakiuno/Desktop/research/abcomp/NALM6/NALM6_result/comp_open_closed.txt', 
                    header = FALSE, stringsAsFactors = FALSE)
coord.list <- coord[, 1]
coord.gr <- lapply(coord.list, function (x) {res=strsplit(x, ':')}) %>%
  unlist %>%
  as.numeric %>%
  matrix(ncol=3, byrow=T) %>%
  as.data.frame %>%
  select(chrom=V1, start=V2, end=V3) %>%
  mutate(chrom=paste0('chr', chrom)) %>%
  makeGRangesFromDataFrame
coord.genes <- subsetByOverlaps(genes(TxDb.Hsapiens.UCSC.hg19.knownGene), coord.gr)
coord.genes <- coord.genes$gene_id
write.table(coord.genes, file='/Users/toshiakiuno/Desktop/research/abcomp/NALM6/NALM6_result/geneID_open_closed.txt',
            quote = FALSE, row.names = FALSE, col.names = FALSE)
