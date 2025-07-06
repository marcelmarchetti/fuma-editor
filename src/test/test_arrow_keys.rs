use std::io;

fn test_arrow_keys() -> io::Result<()> {
    println!("Pulsa teclas (q para salir)");

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        println!("Saliendo...");
                        break;
                    }
                    KeyCode::Char(c) => println!("Has pulsado el carácter '{}'", c),
                    KeyCode::Enter => println!("Enter"),
                    KeyCode::Backspace => println!("Backspace"),
                    KeyCode::Left => println!("Flecha izquierda"),
                    KeyCode::Right => println!("Flecha derecha"),
                    KeyCode::Up => println!("Flecha arriba"),
                    KeyCode::Down => println!("Flecha abajo"),
                    KeyCode::Esc => println!("Escape"),
                    _ => println!("Otra tecla"),
                }
            }
        }
    }

    Ok(())
}
