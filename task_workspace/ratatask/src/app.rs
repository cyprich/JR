use std::{fmt::format, path::PathBuf, vec};

use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, Paragraph, Row, Table},
};
use task_library::{control::deserialize_json, task::TaskManager};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,

    pub tm: TaskManager,
    pub message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            tm: deserialize_json(&PathBuf::from("tasks.json")),
            message: String::from(""),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&self, frame.area());
            })?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::Add => self.add_task(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // KeyCode::Right => self.events.send(AppEvent::Increment),
            // KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Char('a' | 'A') => self.events.send(AppEvent::Add),
            // KeyCode::Char('l' | 'L') => self.events.send(AppEvent::List),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_task(&self) {}

    pub fn get_tasks_table(&self) -> Table {
        // let rows = self.tm.get_tasks();
        // let widths = vec![];
        // let t = Table::new(rows, widths);

        // pub planned_from: NaiveDate,
        // pub planned_duration: TimeDelta,
        // pub real_from: Option<NaiveDate>,
        // pub real_duration: Option<TimeDelta>,

        let count = self.tm.get_tasks().len();

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

        let rows: Vec<Row> = self
            .tm
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

        t.rows(rows)
            .footer(Row::new(vec![format!("Total: {} tasks", count)]))
    }
}
