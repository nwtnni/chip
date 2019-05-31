use std::io::Write;

use termion::cursor;
use termion::clear;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let path = &args[1];
    let file = std::fs::read(path)?;
    let mut chip = chip::Chip::new(file);

    let stdin = termion::async_stdin();
    let mut stdout = std::io::stdout().into_raw_mode()?;
    let mut stream = stdin.events();

    write!(stdout, "{}", clear::All)?;
    chip.draw(&mut stdout)?;

    loop {
        match stream.next() {
        | Some(Ok(Event::Key(Key::Esc))) => break,
        | Some(Ok(Event::Key(key))) => chip.set_key(key),
        | _ => chip.clear_key(),
        }

        chip.step();
        chip.draw(&mut stdout)?;
        stdout.flush()?;
    }

    write!(stdout, "{}{}", clear::All, cursor::Goto(0, 0))?;
    Ok(())
}
