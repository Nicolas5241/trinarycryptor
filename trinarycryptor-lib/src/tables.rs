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
	"41" => 'a',
	"42" => 'b',
	"43" => 'c',
	"121" => 'd',
	"15" => 'e',
	"123" => 'f',
	"131" => 'g',
	"132" => 'h',
	"16" => 'i',
	"24" => 'j',
	"212" => 'k',
	"213" => 'l',
	"51" => 'm',
	"52" => 'n',
	"53" => 'o',
	"231" => 'p',
	"232" => 'q',
	"26" => 'r',
	"34" => 's',
	"312" => 't',
	"313" => 'u',
	"321" => 'v',
	"35" => 'w',
	"323" => 'x',
	"61" => 'y',
	"62" => 'z',
});
