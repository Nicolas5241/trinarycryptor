use trinarycryptor::{decode_text, encode_text, tables::{NEW_TABLE, OLD_TABLE}};
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Default)]
enum EncryptionType {
	#[default]
	Old,
	New,
}

#[derive(ValueEnum, Clone)]
enum OperatingMode {
	Decode,
	Encode,
}

#[derive(Parser)]
#[command(version, about = "Trinary code decoder and encoder")]
struct Args {
	input: String,

	#[arg(short, long, default_value_t, value_enum)]
	r#type: EncryptionType,

	#[arg(short, long, value_enum)]
	mode: OperatingMode,
}

fn main() {
	let args = Args::parse();

	let encryption_table = match args.r#type {
	    EncryptionType::Old => &OLD_TABLE,
		EncryptionType::New => &NEW_TABLE,
	};

	let result = match args.mode {
		OperatingMode::Decode => decode_text(&args.input, encryption_table),
		OperatingMode::Encode => encode_text(&args.input, encryption_table),
	};

	println!("{}", result);
}
