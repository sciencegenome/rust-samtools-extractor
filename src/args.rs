use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct GenomeArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
    /// please provide the id list for the specific region
    pub idlist_arg: String,
}
