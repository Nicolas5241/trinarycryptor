mod tables;

pub enum EncodingTable {
	Old,
	New,
}

pub fn decode_text(text: &str, table: EncodingTable) -> String {
	let encoding_table = match table {
		EncodingTable::Old => &tables::OLD_TABLE,
		EncodingTable::New => &tables::OLD_TABLE,
	};

	todo!()
}
