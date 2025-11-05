use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, Paragraph, Row, Table, Widget},
};
use task_library::task::TaskManager;

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let middle_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)])
            .split(area);

        let side_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(4), Constraint::Fill(1)])
            .split(middle_layout[0]);

        let table_widget = get_tasks_table(&self.tm)
            .block(block.clone().title("Tasks"))
            .fg(Color::Cyan)
            .bg(Color::Black);

        let name_widget = Paragraph::new("Task Manager\nratatask")
            .block(block.clone())
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        let action_names = [
            "Add task",
            "Remove task",
            "Sort by ID",
            "Sort by Planned Date",
            "Sort by Planned Duration",
        ];
        let actions_widget = List::new(action_names)
            .block(block.clone().title("Actions"))
            .fg(Color::Cyan)
            .bg(Color::Black);

        table_widget.render(middle_layout[1], buf);
        name_widget.render(side_layout[0], buf);
        actions_widget.render(side_layout[1], buf);
    }
}

fn get_tasks_table(tm: &TaskManager) -> Table {
    let t = Table::default().header(
        Row::new(vec![
            "ID",
            "Name",
            "Description",
            "Priority",
            "Planned date",
            "Real date",
        ])
        .style(Style::new().bold().fg(Color::Black).bg(Color::Cyan)),
    ); // TODO add other fields 

    let row_widths = [
        Constraint::Length(5),
        Constraint::Min(0),
        Constraint::Min(0),
        Constraint::Min(0),
        Constraint::Min(0),
        Constraint::Min(0),
    ];

    let rows: Vec<Row> = tm
        .get_tasks()
        .iter()
        .map(|task| {
            Row::new(vec![
                task.id.to_string(),
                task.name.clone(),
                task.description.clone(),
                task.priority.to_string(),
                format!(
                    "{} to {}",
                    task.planned_from.to_string(),
                    task.calculate_planned_end().to_string(),
                ),
                format!(
                    "{} to {}",
                    // task.real_from.unwrap_or("-"),
                    // task.calculate_real_end().unwrap_or("-")
                    match task.real_from {
                        Some(val) => val.to_string(),
                        None => String::from("-"),
                    },
                    match task.calculate_real_end() {
                        Some(val) => val.to_string(),
                        None => String::from("-"),
                    }
                ),
            ])
        })
        .collect();

    t.rows(rows).widths(row_widths)
}
