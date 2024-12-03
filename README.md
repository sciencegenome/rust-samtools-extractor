# rust-samtools-extractor

- rust samtools extractor.
- give the id list and extracts all the reads corresponding to those ids and writes a fasta. 

```
cargo build 

```

```
λ gauravsablok rust-samtools-extractor → λ git main* → ./target/debug/rust-samtools-extractor -h
Usage: rust-samtools-extractor <ALIGNMENT_ARG> <IDLIST_ARG>

Arguments:
  <ALIGNMENT_ARG>  please provide the path to the alignment file
  <IDLIST_ARG>     please provide the id list for the specific region

Options:
  -h, --help     Print help
  -V, --version  Print version

```
- how to run the binary

```
./target/debug/rust-samtools-extractor ./sample-files/alignreads-metagenomics.sam ./sample-files/sample-list.txt

```

Gaurav Sablok
