use trinarycryptor::{tables::OLD_TABLE, decode_text};
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
		EncryptionType::New => todo!(),
	};

	let result = match args.mode {
		OperatingMode::Decode => decode_text(&args.input, encryption_table),
		OperatingMode::Encode => todo!(),
	};

	println!("{}", result);
}
