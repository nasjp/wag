use crate::result::Result;
use crate::stdin::StdinRawMode;

pub struct Editor {
	stdin: StdinRawMode,
}

impl Editor {
	pub fn new(stdin: StdinRawMode) -> Result<Editor> {
		Ok(Editor { stdin })
	}

	pub fn process_key_press(&mut self) -> Result<Option<String>> {
		match self.stdin.read_byte()? {
			Some(b) if b.is_ascii_control() => Ok(Some(format!("<CTRL>: {}\r\n", b))),
			Some(b) if b == b'q' => Ok(None),
			Some(b) => Ok(Some(format!("{}\r\n", b as char))),
			_ => Ok(None),
		}
	}
}
