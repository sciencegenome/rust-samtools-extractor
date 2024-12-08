mod args;
use args::GenomeArgs;
use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-12-3

 rust-samtools-extractor: This part of the samtoolsextracts the read alignment and
 also the corresponding portion from the fasta and also the upstream and the downstream
 regions from the fasta so that read aligned and the upstream and the downstream regions
 can be used.

 * */

fn main() {

    let args = GenomeArgs::parse();
    let result = samtools_extractor(&args.alignment_arg, &args.idlist_arg).unwrap();
    println!("The result for the specific IDs have been written {}", result);
}

fn samtools_extractor(pathsam: &str, pathlist:&str) -> Result<String, Box<dyn Error>> {

  #[derive(Debug, Clone, PartialEq, PartialOrd)]
  struct Limit {
    headerline: String,
    line: String,
  }

  #[derive(Debug, Clone, PartialEq, PartialOrd)]
  struct Fastaline {
    header: String,
    sequence: String,
  }


  let fileopen = File::open(pathsam).expect("file not present");
  let fileread = BufReader::new(fileopen);
  let mut limit_lines = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not found");
    if ! line.starts_with("@") {
      let iden = line;
      limit_lines.push(iden);
    }
  }
  let stringfile = File::open(pathlist).expect("file not present");
  let stringfileread = BufReader::new(stringfile);
  let mut stringsearch:Vec<String> = Vec::new();
  for i in stringfileread.lines(){
      let id = i.expect("line not found");
      stringsearch.push(id)
  }

  let mut limit:Vec<Limit> = Vec::new();
  for i in limit_lines.iter(){
    let mutable = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
    if mutable.is_empty() {
      continue
    } else {
      limit.push(Limit {
        headerline: mutable[2].to_string(),
        line: mutable[9].to_string()});
    }
  }

  let mut selected_string:Vec<Fastaline> = Vec::new();
  for i in limit.iter(){
    for j in stringsearch.iter(){
      if *j == i.headerline {
        selected_string.push(Fastaline{
          header: i.headerline.clone(),
          sequence: i.line.clone()
        })
      }
    }
  }

  let mut fasta_write = File::create("selected-ids.fasta").expect("file not present");
  for i in selected_string.iter_mut(){
    write!(fasta_write, ">{}\n{}\n", i.header, i.sequence).expect("not able to write th line");
  }
    Ok("The files have been written and the summary is given below".to_string())
}
