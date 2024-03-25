mod args;
mod as_aob;
mod scan;

use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;
use clap::Parser;

use args::{Args, Endianness, ValueType};

use crate::scan::scan_value_str;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        args::Command::Scan {
            in_file,
            out_file,
            compare_file,
            value,
            value_type,
            endianness,
            dump,
        } => command_scan(
            &in_file,
            &out_file,
            compare_file.as_deref(),
            &value,
            value_type,
            endianness,
            dump,
        ),
        args::Command::Dump { file } => command_dump(&file),
    }
}

fn command_dump(file: &str) -> Result<(), anyhow::Error> {
    let results = read_output_file(file)?;
    for res in &results {
        println!("{res:#X}");
    }
    Ok(())
}

fn command_scan(
    in_file: &str,
    out_file: &str,
    compare_file: Option<&str>,
    value: &str,
    value_type: ValueType,
    endianness: Endianness,
    dump: bool,
) -> Result<(), anyhow::Error> {
    // open files
    let mut in_file = File::open(in_file).context("unable to open input file")?;
    let mut out_file = File::create(out_file).context("unable to create output file")?;
    let mut data = Vec::new();
    in_file.read_to_end(&mut data)?;

    // scan input file
    let mut results = scan_value_str(&data, value, value_type, endianness)?;
    println!("{} results found", results.len());

    // compare results to saved compare file
    if let Some(compare) = compare_file {
        let compare_results = read_output_file(compare)?;
        let compare_results = compare_results.into_iter().collect::<HashSet<_>>();
        results = results
            .iter()
            .filter(|res| compare_results.contains(res))
            .copied()
            .collect();
        println!("{} results found after filter", results.len());
    }

    // write results
    let output = results
        .iter()
        .flat_map(|res| res.to_le_bytes())
        .collect::<Vec<_>>();
    out_file.write_all(&output)?;

    if dump {
        println!();
        for res in &results {
            println!("{res:#X}");
        }
        println!();
    }

    println!("{} results saved", results.len());

    Ok(())
}

fn read_output_file(file: &str) -> Result<Vec<u64>, anyhow::Error> {
    let mut file = File::open(file).context("unable to open output file")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let results = data
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    Ok(results)
}
