fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let path = &args[1];
    let file = std::fs::read(path)?;
    let mut iter = file.iter().peekable();

    while iter.peek().is_some() {
        let hi = *iter.next().unwrap();
        let lo = *iter.next().unwrap();
        let op = chip::asm::Asm::from(((hi as u16) << 8) | (lo as u16));
        println!("{:?}", op);
    }

    Ok(())
}
