use std::io::{stdout, Write};

use crossterm::{
        event::{KeyEvent, KeyCode, Event}, 
    event::{read, DisableMouseCapture, EnableMouseCapture},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};

fn main()-> Result<()> {
    let mut buffer = String::new();


    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?
        // .execute(EnableMouseCapture)?;

    terminal::enable_raw_mode()?;

    loop {
        match read()? {
            Event::Key(KeyEvent {code, modifiers}) => {
                match code {
                    KeyCode::Char(c) => {
                        buffer.push(c);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                };
                println!("{:?}", event)
            }
            Event::Mouse(event) => {
                println!("{:?}", event)
            }
            Event::Resize(width, height) => {
                println!("width: {} and height: {}", width, height)
            }
        }
    }


    // stdout().execute(DisableMouseCapture)?;

    terminal::disable_raw_mode()?;

    Ok(())
}