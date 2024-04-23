use anyhow::Error;
use async_trait::async_trait;
use prost::Message;
use wasmtime::component::{Component, Instance, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::grpc::engine_client::EngineClient;
use crate::grpc::resource_monitor_client::ResourceMonitorClient;
use crate::grpc::{
    GetRootResourceRequest, RegisterResourceOutputsRequest, RegisterResourceRequest,
    RegisterResourceResponse, SetRootResourceRequest,
};
use crate::pulumi::server::Main;

pub struct Pulumi {
    plugin: Main,
    _instance: Instance,
    store: Store<SimplePluginCtx>,
}

pub(crate) mod server {
    wasmtime::component::bindgen!({
        path: "../wits/world.wit",
        world: "main",
        async: true
    });
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

struct MyState {
    pulumi_monitor_url: Option<String>,
    pulumi_engine_url: Option<String>,
    pulumi_stack: Option<String>,
    pulumi_project: Option<String>,
}

#[async_trait]
impl server::component::pulumi_wasm::external_world::Host for MyState {
    async fn is_in_preview(&mut self) -> wasmtime::Result<bool> {
        Ok(std::env::var("PULUMI_DRY_RUN")
            .map(|s| s.to_ascii_lowercase() == "true")
            .unwrap_or_else(|_| false))
    }
    async fn register_resource(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        Ok(self.register_async(request).await?)
    }
    async fn register_resource_outputs(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        Ok(self.register_resource_outputs_async(request).await?)
    }
    async fn get_root_resource(&mut self) -> wasmtime::Result<String> {
        Ok(self.get_root_resource_async().await?)
    }
}

#[async_trait]
impl server::component::pulumi_wasm::log::Host for MyState {
    async fn log(
        &mut self,
        content: server::component::pulumi_wasm::log::Content,
    ) -> wasmtime::Result<()> {
        log::logger().log(
            &log::Record::builder()
                .metadata(
                    log::Metadata::builder()
                        .level(match content.level {
                            server::component::pulumi_wasm::log::Level::Trace => log::Level::Trace,
                            server::component::pulumi_wasm::log::Level::Debug => log::Level::Debug,
                            server::component::pulumi_wasm::log::Level::Info => log::Level::Info,
                            server::component::pulumi_wasm::log::Level::Error => log::Level::Error,
                            server::component::pulumi_wasm::log::Level::Warn => log::Level::Warn,
                        })
                        .target(&content.target)
                        .build(),
                )
                .args(format_args!("{}", content.args))
                .module_path(content.module_path.as_deref())
                .file(content.file.as_deref())
                .line(content.line)
                .key_values(
                    &content
                        .key_values
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<(&str, &str)>>(),
                )
                .build(),
        );

        Ok(())
    }
}

impl MyState {
    async fn register_async(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        let engine_url = self
            .pulumi_monitor_url
            .clone()
            .ok_or(Error::msg("pulumi_monitor_url not set"))?;

        let request = RegisterResourceRequest::decode(&mut request.as_slice())?;

        let mut client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;

        let result = client.register_resource(request).await?;

        Ok(result.get_ref().encode_to_vec())
    }

    async fn register_resource_outputs_async(
        &mut self,
        request: Vec<u8>,
    ) -> wasmtime::Result<Vec<u8>> {
        let engine_url = self
            .pulumi_monitor_url
            .clone()
            .ok_or(Error::msg("pulumi_monitor_url not set"))?;

        let request = RegisterResourceOutputsRequest::decode(&mut request.as_slice())?;

        let mut client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;

        let result = client.register_resource_outputs(request).await?;

        Ok(result.get_ref().encode_to_vec())
    }

    async fn set_root_resource_async(&mut self, urn: String) -> wasmtime::Result<()> {
        let engine_url = self
            .pulumi_engine_url
            .clone()
            .ok_or(Error::msg("pulumi_monitor_url not set"))?;

        let mut client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        let request = SetRootResourceRequest { urn };

        let _ = client.set_root_resource(request).await?;

        Ok(())
    }

    async fn get_root_resource_async(&mut self) -> wasmtime::Result<String> {
        let engine_url = self
            .pulumi_engine_url
            .clone()
            .ok_or(Error::msg("pulumi_monitor_url not set"))?;

        let mut client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        let request = GetRootResourceRequest {};

        let result = client.get_root_resource(request).await?;

        Ok(result.get_ref().urn.clone())
    }
}

impl WasiView for SimplePluginCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

pub enum WasmFile {
    Wasm(String),
    CompiledWasm(String),
}

impl Pulumi {
    pub async fn create(
        pulumi_wasm_file: &WasmFile,
        pulumi_monitor_url: &Option<String>,
        pulumi_engine_url: &Option<String>,
        pulumi_stack: &Option<String>,
        pulumi_project: &Option<String>,
    ) -> Result<Pulumi, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config)?;

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Main::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let table = ResourceTable::new();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build();

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                // logger: SimpleLogger {},
                table,
                context: wasi_ctx,
                my_state: MyState {
                    pulumi_monitor_url: pulumi_monitor_url.clone(),
                    pulumi_engine_url: pulumi_engine_url.clone(),
                    pulumi_stack: pulumi_stack.clone(),
                    pulumi_project: pulumi_project.clone(),
                },
            },
        );

        let component = match pulumi_wasm_file {
            WasmFile::Wasm(file) => Component::from_file(&engine, file),
            WasmFile::CompiledWasm(file) => unsafe { Component::deserialize_file(&engine, file) },
        }?;

        let (plugin, instance) = Main::instantiate_async(&mut store, &component, &linker).await?;

        Ok(Pulumi {
            plugin,
            _instance: instance,
            store,
        })
    }

    pub fn compile(pulumi_wasm_file: &str) -> Result<Vec<u8>, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config).unwrap();

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Main::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

        let component = Component::from_file(&engine, pulumi_wasm_file)?;

        component.serialize()
    }

    pub async fn create_root_stack(&mut self) -> Result<(), Error> {
        let request = RegisterResourceRequest {
            r#type: "pulumi:pulumi:Stack".to_string(),
            name: format!(
                "{}-{}",
                self.store
                    .data_mut()
                    .my_state
                    .pulumi_project
                    .clone()
                    .unwrap(),
                self.store.data_mut().my_state.pulumi_stack.clone().unwrap()
            ),
            custom: false,
            ..Default::default()
        };

        let result = self
            .store
            .data_mut()
            .my_state
            .register_async(request.encode_to_vec())
            .await?;

        let url = RegisterResourceResponse::decode(&mut result.as_slice())?.urn;
        self.store
            .data_mut()
            .my_state
            .set_root_resource_async(url)
            .await?;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        self.plugin
            .component_pulumi_wasm_pulumi_main()
            .call_main(&mut self.store)
            .await?;

        Ok(())
    }
}
