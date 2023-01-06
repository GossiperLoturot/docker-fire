// dockerコンテナの起動を簡単に行うアプリ
// 存在するコンテナを一覧表示し、ユーザの入力によって選択したコンテナを起動する
// 選択にはコンテナの順序を利用する

mod container;

use anyhow::{bail, Context, Result};

use crate::container::{get_containers, start_container};

fn main() -> Result<()> {
    let containers = get_containers()?;

    // コンテナが存在しない場合は終了
    if containers.is_empty() {
        bail!("container is empty")
    }

    println!(
        "input index of container to start: 0 ..= {}",
        containers.len() - 1
    );

    // コンテナの一覧を表示
    for (i, container) in containers.iter().enumerate() {
        println!(
            "{} image: {}, status: {}, names: {}",
            i,
            container.get_image(),
            container.get_status(),
            container.get_names()
        );
    }

    // 起動するコンテナの選択を待機
    let current_stdin = {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .context("failed to read stdin")?;
        buf
    };

    let selected_index = current_stdin
        .trim()
        .parse::<usize>()
        .context("failed to parse input")?;

    let selected_container = containers
        .get(selected_index)
        .context("out of container list range")?;

    println!(
        "starting container: image: {}, status: {}, names: {}",
        selected_container.get_image(),
        selected_container.get_status(),
        selected_container.get_names()
    );

    // コンテナを起動
    let id = selected_container.get_id();
    start_container(id)?;

    Ok(())
}
