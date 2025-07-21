use ratatui::{
    crossterm::event::{self, Event}, widgets::{Paragraph, Widget}, DefaultTerminal, Frame
};

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal)
{
    // rendering
    terminal.draw(render).unwrap();

    // input handeling
    'main_render_loop: loop
    {
        if let Event::Key(key) = event::read().unwrap()
        {
            match key.code
            {
                event::KeyCode::Esc => { break 'main_render_loop; }
                _ => {},
            }
        }
    }
}

fn render(frame: &mut Frame)
{
    Paragraph::new("Hello ma'am :D").render(frame.area(), frame.buffer_mut());
}
