use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Scan {
        /// the file to search in
        in_file: String,
        /// the file to output results in
        out_file: String,
        /// the file containing results to compare to
        compare_file: Option<String>,
        /// the value to search for
        #[arg(short, long, allow_hyphen_values = true)]
        value: String,
        #[arg(short = 't', long)]
        value_type: ValueType,
        #[arg(short, long, default_value = "little")]
        endianness: Endianness,
        /// print results
        #[arg(short, long)]
        dump: bool,
    },
    Dump {
        file: String,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ValueType {
    I8,
    I16,
    I32,
    I64,
    // Float,
    // Double,
    AOB,
    // CStr,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Endianness {
    Little,
    Big,
}
