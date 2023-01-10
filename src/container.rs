use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};

pub struct Container {
    id: String,
    image: String,
    status: String,
    names: String,
}

impl Container {
    pub fn new(id: String, image: String, status: String, names: String) -> Self {
        Self {
            id,
            image,
            status,
            names,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_image(&self) -> &str {
        &self.image
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_names(&self) -> &str {
        &self.names
    }
}

pub fn get_containers() -> Result<Vec<Container>> {
    // docker ps コマンドを実行して出力を取得
    let docker_ps_output = Command::new("docker")
        .args([
            "ps",
            "--all",
            "--format",
            "{{.ID}},{{.Image}},{{.Status}},{{.Names}}",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .context("failed to run docker")?;

    // 実行に失敗した場合
    if !docker_ps_output.status.success() {
        bail!("failed to run command line");
    }

    let docker_ps_stdout =
        String::from_utf8(docker_ps_output.stdout).context("failed to encode stdout")?;

    // docker psコマンドの出力を解析
    let mut containers = vec![];
    for line in docker_ps_stdout.split_terminator('\n') {
        let mut elements = line.split(',');

        let id = elements
            .next()
            .context("failed to parse docker ps stdout")?
            .to_string();
        let image = elements
            .next()
            .context("failed to parse docker ps stdout")?
            .to_string();
        let status = elements
            .next()
            .context("failed to parse docker ps stdout")?
            .to_string();
        let names = elements
            .next()
            .context("failed to parse docker ps stdout")?
            .to_string();

        let container = Container::new(id, image, status, names);
        containers.push(container);
    }

    Ok(containers)
}

pub fn start_container(id: &str) -> Result<()> {
    // コンテナを起動
    Command::new("docker")
        .args(["start", "-ai", id])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .context("failed to run docker start")?;

    Ok(())
}
