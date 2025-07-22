use ratatui::{
    crossterm::event::{self, Event}, layout::{Constraint, Direction, Layout}, prelude::Rect, style::{Color, Style, Stylize}, widgets::{self, Block, List, ListItem, Paragraph, Widget}, DefaultTerminal, Frame
};

use crate::commands::{
        self, tasks
    };

use crate::structs::Data;

pub fn start()
{
    color_eyre::install().unwrap();
}

pub fn run(mut terminal: DefaultTerminal, data: &mut Data)
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
                event::KeyCode::Char(char) =>
                {
                    match char
                    {
                        'X' =>
                        {
                            if let Some(tasks) = data.tasks.as_mut()
                            {
                                if let Some(index) = tasks.list_state.selected()
                                {
                                    tasks.tasks.remove(index);
                                };
                            }
                        }
                        'k' =>
                        {
                            if let Some(tasks) = data.tasks.as_mut()
                            {
                                tasks.list_state.select_previous();
                            }
                        }
                        'j' =>
                        {
                            if let Some(tasks) = data.tasks.as_mut()
                            {
                                tasks.list_state.select_next();
                            }
                        }
                        _ => {},
                    }
                }
                _ => {},
            }
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
}

fn render(frame: &mut Frame, data: &mut Data)
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


    let list: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.task.clone()))
        )
        .highlight_symbol(">")
        .highlight_style(Style::default()
            .fg(Color::Green)
        )
    ;
        //.render(inner_area, frame.buffer_mut());

    let tasks: &mut tasks::Tasks = data.tasks.as_mut().unwrap();
    frame.render_stateful_widget(list, inner_area, &mut tasks.list_state);

    Paragraph::new("Hello World! :D").render(frame.area(), frame.buffer_mut());
}
