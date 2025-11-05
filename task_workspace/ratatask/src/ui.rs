use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, List, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let block = Block::bordered()
        let main_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        let sub_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(main_layout[0]);

        let block = Block::bordered()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let table_widget = self
            .get_tasks_table()
            .block(block.clone().title("Tasks"))
            .fg(Color::Cyan)
            .bg(Color::Black);

        let name_widget = Paragraph::new("Task Manager")
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

        table_widget.render(main_layout[1], buf);
        name_widget.render(sub_layout[0], buf);
        actions_widget.render(sub_layout[1], buf);
    }
}
