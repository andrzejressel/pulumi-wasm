use crate::pulumi::Pulumi;
use anyhow::{Context, Error};
use clap::{arg, Args, Parser, Subcommand};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log4rs::Config;
use pulumi_wasm_proto::grpc;
use pulumi_wasm_runner_component_creator::source::{
    GithubPulumiWasmSource, ProviderSource, PulumiWasmSource,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

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
        #[arg(
            long="provider",
            value_parser = parse_key_val::<String, PathBuf>,
            help="Example: --provider provider_name=provider.wasm --provider provider2_name=provider2.wasm "
        )]
        providers: Vec<(String, PathBuf)>,
        #[arg(long)]
        pulumi_wasm: Option<PathBuf>,
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

/// Parse a single key-value pair
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
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
            use pulumi_wasm_runner_component_creator::source::FileSource;
            let providers: BTreeMap<_, _> = providers
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        Box::new(FileSource::new(v.clone())) as Box<dyn ProviderSource>,
                    )
                })
                .collect();
            log::info!("Creating final component");
            let pulumi_wasm_source: Box<dyn PulumiWasmSource> = match pulumi_wasm {
                None => Box::new(GithubPulumiWasmSource {}),
                Some(location) => Box::new(FileSource::new(location.clone())),
            };

            let component = pulumi_wasm_runner_component_creator::create(
                providers,
                &GithubPulumiWasmSource {},
                pulumi_wasm_source.as_ref(),
                fs::read(program)
                    .context(format!("Cannot read program {}", program.to_str().unwrap()))?,
            )
            .await?;
            log::info!("Created final component");
            let wasm = component;

            let uuid = Uuid::now_v7().to_string();
            fs::write(format!("{uuid}.wasm"), &wasm)?;

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
            log::info!("Creating root stack");
            pulumi.create_root_stack().await?;
            log::info!("Created root stack. Invoking main");
            pulumi.start().await?;
        }
    }

    Ok(())
}
