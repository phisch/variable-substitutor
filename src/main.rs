use futures::{channel::mpsc::channel, SinkExt, StreamExt};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fmt::Debug,
    fs::{self},
    path::{Path, PathBuf},
    process::exit,
};
use tracing::{error, info};

use clap::Parser;

/// Quick and dirty CLI tool to substitute variables in a template file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to the template file
    template: PathBuf,

    /// path to the variables file
    #[arg(
        short,
        long,
        help = "Path to variables file. Defaults to `variables.toml` in the same directory as the template file."
    )]
    variables: Option<PathBuf>,

    /// path to the output file
    #[arg(
        short,
        long,
        help = "Path to the output file. Defaults to `~/.config/zed/themes/<FILENAME>` where `<FILENAME>` is the name of the template file without `.template` suffix."
    )]
    output: Option<PathBuf>,
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut args = Args::parse();

    if !args.template.is_file() {
        error!(
            "Template file '{}' does not exist.",
            args.template.display()
        );
        exit(1);
    }

    dbg!(&args.template);

    if args.variables.is_none() {
        let mut default_variables_path = args
            .template
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();
        default_variables_path.push("variables.toml");
        if !default_variables_path.is_file() {
            error!(
                "Default variables file '{}' does not exist. Please create it!",
                default_variables_path.display()
            );
            exit(1);
        }
        args.variables = Some(default_variables_path);
    } else if !args.variables.as_ref().unwrap().is_file() {
        error!(
            "Variables file '{}' does not exist.",
            args.variables.as_ref().unwrap().display()
        );
        exit(1);
    }

    if args.output.is_none() {
        let mut output_path = args.output.unwrap_or_else(|| {
            let mut default_output_path = dirs::config_dir()
                .unwrap_or_else(|| Path::new("").to_path_buf())
                .to_path_buf();
            default_output_path.push("zed/themes/");
            default_output_path
        });

        output_path.push(
            args.template
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );

        output_path.set_extension("json");

        args.output = Some(output_path);
    }

    let mut substitutor =
        Substitutor::new(args.template, args.variables.unwrap(), args.output.unwrap());

    futures::executor::block_on(substitutor.watch()).expect("Failed to watch files");
}

struct Substitutor {
    template_file: PathBuf,
    variables_file: PathBuf,
    output_file: PathBuf,
}

impl Substitutor {
    fn new(template_file: PathBuf, variables_file: PathBuf, output_file: PathBuf) -> Self {
        Self {
            template_file,
            variables_file,
            output_file,
        }
    }

    async fn watch(&mut self) -> notify::Result<()> {
        let (mut tx, mut rx) = channel(1);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    if let Err(e) = tx.send(res).await {
                        error!("Failed to send watch event: {}", e);
                    }
                })
            },
            Config::default(),
        )
        .expect("Failed to create watcher");

        watcher.watch(&self.template_file, RecursiveMode::Recursive)?;
        watcher.watch(&self.variables_file, RecursiveMode::NonRecursive)?;

        info!(
            "Watching for changes in '{}' and '{}', happy theming!",
            self.template_file.display(),
            self.variables_file.display()
        );

        while let Some(res) = rx.next().await {
            if let Ok(event) = res {
                if let notify::EventKind::Access(notify::event::AccessKind::Close(_)) = event.kind {
                    self.substitute_variables()
                        .expect("Failed to substitute variables");
                }
            } else if let Err(e) = res {
                error!("watch error: {:?}", e);
            }
        }

        Ok(())
    }

    fn substitute_variables(&self) -> std::io::Result<()> {
        let variables: toml::Value =
            toml::from_str(&fs::read_to_string(&self.variables_file)?).unwrap();

        let mut content = fs::read_to_string(&self.template_file)?.to_string();

        if let Some(colors) = variables.get("colors").and_then(|c| c.as_table()) {
            for (key, value) in colors {
                let key: String = format!("${}", key);
                if let Some(value) = value.as_str() {
                    content = content.replace(&key, value);
                }
            }
        }

        if let Err(e) = fs::write(&self.output_file, &content) {
            error!(
                "Could not write to the output file: {:?}, error: {}",
                self.output_file, e
            );
            std::process::exit(1);
        } else {
            info!("Wrote changes to: {:?}", self.output_file);
        }

        Ok(())
    }
}
