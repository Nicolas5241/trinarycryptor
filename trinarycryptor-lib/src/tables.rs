use phf::phf_map;
pub struct SubstitutionTable<K, V> where K: 'static, V: 'static {
	pub decoding: phf::Map<K, V>,
	pub encoding: phf::Map<V, K>,
}

macro_rules! phf_bimap {
    ({ $($k:expr => $v:expr),* $(,)? }) => {
        SubstitutionTable {
            decoding: phf_map! {
                $($k => $v),*
            },
            encoding: phf_map! {
                $($v => $k),*
            },
        }
    };
}

pub static OLD_TABLE: SubstitutionTable<&str, char> = phf_bimap!({
	"111" => 'a',
	"112" => 'b',
	"113" => 'c',
	"121" => 'd',
	"122" => 'e',
	"123" => 'f',
	"131" => 'g',
	"132" => 'h',
	"133" => 'i',
	"211" => 'j',
	"212" => 'k',
	"213" => 'l',
	"221" => 'm',
	"222" => 'n',
	"223" => 'o',
	"231" => 'p',
	"232" => 'q',
	"233" => 'r',
	"311" => 's',
	"312" => 't',
	"313" => 'u',
	"321" => 'v',
	"322" => 'w',
	"323" => 'x',
	"331" => 'y',
	"332" => 'z',

	"181" => '1',
	"182" => '2',
	"183" => '3',
	"281" => '4',
	"282" => '5',
	"283" => '6',
	"381" => '7',
	"382" => '8',
	"383" => '9',
	"118" => '0',
});
