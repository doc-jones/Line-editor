use std::io::{stdout, Write};

use crossterm::{
    event::read,
    event::Event,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};

fn main()-> Result<()> {
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?
        .execute(EnableMouseCapture)?;

    terminal::enable_raw_mode()?;

    match event::read()? {
        Event::Key(event) => {
            println!("{:?}", event)
        }
        Event::Mouse(event) => {
            println!("{:?}", event)
        }
        Event::Resize(width, height) => {
            println!("width: {} and height: {}", width, height)
        }
    }

    stdout().execute(DisableMouseCapture)?;

    terminal::disable_raw_mode()?;

    Ok(())
}