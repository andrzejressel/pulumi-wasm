use crate::output_id::OutputId;
use crate::pulumi_state::PulumiState;
use pulumi_gestalt_proto::mini::pulumirpc::ResourceInvokeRequest;
use pulumi_gestalt_proto::mini::pulumirpc::{
    RegisterResourceOutputsRequest, RegisterResourceRequest,
};
use tokio::runtime::{Builder, Runtime};

pub struct PulumiStateSync {
    pulumi_state: PulumiState,
    runtime: Runtime,
}

impl PulumiStateSync {
    pub fn new(
        monitor_url: String,
        engine_url: String,
        pulumi_project: String,
        pulumi_stack: String,
    ) -> Self {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let pulumi_state = runtime
            .block_on(PulumiState::new(
                monitor_url,
                engine_url,
                pulumi_project,
                pulumi_stack,
            ))
            .unwrap();
        Self {
            pulumi_state,
            runtime,
        }
    }

    pub fn send_register_resource_request(
        &mut self,
        output_id: OutputId,
        request: RegisterResourceRequest,
    ) {
        let _guard = self.runtime.handle().enter();
        self.pulumi_state
            .send_register_resource_request(output_id, request);
    }

    pub fn send_resource_invoke_request(
        &mut self,
        output_id: OutputId,
        request: ResourceInvokeRequest,
    ) {
        let _guard = self.runtime.handle().enter();
        self.pulumi_state
            .send_resource_invoke_request(output_id, request);
    }

    pub fn register_resource_outputs(&mut self, request: RegisterResourceOutputsRequest) {
        let _guard = self.runtime.handle().enter();
        self.runtime
            .block_on(self.pulumi_state.register_resource_outputs(request))
            .unwrap();
    }

    pub fn get_created_resources(&mut self) -> Vec<(OutputId, Vec<u8>)> {
        let _guard = self.runtime.handle().enter();
        self.runtime
            .block_on(self.pulumi_state.get_created_resources())
    }
}

#[cfg(test)]
mod tests {
    use crate::output_id::OutputId;
    use pulumi_gestalt_proto::full::pulumirpc::engine_server::EngineServer;
    use std::time::Instant;
    use tokio::net::TcpListener;

    use tonic::codegen::tokio_stream;
    use tonic::transport::Server;

    use crate::sync_pulumi_state::PulumiStateSync;
    use crate::test_server::{MyResourceEngineServer, MyResourceMonitorServer};
    use pulumi_gestalt_proto::full::pulumirpc::resource_monitor_server::ResourceMonitorServer;
    use pulumi_gestalt_proto::full::pulumirpc::RegisterResourceRequest;
    use pulumi_gestalt_proto::IntoMini;

    #[test]
    fn test() -> Result<(), anyhow::Error> {
        let runtime = tokio::runtime::Runtime::new()?;

        let monitor_listener = runtime.block_on(TcpListener::bind("127.0.0.1:0"))?;
        let engine_listener = runtime.block_on(TcpListener::bind("127.0.0.1:0"))?;
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

        runtime.spawn(monitor_server);
        runtime.spawn(engine_server);

        let mut pulumi_state = PulumiStateSync::new(
            monitor_addr.to_string(),
            engine_addr.to_string(),
            "project".to_string(),
            "stack".to_string(),
        );

        let output_id_1 = OutputId::new("1".into());
        let output_id_2 = OutputId::new("2".into());
        let output_id_3 = OutputId::new("3".into());

        pulumi_state
            .send_register_resource_request(output_id_1, create_request("test1").into_mini());
        pulumi_state
            .send_register_resource_request(output_id_2, create_request("test2").into_mini());
        pulumi_state
            .send_register_resource_request(output_id_3, create_request("test3").into_mini());

        std::thread::sleep(std::time::Duration::from_secs(2));

        let start = Instant::now();
        let result = pulumi_state.get_created_resources();
        assert_eq!(result.len(), 2);
        assert!(start.elapsed().as_secs() <= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources();
        assert_eq!(result.len(), 1);
        assert!(start.elapsed().as_secs() <= 3);
        assert!(start.elapsed().as_secs() >= 1);

        let start = Instant::now();
        let result = pulumi_state.get_created_resources();
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
            transforms: vec![],
            supports_result_reporting: false,
            package_ref: "".to_string(),
        }
    }
}
