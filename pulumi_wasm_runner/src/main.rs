use crate::pulumi::{Pulumi, WasmFile};
use anyhow::Error;
use clap::{arg, Args, Parser, Subcommand};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::Config;

mod pulumi;

mod grpc {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    tonic::include_proto!("pulumirpc");
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[arg(short, long)]
        wasm: Option<String>,

        #[arg(short, long)]
        cwasm: Option<String>,
    },
    Plugins {
        #[arg(short, long)]
        wasm: Option<String>,

        #[arg(short, long)]
        cwasm: Option<String>,
    },
    Compile {
        #[arg(short, long)]
        wasm: String,
        #[arg(short, long)]
        output: String,
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
        Command::Run { wasm, cwasm } => {
            let wasm = match (wasm, cwasm) {
                (Some(_), Some(_)) => {
                    eprintln!("Cannot specify both wasm and cwasm");
                    std::process::exit(1);
                }
                (Some(wasm), None) => WasmFile::Wasm(wasm.clone()),
                (None, Some(cwasm)) => WasmFile::CompiledWasm(cwasm.clone()),
                (None, None) => {
                    eprintln!("Must specify either wasm or cwasm");
                    std::process::exit(1);
                }
            };

            let pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
            let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;
            let pulumi_stack = std::env::var("PULUMI_STACK")?;
            let pulumi_project = std::env::var("PULUMI_PROJECT")?;

            let mut pulumi = Pulumi::create(
                &wasm,
                &Some(pulumi_monitor_url),
                &Some(pulumi_engine_url),
                &Some(pulumi_stack),
                &Some(pulumi_project),
            )
            .await?;
            pulumi.create_root_stack().await?;
            pulumi.start().await?;
        }
        Command::Compile { wasm, output } => {
            let compiled = Pulumi::compile(wasm)?;
            std::fs::write(output, compiled)?;
        }
        Command::Plugins { .. } => todo!(),
    }

    Ok(())
}
