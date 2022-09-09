use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{io::{stdout, Write}, thread, time::Duration};

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut current = 0;

    loop {
        current += 1;

        let abs = match current % 2 {
            0 => "█".magenta(),
            _ => "█".blue(),
        };

        for y in 0..40 {
            for x in 0..150 {
                if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent(abs))?;
                }
            }
        }
        stdout.flush()?;  
        thread::sleep(Duration::from_millis(25));
    }

    // Ok(())
}
