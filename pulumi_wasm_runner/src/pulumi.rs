use crate::grpc::{
    RegisterResourceOutputsRequest, RegisterResourceRequest, RegisterResourceResponse,
};
use crate::pulumi::runner::component::pulumi_wasm_external::external_world;
use crate::pulumi::runner::component::pulumi_wasm_external::external_world::Host;
use crate::pulumi::runner::component::pulumi_wasm_external::external_world::RegisteredResource;
use crate::pulumi::runner::Runner;
use anyhow::Error;
use log::info;
use prost::Message;
use pulumi_wasm_grpc_connection::pulumi_state::PulumiState;
use pulumi_wasm_proto::grpc::ResourceInvokeRequest;
use pulumi_wasm_wit::bindings_runner as runner;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct Pulumi {
    plugin: Runner,
    store: Store<SimplePluginCtx>,
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

struct MyState {
    pulumi_state: PulumiState,
}

impl Host for MyState {
    async fn register_resource_outputs(&mut self, request: Vec<u8>) -> wasmtime::Result<()> {
        self.register_resource_outputs_async(request).await
    }

    async fn resource_invoke(
        &mut self,
        request: external_world::ResourceInvokeRequest,
    ) -> wasmtime::Result<()> {
        let b = ResourceInvokeRequest::decode(&*(request.body)).unwrap();

        info!("registering resource: {:?}", b);

        self.pulumi_state
            .send_resource_invoke_request(request.output_id.into(), b);

        Ok(())
    }

    async fn register_resource(
        &mut self,
        request: external_world::RegisterResourceRequest,
    ) -> wasmtime::Result<()> {
        let b = RegisterResourceRequest::decode(&*(request.body)).unwrap();

        info!("registering resource: {:?}", b);

        self.pulumi_state
            .send_register_resource_request(request.output_id.into(), b);

        Ok(())
    }

    async fn wait_for_resource_operations(&mut self) -> wasmtime::Result<Vec<RegisteredResource>> {
        let mut outputs = Vec::new();
        for (output_id, body) in self.pulumi_state.get_created_resources().await {
            let b = RegisterResourceResponse::decode(&*body).unwrap();
            outputs.push(RegisteredResource {
                output_id: output_id.0,
                body: b.encode_to_vec(),
            });
        }
        Ok(outputs)
    }
}

impl runner::component::pulumi_wasm_external::log::Host for MyState {
    async fn log(
        &mut self,
        content: runner::component::pulumi_wasm_external::log::Content,
    ) -> wasmtime::Result<()> {
        log::logger().log(
            &log::Record::builder()
                .metadata(
                    log::Metadata::builder()
                        .level(match content.level {
                            runner::component::pulumi_wasm_external::log::Level::Trace => {
                                log::Level::Trace
                            }
                            runner::component::pulumi_wasm_external::log::Level::Debug => {
                                log::Level::Debug
                            }
                            runner::component::pulumi_wasm_external::log::Level::Info => {
                                log::Level::Info
                            }
                            runner::component::pulumi_wasm_external::log::Level::Error => {
                                log::Level::Error
                            }
                            runner::component::pulumi_wasm_external::log::Level::Warn => {
                                log::Level::Warn
                            }
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
    async fn register_resource_outputs_async(&mut self, request: Vec<u8>) -> wasmtime::Result<()> {
        let request = RegisterResourceOutputsRequest::decode(&mut request.as_slice())?;
        self.pulumi_state.register_resource_outputs(request).await
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

impl Pulumi {
    pub async fn create(
        pulumi_wasm_file: Vec<u8>,
        pulumi_monitor_url: String,
        pulumi_engine_url: String,
        pulumi_stack: String,
        pulumi_project: String,
    ) -> Result<Pulumi, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config)?;

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Runner::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        let table = ResourceTable::new();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build();

        let pulumi_state = PulumiState::new(
            pulumi_monitor_url.clone(),
            pulumi_engine_url.clone(),
            pulumi_project.clone(),
            pulumi_stack.clone(),
        )
        .await?;

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                // logger: SimpleLogger {},
                table,
                context: wasi_ctx,
                my_state: MyState { pulumi_state },
            },
        );

        info!("Creating Wasm component");
        let component = Component::from_binary(&engine, &pulumi_wasm_file)?;
        info!("Instantiating Wasm component");
        let plugin = Runner::instantiate_async(&mut store, &component, &linker).await?;
        info!("Wasm component instantiated");

        Ok(Pulumi { plugin, store })
    }

    pub fn compile(pulumi_wasm_file: &str) -> Result<Vec<u8>, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config).unwrap();

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Runner::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

        let component = Component::from_file(&engine, pulumi_wasm_file)?;

        component.serialize()
    }

    pub async fn start(&mut self, in_preview: bool) -> Result<(), Error> {
        self.plugin
            .component_pulumi_wasm_external_pulumi_main()
            .call_main(&mut self.store, in_preview)
            .await?;

        Ok(())
    }
}
