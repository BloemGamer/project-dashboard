use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Direction}, 
    style::{Color, Stylize},
    widgets::{self, Block, List, ListItem, Paragraph, Widget},
    prelude::{Rect},
    DefaultTerminal,
    Frame
};

use crate::structs::Data;

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal, mut data: &mut Data)
{
    'main_render_loop: loop
    {
        // rendering
        terminal.draw(|frame| render(frame, data)).unwrap();

        // input handeling
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

fn render(frame: &mut Frame, mut data: &Data)
{
    let chunks: [Rect; 1] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Fill(1)].as_ref())
        .areas(frame.area());

    let border_area: Rect = chunks[0];

    let [inner_area] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Fill(1)].as_ref())
        .areas(border_area);

    Block::bordered().border_type(widgets::BorderType::Rounded).
        fg(Color::Magenta).
        render(border_area, frame.buffer_mut());


    List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.task.clone()))
        )
        .render(inner_area, frame.buffer_mut());


    Paragraph::new("Hello World! :D").render(frame.area(), frame.buffer_mut());

}
