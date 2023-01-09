use std::io::stdout;

use anyhow::{Context, Result};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
    Terminal,
};

use crate::container::Container;

// コンテナの選択画面
pub fn select_container(containers: &[Container]) -> Result<&Container> {
    let backend = CrosstermBackend::new(stdout());
    let mut term = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    // 選択状態の保持
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        // コンテナ一覧を表示
        term.draw(|frame| {
            let mut items = Vec::new();
            for container in containers {
                let text = format!(
                    "{}, {}, {}, {}",
                    container.get_id(),
                    container.get_image(),
                    container.get_names(),
                    container.get_status()
                );
                let item = ListItem::new(text);
                items.push(item);
            }

            let hl_style = Style::default().fg(Color::Black).bg(Color::White);
            let list_view = List::new(items).highlight_style(hl_style);

            frame.render_stateful_widget(list_view, frame.size(), &mut state);
        })?;

        match read()? {
            Event::Key(event) => match event.code {
                // 選択位置を上に移動
                KeyCode::Up => {
                    if let Some(x) = state.selected() {
                        if 0 < x {
                            state.select(Some(x - 1));
                        }
                    }
                }

                // 選択位置を下に移動
                KeyCode::Down => {
                    if let Some(x) = state.selected() {
                        if x < containers.len() - 1 {
                            state.select(Some(x + 1));
                        }
                    }
                }

                // 選択
                KeyCode::Enter => {
                    break;
                }

                // 中断
                KeyCode::Esc => {
                    state.select(None);
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    let selected = state.selected().context("none selected")?;
    let container = containers.get(selected).context("invalid selected")?;
    Ok(container)
}
