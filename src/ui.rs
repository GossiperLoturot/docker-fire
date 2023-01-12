use std::io::stdout;

use anyhow::{bail, Context, Result};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{List, ListItem},
    Terminal,
};

use crate::container::Container;

// コンテナの選択画面を開始
pub fn select_container(containers: &[Container]) -> Result<Option<&Container>> {
    if containers.is_empty() {
        bail!("containers are nothing");
    }

    let backend = CrosstermBackend::new(stdout());
    let mut term = Terminal::new(backend)?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let res = run(&mut term, containers);

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    res
}

fn run<'a, B: Backend>(
    term: &mut Terminal<B>,
    containers: &'a [Container],
) -> Result<Option<&'a Container>> {
    // 選択状態の保持
    let mut selected = 0;

    loop {
        // コンテナ一覧を表示
        term.draw(|frame| {
            let mut items = Vec::new();
            for (i, container) in containers.iter().enumerate() {
                let text = format!(
                    "{}, {}, {}, {}",
                    container.get_id(),
                    container.get_image(),
                    container.get_names(),
                    container.get_status()
                );
                let mut item = ListItem::new(text);

                // 選択項目をハイライト
                if i == selected {
                    item = item.style(Style::default().fg(Color::Black).bg(Color::White));
                }

                items.push(item);
            }

            let list_view = List::new(items);
            frame.render_widget(list_view, frame.size());
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                // 選択位置を上に移動
                KeyCode::Up => {
                    if selected == 0 {
                        selected = containers.len() - 1;
                    } else {
                        selected -= 1;
                    }
                }

                // 選択位置を下に移動
                KeyCode::Down => {
                    if selected == containers.len() - 1 {
                        selected = 0;
                    } else {
                        selected += 1;
                    }
                }

                // 選択
                KeyCode::Enter => {
                    let container = containers.get(selected).context("out of range")?;
                    return Ok(Some(container));
                }

                // 中断
                KeyCode::Esc => {
                    return Ok(None);
                }

                _ => {}
            }
        }
    }
}
