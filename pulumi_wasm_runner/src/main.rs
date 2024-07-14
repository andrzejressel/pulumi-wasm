use crate::pulumi::Pulumi;
use anyhow::Error;
use clap::{arg, Args, Parser, Subcommand};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::Config;
use pulumi_wasm_proto::grpc;
use std::path::PathBuf;

mod create_final_component;
mod model;
mod pulumi;
mod pulumi_state;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[arg(long = "provider")]
        providers: Vec<PathBuf>,
        #[arg(long)]
        pulumi_wasm: PathBuf,
        program: PathBuf,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(short, long)]
    wasm: Option<String>,

    #[arg(short, long)]
    cwasm: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = App::parse();

    let logfile = FileAppender::builder()
        // .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} - [{l}] [{M}] [{f}:{L}] {m}{n})}")))
        .encoder(Box::new(JsonEncoder::new()))
        .build("output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    let _handle = log4rs::init_config(config)?;

    match &args.command {
        Command::Run {
            providers,
            pulumi_wasm,
            program,
        } => {
            let component = create_final_component::create(providers, pulumi_wasm, program);
            let wasm = component;

            let pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
            let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;
            let pulumi_stack = std::env::var("PULUMI_STACK")?;
            let pulumi_project = std::env::var("PULUMI_PROJECT")?;

            let mut pulumi = Pulumi::create(
                wasm,
                pulumi_monitor_url,
                pulumi_engine_url,
                pulumi_stack,
                pulumi_project,
            )
            .await?;
            pulumi.create_root_stack().await?;
            pulumi.start().await?;
        }
    }

    Ok(())
}
