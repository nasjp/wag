use std::fmt;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use termios::{
    tcsetattr, Termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
    TCSAFLUSH, VMIN, VTIME,
};

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut stdin = StdinRawMode::enable()?;
    let mut stdout = Stdout::new();

    loop {
        editor_refresh_screen(&mut stdout)?;
        match editor_process_key_press(&mut stdin, ctrl_key(b'q')) {
            Err(e) => return Err(e),
            _ => continue,
        }
    }
}

fn editor_refresh_screen(stdout: &mut Stdout) -> Result<Option<u8>> {
    stdout.write_flush(b"\x1b[2J")?;
    stdout.write_flush(b"\x1b[H")?;

    editor_draw_row(stdout, "~", 24)?;

    stdout.write_flush(b"\x1b[H")?;

    Ok(None)
}

fn editor_draw_row(stdout: &mut Stdout, header: &str, window_height: usize) -> Result<Option<u8>> {
    for _ in 0..=window_height {
        stdout.write_flush(format!("{}\r\n", header).as_bytes())?;
    }

    Ok(None)
}

fn editor_process_key_press(stdin: &mut StdinRawMode, quit_key: u8) -> Result<Option<u8>> {
    if let Some(key) = editor_read_key(stdin)? {
        if key == quit_key {
            return Err(Error::Quit);
        }

        return Ok(Some(key));
    }

    Ok(None)
}

fn editor_read_key(stdin: &mut StdinRawMode) -> Result<Option<u8>> {
    return stdin.read_byte();
}

pub struct StdinRawMode {
    stdin: io::Stdin,
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
        read_byte(&mut self.stdin)
    }
}

fn read_byte<R: Read>(reader: R) -> Result<Option<u8>> {
    match reader.bytes().next().transpose() {
        Ok(ret) => Ok(ret),
        Err(err) => Err(Error::from(err)),
    }
}

pub struct Stdout {
    stdout: io::Stdout,
}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout {
            stdout: io::stdout(),
        }
    }

    fn write_flush(&mut self, bytes: &[u8]) -> Result<()> {
        self.stdout.write(bytes)?;
        self.stdout.flush()?;
        Ok(())
    }
}

fn ctrl_key(key: u8) -> u8 {
    key & 0x1f
}

impl Drop for StdinRawMode {
    fn drop(&mut self) {
        tcsetattr(self.stdin.as_raw_fd(), TCSAFLUSH, &self.original_terminal).unwrap();
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Quit,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            Quit => write!(f, "quit"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
