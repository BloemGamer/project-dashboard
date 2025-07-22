use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style, Stylize},
    widgets::{self, Block, List, ListItem, Paragraph, Widget},
    DefaultTerminal,
    Frame,
};

use crate::{
    commands::{
        self
    },
    structs::Data,
    tui::{
        self,
    }
};



pub fn run(terminal: &mut DefaultTerminal, data: &mut Data) -> tui::tui::TuiState
{
    'tasks_render_loop: loop
    {
        // rendering
        terminal.draw(|frame| render(frame, data)).unwrap();

        // input handeling
        if let Event::Key(key) = event::read().unwrap()
        {
            match key.code
            {
                event::KeyCode::Esc => { break 'tasks_render_loop; }
                event::KeyCode::Char(char) =>
                {
                    if let Some(tasks) = data.tasks.as_mut()
                    {
                        match char
                        {
                            'X' =>
                            {
                                if let Some(index) = tasks.list_state.selected()
                                {
                                    tasks.tasks.remove(index);
                                };
                            }
                            'k' =>
                            {
                                tasks.list_state.select_previous();
                            }
                            'j' =>
                            {
                                tasks.list_state.select_next();
                            }
                            _ => {},
                        }
                    }
                }
                _ => {},
            }
        }
    }
    commands::tasks::write_tasks(data.tasks.as_ref().unwrap());
    tui::tui::TuiState::Exit
}

fn render(frame: &mut Frame, data: &mut Data)
{
    let chunks: [Rect; 1] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Fill(1)].as_ref())
        .areas(frame.area());

    let border_area: Rect = chunks[0];

    let chunks_inner: [Rect; 3] = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(20), Constraint::Percentage(50)].as_ref())
        .areas(border_area);

    let inner_name_area: Rect = chunks_inner[0];
    let inner_priority_area: Rect = chunks_inner[1];
    let inner_description_area: Rect = chunks_inner[2];

    Block::bordered().border_type(widgets::BorderType::Rounded)
        .fg(Color::Magenta)
        .render(border_area, frame.buffer_mut());


    let list_name: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.task.clone()))
        )
        .highlight_symbol(">")
        .highlight_style(Style::default()
            .fg(Color::Green)
        );

    let list_priority: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.priority.to_string()))
        )
        .highlight_style(Style::default()
            .fg(Color::Green)
        );

    let list_description: List<'_> = List::new(data.tasks.as_ref().unwrap().tasks
            .iter()
            .map(|x| ListItem::from(x.explanation.clone()))
        )
        .highlight_style(Style::default()
            .fg(Color::Green)
        );

    let tasks: &mut commands::tasks::Tasks = data.tasks.as_mut().unwrap();
    frame.render_stateful_widget(list_name, inner_name_area, &mut tasks.list_state);
    frame.render_stateful_widget(list_priority, inner_priority_area, &mut tasks.list_state);
    frame.render_stateful_widget(list_description, inner_description_area, &mut tasks.list_state);

    Paragraph::new("Hello World! :D").render(frame.area(), frame.buffer_mut());
}
