use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    /// the file to search in
    pub filename: String,
    #[arg(short = 't', long)]
    pub value_type: ValueType,
    /// the value to search for
    #[arg(short, long, allow_hyphen_values = true)]
    pub value: String,
    #[arg(short, long, default_value = "little")]
    pub endianness: Endianness,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ValueType {
    I8,
    I16,
    I32,
    I64,
    Float,
    Double,
    AOB,
    CStr,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Endianness {
    Little,
    Big,
}
