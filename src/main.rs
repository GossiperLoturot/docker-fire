// dockerコンテナの起動を簡単に行うアプリ
// 存在するコンテナを一覧表示し、ユーザの入力によって選択したコンテナを起動する
// 選択にはコンテナの順序を利用する

mod container;
mod ui;

use anyhow::Result;

use crate::{
    container::{get_containers, start_container},
    ui::select_container,
};

fn main() -> Result<()> {
    let containers = get_containers()?;

    if let Some(container) = select_container(&containers)? {
        println!(
            "starting container (image: {}, status: {}, names: {})",
            container.get_image(),
            container.get_status(),
            container.get_names()
        );

        let id = container.get_id();
        start_container(id)?;
    }

    Ok(())
}
