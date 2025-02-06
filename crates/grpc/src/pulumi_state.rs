use crate::output_id::OutputId;
use anyhow::{Context, Error, Result};
use futures::FutureExt;
use prost::Message;
use pulumi_wasm_proto::grpc::engine_client::EngineClient;
use pulumi_wasm_proto::grpc::resource_monitor_client::ResourceMonitorClient;
use pulumi_wasm_proto::grpc::{
    GetRootResourceRequest, RegisterResourceOutputsRequest, ResourceInvokeRequest,
};
use pulumi_wasm_proto::grpc::{
    RegisterResourceRequest, RegisterResourceResponse, SetRootResourceRequest,
};
use std::future::poll_fn;
use tokio::task::JoinSet;
use tonic::transport::Channel;

pub struct PulumiState {
    resource_monitor_client: ResourceMonitorClient<Channel>,
    root_resource: String,
    join_set: JoinSet<Result<(OutputId, Vec<u8>)>>,
}

impl PulumiState {
    pub async fn new(
        monitor_url: String,
        engine_url: String,
        pulumi_project: String,
        pulumi_stack: String,
    ) -> Result<Self> {
        let resource_monitor_client =
            ResourceMonitorClient::connect(format!("tcp://{monitor_url}")).await?;
        let engine_client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        Self::create_root_stack(
            resource_monitor_client.clone(),
            engine_client.clone(),
            pulumi_project.clone(),
            pulumi_stack.clone(),
        )
        .await?;
        let root_resource = PulumiState::get_root_resource_async(engine_url.clone()).await?;

        let s = Self {
            resource_monitor_client,
            root_resource,
            join_set: JoinSet::new(),
        };

        Ok(s)
    }

    pub fn send_register_resource_request(
        &mut self,
        output_id: OutputId,
        request: RegisterResourceRequest,
    ) {
        let monitor = self.resource_monitor_client.clone();
        self.join_set.spawn(async move {
            Self::send_register_resource_request_inner(output_id, request, monitor).await
        });
    }

    pub fn send_resource_invoke_request(
        &mut self,
        output_id: OutputId,
        request: ResourceInvokeRequest,
    ) {
        let monitor = self.resource_monitor_client.clone();
        self.join_set.spawn(async move {
            Self::send_resource_invoke_request_inner(output_id, request, monitor).await
        });
    }

    pub async fn register_resource_outputs(
        &self,
        request: RegisterResourceOutputsRequest,
    ) -> Result<()> {
        let mut monitor = self.resource_monitor_client.clone();
        let mut request = request;
        request.urn = self.root_resource.clone();
        let _ = monitor.register_resource_outputs(request).await?;
        Ok(())
    }

    pub async fn get_created_resources(&mut self) -> Vec<(OutputId, Vec<u8>)> {
        let mut created_resources = Vec::new();
        match self.join_set.join_next().await {
            None => (),
            Some(res) => {
                let res = res.unwrap().unwrap();
                created_resources.push(res);

                loop {
                    match poll_fn(|cx| self.join_set.poll_join_next(cx)).now_or_never() {
                        None => break,
                        Some(None) => break,
                        Some(Some(res)) => {
                            let res = res.unwrap().unwrap();
                            created_resources.push(res);
                        }
                    }
                }
            }
        }

        created_resources
    }
    pub async fn create_root_stack(
        monitor: ResourceMonitorClient<Channel>,
        engine: EngineClient<Channel>,
        pulumi_project: String,
        pulumi_stack: String,
    ) -> std::result::Result<(), Error> {
        let request = RegisterResourceRequest {
            r#type: "pulumi:pulumi:Stack".to_string(),
            name: format!("{}-{}", pulumi_project, pulumi_stack),
            custom: false,
            ..Default::default()
        };

        let result = Self::register_async(request, monitor)
            .await
            .context("Failed to register root stack")?;

        let urn = RegisterResourceResponse::decode(&mut result.as_slice())
            .context("Failed to decode register resource response")?
            .urn;
        Self::set_root_resource_async(urn, engine)
            .await
            .context("Failed to set root resource")?;

        Ok(())
    }

    async fn send_register_resource_request_inner(
        output_id: OutputId,
        request: RegisterResourceRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<(OutputId, Vec<u8>)> {
        let result = monitor.register_resource(request).await?;
        Ok((output_id, result.get_ref().encode_to_vec()))
    }
    async fn send_resource_invoke_request_inner(
        output_id: OutputId,
        request: ResourceInvokeRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<(OutputId, Vec<u8>)> {
        let result = monitor.invoke(request).await?;
        Ok((output_id, result.get_ref().encode_to_vec()))
    }

    async fn register_async(
        request: RegisterResourceRequest,
        mut monitor: ResourceMonitorClient<Channel>,
    ) -> Result<Vec<u8>> {
        let result = monitor
            .register_resource(request)
            .await
            .context("Failed to register resource")?;

        Ok(result.get_ref().encode_to_vec())
    }

    async fn set_root_resource_async(urn: String, mut engine: EngineClient<Channel>) -> Result<()> {
        let request = SetRootResourceRequest { urn };

        let _ = engine
            .set_root_resource(request)
            .await
            .context("Failed to set root resource")?;

        Ok(())
    }

    async fn get_root_resource_async(engine_url: String) -> Result<String> {
        let mut client = EngineClient::connect(format!("tcp://{engine_url}")).await?;

        let request = GetRootResourceRequest {};

        let result = client.get_root_resource(request).await?;

        Ok(result.get_ref().urn.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::output_id::OutputId;
    use crate::pulumi_state::PulumiState;
    use crate::test_server::{MyResourceEngineServer, MyResourceMonitorServer};
    use pulumi_wasm_proto::grpc::engine_server::EngineServer;
    use pulumi_wasm_proto::grpc::resource_monitor_server::ResourceMonitorServer;
    use pulumi_wasm_proto::grpc::RegisterResourceRequest;

    use std::time::Instant;
    use tokio::net::TcpListener;
    use tonic::codegen::tokio_stream;
    use tonic::transport::Server;

    #[tokio::test]
    async fn test() -> Result<(), anyhow::Error> {
        let monitor_listener = TcpListener::bind("127.0.0.1:0").await?;
        let engine_listener = TcpListener::bind("127.0.0.1:0").await?;
        let monitor_addr = monitor_listener.local_addr()?;
        let engine_addr = engine_listener.local_addr()?;

        let monitor_server = Server::builder()
            .add_service(ResourceMonitorServer::new(MyResourceMonitorServer {}))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(
                monitor_listener,
            ));

        let engine_server = Server::builder()
            .add_service(EngineServer::new(MyResourceEngineServer {}))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(
                engine_listener,
            ));

        tokio::spawn(monitor_server);
        tokio::spawn(engine_server);

        let mut pulumi_state = PulumiState::new(
            monitor_addr.to_string(),
            engine_addr.to_string(),
            "project".to_string(),
            "stack".to_string(),
        )
        .await?;

        let output_id_1 = OutputId::new("1".into());
        let output_id_2 = OutputId::new("2".into());
        let output_id_3 = OutputId::new("3".into());

        pulumi_state.send_register_resource_request(output_id_1, create_request("test1"));
        pulumi_state.send_register_resource_request(output_id_2, create_request("test2"));
        pulumi_state.send_register_resource_request(output_id_3, create_request("test3"));

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 2);
        assert!(start.elapsed().as_secs() <= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 1);
        assert!(start.elapsed().as_secs() <= 3);
        assert!(start.elapsed().as_secs() >= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 0);
        assert!(start.elapsed().as_secs() <= 1);

        Ok(())
    }

    fn create_request(name: &str) -> RegisterResourceRequest {
        RegisterResourceRequest {
            r#type: "".to_string(),
            name: name.into(),
            parent: "".to_string(),
            custom: false,
            object: None,
            protect: false,
            dependencies: vec![],
            provider: "".to_string(),
            property_dependencies: Default::default(),
            delete_before_replace: false,
            version: "".to_string(),
            ignore_changes: vec![],
            accept_secrets: false,
            additional_secret_outputs: vec![],
            alias_ur_ns: vec![],
            import_id: "".to_string(),
            custom_timeouts: None,
            delete_before_replace_defined: false,
            supports_partial_values: false,
            remote: false,
            accept_resources: false,
            providers: Default::default(),
            replace_on_changes: vec![],
            plugin_download_url: "".to_string(),
            plugin_checksums: Default::default(),
            retain_on_delete: false,
            aliases: vec![],
            deleted_with: "".to_string(),
            alias_specs: false,
            source_position: None,
        }
    }
}
