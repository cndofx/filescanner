mod args;
mod as_aob;

use std::io::Read;

use clap::Parser;

use args::{Args, Endianness, ValueType};
use as_aob::AsAOB;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut data = Vec::new();
    let mut file = std::fs::File::open(&args.filename).unwrap();
    file.read_to_end(&mut data).unwrap();

    let results = scan_value_str(&data, &args.value, args.value_type, args.endianness)?;

    for addr in &results {
        println!("addr: {addr:#X}");
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
