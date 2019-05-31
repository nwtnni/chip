use std::io::Write;
use std::time;
use std::path;

use structopt::StructOpt;

use termion::cursor;
use termion::clear;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// 1s / 60Hz delay between sound and delay timer decrements.
const DELAY: time::Duration = time::Duration::from_nanos(16_666_666);

#[derive(Debug, StructOpt)]
#[structopt(name = "chip", about = "A terminal-based CHIP-8 emulator.")]
struct Opt {
    /// CPU frequency in hertz.
    #[structopt(long = "hz", default_value = "1000")] 
    hz: u32,

    /// Binary CHIP-8 ROM file to emulate.
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let file = std::fs::read(args.path)?;

    let mut chip = chip::Chip::new(file);
    let mut timer = time::Instant::now();
    let pause = time::Duration::from_secs(1) / args.hz;

    let stdin = termion::async_stdin();
    let mut stdout = std::io::stdout().into_raw_mode()?;
    let mut stream = stdin.events();

    write!(stdout, "{}{}", cursor::Hide, clear::All)?;
    chip.draw(&mut stdout)?;

    loop {
        match stream.next() {
        | Some(Ok(Event::Key(Key::Esc))) => break,
        | Some(Ok(Event::Key(key))) => chip.set_key(key),
        | _ => chip.clear_key(),
        }

        if std::time::Instant::now() - timer > DELAY {
            timer = std::time::Instant::now();
            chip.tick();
        }

        chip.step();
        chip.draw(&mut stdout)?;
        stdout.flush()?;
        std::thread::sleep(pause);
    }

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(0, 0), cursor::Show)?;
    Ok(())
}
