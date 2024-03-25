use std::num::ParseIntError;

use anyhow::{bail, Context};

use crate::{
    args::{Endianness, ValueType},
    as_aob::AsAOB,
};

pub fn scan_value_str(
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
        ValueType::AOB => Ok(scan_aob(data, &parse_aob(value)?)),
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

fn parse_aob(s: &str) -> Result<Vec<u8>, anyhow::Error> {
    if s.len() % 2 != 0 {
        bail!("aob input length must be a multiple of 2");
    }

    let chars = s.chars().collect::<Vec<_>>();
    let bytes = chars
        .chunks_exact(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|str| u8::from_str_radix(&str, 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .context("unable to parse input as hex")?;

    Ok(bytes)
}
