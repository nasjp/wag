use wia::Editor;
use wia::Result;
use wia::StdinRawMode;

fn main() -> Result<()> {
    let mut editor = Editor::new(StdinRawMode::enable()?)?;

    loop {
        if let Some(s) = editor.process_key_press()? {
            print!("{}", s);
            continue;
        }

        return Ok(());
    }
}
