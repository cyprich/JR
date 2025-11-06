use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::app::{App, FocusedWidget};

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let block = Block::bordered()
        //     .title_alignment(Alignment::Center)
        //     .border_type(BorderType::Rounded);

        let areas: [Rect; 4] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Min(0),
                Constraint::Length(9),
            ])
            .areas(area);

        let inner_area: [Rect; 2] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(areas[1]);

        self.render_top_bar(areas[0], buf);
        self.render_task_list(inner_area[0], buf);
        self.render_task_description(inner_area[1], buf);
        self.render_gant(areas[2], buf);
        self.render_help(areas[3], buf);
    }
}

impl App {
    fn render_top_bar(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TopBar.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if matches!(self.focused_widget, crate::app::FocusedWidget::TopBar) {
            block = block.border_style(Style::new().fg(ratatui::style::Color::Green));
        }

        Paragraph::new(format!(
            "Currently using file: `{}`",
            self.task_path.to_str().unwrap()
        ))
        .block(block)
        .alignment(Alignment::Center)
        .render(area, buf);
    }

    fn render_task_list(&mut self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TaskList.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if matches!(self.focused_widget, crate::app::FocusedWidget::TaskList) {
            block = block.border_style(Style::new().fg(ratatui::style::Color::Green));
        }

        let items: Vec<ListItem> = self
            .task_list
            .task_manager
            .format_all_tasks()
            .into_iter()
            .map(|t| ListItem::new(t))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.task_list.state);
    }

    fn render_task_description(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TaskDescription.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if matches!(
            self.focused_widget,
            crate::app::FocusedWidget::TaskDescription
        ) {
            block = block.border_style(Style::new().fg(ratatui::style::Color::Green));
        }

        block.render(area, buf);
    }

    fn render_gant(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::Gant.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if matches!(self.focused_widget, crate::app::FocusedWidget::Gant) {
            block = block.border_style(Style::new().fg(ratatui::style::Color::Green));
        }

        block.render(area, buf);
    }

    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::Help.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if matches!(self.focused_widget, crate::app::FocusedWidget::Help) {
            block = block.border_style(Style::new().fg(ratatui::style::Color::Green));
        }

        let mut text: Vec<Line<'_>> = vec![
            "Press q to exit".into(),
            "Press Tab to focus to next widget".into(),
            "Press Shift+Tab to focus to previous widget".into(),
            format!(
                "Currently focused widget: {}",
                self.focused_widget.to_string()
            )
            .into(),
        ];

        match self.focused_widget {
            FocusedWidget::TaskList => add_task_list_info(&mut text),
            _ => (),
        }

        let lines = Text::from(text);

        Paragraph::new(lines).block(block).render(area, buf);

        // block.render(area, buf);
    }
}

fn add_task_list_info(text: &mut Vec<Line<'_>>) {
    text.push(Line::from("Select task using Up ↑ and Down ↓ arrows"));
    text.push(Line::from("Press Esc to deselect all"));
    text.push(Line::from("Press Enter to show description"));
}
