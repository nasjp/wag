use crate::result::Result;
use std::io::Read;
use std::io::{self};
use std::os::unix::io::AsRawFd;
use termios::{
	tcsetattr, Termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
	TCSAFLUSH, VMIN, VTIME,
};

pub struct StdinRawMode {
	pub stdin: io::Stdin,
	original_terminal: termios::Termios,
}

impl StdinRawMode {
	pub fn enable() -> Result<StdinRawMode> {
		let stdin = io::stdin();
		let fd = stdin.as_raw_fd();
		let mut terminal = Termios::from_fd(fd)?;
		let original_terminal = terminal;

		tcsetattr(fd, TCSAFLUSH, &terminal)?;

		// Fix Ctrl-M
		// Disable Ctrl-S Ctrl-Q
		terminal.c_iflag &= !(ICRNL | IXON);
		// Disable Echo
		// Disable Canonical Mode
		// Disable Ctrl-C
		// Disable Ctrl-Z
		// Disalbe Ctrl-V
		terminal.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);

		// Disable post processing of output
		terminal.c_oflag &= !(OPOST);

		tcsetattr(fd, TCSAFLUSH, &terminal)?;
		Ok(StdinRawMode {
			stdin,
			original_terminal,
		})
	}

	pub fn read_byte(&mut self) -> Result<Option<u8>> {
		let mut bs: [u8; 1] = [0];
		Ok(if self.stdin.read(&mut bs)? == 0 {
			None
		} else {
			Some(bs[0])
		})
	}
}

impl Drop for StdinRawMode {
	fn drop(&mut self) {
		tcsetattr(self.stdin.as_raw_fd(), TCSAFLUSH, &self.original_terminal).unwrap();
	}
}
