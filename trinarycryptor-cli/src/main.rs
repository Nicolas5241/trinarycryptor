use trinarycryptor::{tables::OLD_TABLE, decode_text};
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Default)]
enum EncryptionType {
	#[default]
	Old,
	New,
}

#[derive(Parser)]
#[command(version, about = "Trinary code decoder and encoder")]
struct Args {
	input: String,

	#[arg(short, long, default_value_t, value_enum)]
	r#type: EncryptionType,
}

fn main() {
	let args = Args::parse();

	let encryption_table = match args.r#type {
	    EncryptionType::Old => &OLD_TABLE,
		EncryptionType::New => todo!(),
	};

	let decoded_string = decode_text(&args.input, encryption_table);
	println!("{}", decoded_string)
}
