# extract genes from input genomic regions and write those Entrez gene IDs into putput file
# -----------------
# Parameter
# input: String
# input file in which the list of the genomic regions are included
# output: String
# output file in which the list of gene ID will be written
# chr: String(ex: 'chr5')
# target chromosome name
# if this parameter is skipped, the whole input genomic regions are used for extraction
extract_genes <- function(input, output, chr=NA){
  library(Homo.sapiens)
  library(dplyr)
  coord <- read.table(input, header=FALSE, stringsAsFactors=FALSE)
  coord.gr <- apply(coord, 1, function (x) {unlist(strsplit(x, ':'))}) %>%
    t %>%
    as.data.frame %>%
    dplyr::select(chrom=V11, start=V12, end=V13) %>%
    dplyr::mutate(chrom=paste0('chr', chrom))
  if (!is.na(chr)) {
    coord.gr <- dplyr::filter(coord.gr, chrom == chr)
  }
  coord.gr <- coord.gr %>%
    dplyr::mutate(start=as.numeric(as.character(start))) %>%
    dplyr::mutate(end=as.numeric(as.character(end))) %>%
    makeGRangesFromDataFrame
  coord.genes <- subsetByOverlaps(genes(TxDb.Hsapiens.UCSC.hg19.knownGene), coord.gr)
  coord.genes <- coord.genes$gene_id
  write.table(coord.genes, file=output, quote = FALSE, row.names = FALSE, col.names = FALSE)
}

