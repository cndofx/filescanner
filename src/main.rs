mod args;
mod as_aob;

use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;
use clap::Parser;

use args::{Args, Endianness, ValueType};
use as_aob::AsAOB;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // open files
    let mut in_file = File::open(&args.in_file).context("unable to open input file")?;
    let mut out_file = File::create(&args.out_file).context("unable to create output file")?;
    let mut data = Vec::new();
    in_file.read_to_end(&mut data)?;

    // scan input file
    let mut results = scan_value_str(&data, &args.value, args.value_type, args.endianness)?;
    println!("{} results found", results.len());

    // compare results to saved compare file
    if let Some(compare) = &args.compare_file {
        let mut compare_file = File::open(compare).context("unable to open compare file")?;
        let mut compare_data = Vec::new();
        compare_file.read_to_end(&mut compare_data)?;
        let compare_results = compare_data
            .chunks_exact(8)
            .map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();
        results = results
            .iter()
            .filter(|res| compare_results.contains(res))
            .copied()
            .collect();
        println!("{} results found after filter", results.len());
    }

    // write results
    for addr in &results {
        println!("addr: {addr:#X}");
        out_file.write_all(&addr.to_le_bytes())?;
    }
    println!("{} results", results.len());

    Ok(())
}

fn scan_value_str(
    data: &[u8],
    value: &str,
    vtype: ValueType,
    endianness: Endianness,
) -> Result<Vec<u64>, anyhow::Error> {
    match vtype {
        ValueType::I8 => Ok(scan_value(data, value.parse::<i16>()? as u8, endianness)),
        ValueType::I16 => Ok(scan_value(data, value.parse::<i32>()? as u16, endianness)),
        ValueType::I32 => Ok(scan_value(data, value.parse::<i64>()? as u32, endianness)),
        ValueType::I64 => Ok(scan_value(data, value.parse::<i128>()? as u64, endianness)),
        _ => todo!(),
    }
}

fn scan_value<V: AsAOB>(data: &[u8], value: V, endianness: Endianness) -> Vec<u64> {
    let aob = match endianness {
        Endianness::Little => value.as_aob_le(),
        Endianness::Big => value.as_aob_be(),
    };

    scan_aob(data, &aob)
}

fn scan_aob(data: &[u8], aob: &[u8]) -> Vec<u64> {
    let mut results = Vec::new();

    let mut cursor = 0;
    loop {
        if cursor + aob.len() >= data.len() {
            println!("end reached!");
            break;
        }

        let slice = &data[cursor..cursor + aob.len()];
        if slice == aob {
            results.push(cursor as u64);
        }

        cursor += 1;
    }

    results
}
