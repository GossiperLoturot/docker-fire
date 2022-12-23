// dockerコンテナの起動を簡単に行うアプリ
// 存在するコンテナを一覧表示し、ユーザの入力によって選択したコンテナを起動する
// 選択にはコンテナの順序を利用する

use anyhow::{bail, Context, Result};

struct Container<'r> {
    id: &'r str,
    image: &'r str,
    status: &'r str,
    names: &'r str,
}

fn main() -> Result<()> {
    loop {
        let status = routine();

        // 成功した場合は終了
        if status.is_ok() {
            break;
        }

        // 失敗した場合は再実行を促す
        let err = status.unwrap_err();
        eprintln!("{:?}", err);

        println!("input [y] to continue, other key to quit.");

        let current_stdin = {
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .context("failed to read stdin")?;
            buf
        };

        if current_stdin.trim() != "y" {
            break;
        }
    }

    Ok(())
}

fn routine() -> Result<()> {
    // docker ps コマンドを実行して出力を取得
    let docker_ps_output = std::process::Command::new("docker")
        .args([
            "ps",
            "--all",
            "--format",
            "{{.ID}},{{.Image}},{{.Status}},{{.Names}}",
        ])
        .output()
        .context("failed to run docker ps")?;

    // 実行に失敗した場合
    if !docker_ps_output.status.success() {
        let docker_ps_stderr = String::from_utf8(docker_ps_output.stderr)
            .context("failed to encode docker ps stderr")?;
        bail!("failed to run docker ps. {:?}", docker_ps_stderr);
    }

    let docker_ps_stdout =
        String::from_utf8(docker_ps_output.stdout).context("failed to encode docker ps stdout")?;

    // docker psコマンドの出力を解析
    let mut container_list = vec![];
    for line in docker_ps_stdout.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut elements = line.split(',');

        let id = elements
            .next()
            .context("failed to parse docker ps stdout")?;
        let image = elements
            .next()
            .context("failed to parse docker ps stdout")?;
        let status = elements
            .next()
            .context("failed to parse docker ps stdout")?;
        let names = elements
            .next()
            .context("failed to parse docker ps stdout")?;

        container_list.push(Container {
            id,
            image,
            status,
            names,
        });
    }

    // コンテナが存在しない場合は終了
    if container_list.is_empty() {
        bail!("container is empty")
    }

    println!(
        "input index of container to start: 0 ..= {}",
        container_list.len() - 1
    );

    // コンテナの一覧を表示
    for (i, container) in container_list.iter().enumerate() {
        println!(
            "{} image: {}, status: {}, names: {}",
            i, container.image, container.status, container.names
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

    let selected_container = container_list
        .get(selected_index)
        .context("out of container list range")?;

    println!(
        "starting container: image: {}, status: {}, names: {}",
        selected_container.image, selected_container.status, selected_container.names
    );

    // コンテナを起動
    let _ = std::process::Command::new("docker")
        .args(["start", "-ai", selected_container.id])
        .status()
        .context("failed to run docker start")?;

    Ok(())
}
