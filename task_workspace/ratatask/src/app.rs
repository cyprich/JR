use std::{fmt::Display, path::PathBuf, task};

use crate::{
    event::{AppEvent, Event, EventHandler, TaskListEvent},
    task_list::{self, TaskList},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use task_library::{control::deserialize_json, task::TaskManager};

#[derive(Debug)]
pub enum FocusedWidget {
    TopBar,
    TaskList,
    TaskDescription,
    Gant,
    Help,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub focused_widget: FocusedWidget,
    pub task_list: TaskList,
    pub task_path: PathBuf,
}

impl Display for FocusedWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FocusedWidget::TopBar => write!(f, "Top Bar"),
            FocusedWidget::TaskList => write!(f, "Task List"),
            FocusedWidget::TaskDescription => write!(f, "Task Description"),
            FocusedWidget::Gant => write!(f, "Gant"),
            FocusedWidget::Help => write!(f, "Help"),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let task_path = PathBuf::from("tasks.json");
        Self {
            running: true,
            events: EventHandler::new(),
            focused_widget: FocusedWidget::TopBar,
            task_list: TaskList::new().open(&task_path),
            task_path,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
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
                    AppEvent::FocusNext => self.focus_next(),
                    AppEvent::FocusPrevious => self.focus_previous(),
                    AppEvent::TaskList => self.handle_task_list_event(&mut self, task_list_event: TaskListEvent), 
                },
            }
        }
        Ok(())
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Tab => self.events.send(AppEvent::FocusNext),
            KeyCode::BackTab => self.events.send(AppEvent::FocusPrevious),

            _ => {}
        }

        match self.focused_widget {
            FocusedWidget::TaskList => match key_event.code {
                KeyCode::Esc => self
                    .events
                    .send(AppEvent::TaskList(crate::event::TaskListEvent::Deselect)),
                KeyCode::Enter => self.events.send(AppEvent::TaskList(
                    crate::event::TaskListEvent::ShowTaskDescription,
                )),
                KeyCode::Up => self.events.send(AppEvent::TaskList(
                    crate::event::TaskListEvent::SelectPrevious,
                )),
                KeyCode::Down => self
                    .events
                    .send(AppEvent::TaskList(crate::event::TaskListEvent::SelectNext)),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn focus_next(&mut self) {
        self.focused_widget = match self.focused_widget {
            FocusedWidget::TopBar => FocusedWidget::TaskList,
            FocusedWidget::TaskList => FocusedWidget::TaskDescription,
            FocusedWidget::TaskDescription => FocusedWidget::Gant,
            FocusedWidget::Gant => FocusedWidget::Help,
            FocusedWidget::Help => FocusedWidget::TopBar,
        }
    }

    pub fn focus_previous(&mut self) {
        self.focused_widget = match self.focused_widget {
            FocusedWidget::TopBar => FocusedWidget::Help,
            FocusedWidget::TaskList => FocusedWidget::TopBar,
            FocusedWidget::TaskDescription => FocusedWidget::TaskList,
            FocusedWidget::Gant => FocusedWidget::TaskDescription,
            FocusedWidget::Help => FocusedWidget::Gant,
        }
    }

    pub fn handle_task_list_event(&mut self, task_list_event: TaskListEvent) {
        match task_list_event {
            TaskListEvent::SelectNext => self.task_list.state.select_next(),
            TaskListEvent::SelectPrevious =>self.task_list.state.select_previous(), 
            TaskListEvent::Deselect =>self.task_list.state.select(None), 
            TaskListEvent::ShowTaskDescription => todo!(),
        }
    }
    }
