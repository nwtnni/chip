use std::io::Write;
use std::time;
use std::path;

use structopt::StructOpt;

use termion::cursor;
use termion::clear;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// 1s
const SECOND: time::Duration = time::Duration::from_secs(1);

/// 1s / 60Hz delay between sound and delay timer decrements.
const TICK: time::Duration = time::Duration::from_nanos(16_666_666);

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

impl Drop for Opt {
    fn drop(&mut self) {
        print!("{}{}{}", clear::All, cursor::Goto(0, 0), cursor::Show);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let file = std::fs::read(&args.path)?;

    let mut chip = chip::Chip::new(file);
    let mut timer = time::Instant::now();

    let mut hz = args.hz;
    let mut delay = SECOND / hz;
    let mut fuel = -1;

    let stdin = termion::async_stdin();
    let mut stdout = std::io::stdout().into_raw_mode()?;
    let mut stream = stdin.events();


    write!(stdout, "{}{}", cursor::Hide, clear::All)?;
    chip.draw(0, 0, &mut stdout)?;

    loop {

        std::thread::sleep(delay);
        fuel = if fuel <= 0 { fuel } else { fuel - 1 };

        match stream.next() {
        | Some(Ok(Event::Key(Key::Esc))) => break,
        | Some(Ok(Event::Key(Key::Char(' ')))) => fuel = if fuel < 0 { 0 } else { -1 },
        | Some(Ok(Event::Key(Key::Char('-')))) => { hz -= 10; delay = SECOND / hz; }
        | Some(Ok(Event::Key(Key::Char('+')))) => { hz += 10; delay = SECOND / hz; }
        | Some(Ok(Event::Key(Key::Char('n'))))
        | Some(Ok(Event::Key(Key::Char('>'))))
        | Some(Ok(Event::Key(Key::Down))) if fuel >= 0 => fuel += 1,
        | Some(Ok(Event::Key(key))) => chip.set_key(key),
        | _ => (),
        }

        if fuel == 0 { timer += delay; continue }

        if std::time::Instant::now() - timer > TICK {
            timer = std::time::Instant::now();
            chip.tick();
            chip.draw(0, 0, &mut stdout)?;
            stdout.flush()?;
        }

        chip.step();
    }

    Ok(())
}
