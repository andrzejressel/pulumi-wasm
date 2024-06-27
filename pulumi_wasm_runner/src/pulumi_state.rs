use futures::future::poll_fn;
use futures::FutureExt;
use prost::Message;
use tokio::task::JoinSet;

use crate::grpc::RegisterResourceRequest;
use crate::grpc::resource_monitor_client::ResourceMonitorClient;
use crate::model::OutputId;

pub(crate) struct PulumiState {
    engine_url: String,
    join_set: JoinSet<anyhow::Result<(OutputId, Vec<u8>)>>,
    // resource_monitor_client: Arc<ResourceMonitorClient<Channel>>,
}

impl PulumiState {
    pub(crate) fn new(engine_url: String) -> Self {
        Self {
            engine_url,
            join_set: JoinSet::new()
        }
    }

    pub(crate) fn send_request(&mut self, output_id: OutputId, request: RegisterResourceRequest) {
        let engine_url = self.engine_url.clone();
        self.join_set.spawn(async move {
            Self::send_request_inner(output_id, request, engine_url).await
        });
    }

    pub(crate) async fn get_created_resources(&mut self) -> Vec<(OutputId, Vec<u8>)> {
        if self.join_set.is_empty() {
            return Vec::new();
        }

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
                };
            }
        }

        created_resources
    }

    async fn send_request_inner(output_id: OutputId, request: RegisterResourceRequest, engine_url: String) -> anyhow::Result<(OutputId, Vec<u8>)> {
        let mut resource_monitor_client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;
        let result = resource_monitor_client.register_resource(request).await?;
        Ok((output_id, result.get_ref().encode_to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use tonic::{Request, Response, Status};
    use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
    use tonic::transport::Server;

    use crate::grpc::{CallRequest, CallResponse, InvokeResponse, ReadResourceRequest, ReadResourceResponse, RegisterResourceOutputsRequest, RegisterResourceRequest, RegisterResourceResponse, ResourceInvokeRequest, SupportsFeatureRequest, SupportsFeatureResponse};
    use crate::grpc::resource_monitor_server::{ResourceMonitor, ResourceMonitorServer};
    use crate::model::OutputId;
    use crate::pulumi_state::PulumiState;

    struct MyServer {}

    #[tonic::async_trait]
    impl ResourceMonitor for MyServer {
        async fn supports_feature(&self, request: Request<SupportsFeatureRequest>) -> Result<Response<SupportsFeatureResponse>, Status> {
            unimplemented!()
        }

        async fn invoke(&self, request: Request<ResourceInvokeRequest>) -> Result<Response<InvokeResponse>, Status> {
            unimplemented!()
        }

        type StreamInvokeStream = ReceiverStream<Result<InvokeResponse, Status>>;

        async fn stream_invoke(&self, request: Request<ResourceInvokeRequest>) -> Result<Response<Self::StreamInvokeStream>, Status> {
            unimplemented!()
        }

        async fn call(&self, request: Request<CallRequest>) -> Result<Response<CallResponse>, Status> {
            unimplemented!()
        }

        async fn read_resource(&self, request: Request<ReadResourceRequest>) -> Result<Response<ReadResourceResponse>, Status> {
            unimplemented!()
        }

        async fn register_resource(&self, request: Request<RegisterResourceRequest>) -> Result<Response<RegisterResourceResponse>, Status> {
            let request = request.into_inner();
            match request.name.as_str() {
                "test1" => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let mut response = RegisterResourceResponse::default();
                    response.id = "1".to_string();
                    Ok(Response::new(response))
                }
                "test2" => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let mut response = RegisterResourceResponse::default();
                    response.id = "2".to_string();
                    Ok(Response::new(response))
                }
                "test3" => {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    let mut response = RegisterResourceResponse::default();
                    response.id = "2".to_string();
                    Ok(Response::new(response))
                }
                _ => return Err(Status::aborted(format!("unknown resource name: {}", request.name)))
            }
        }

        async fn register_resource_outputs(&self, request: Request<RegisterResourceOutputsRequest>) -> Result<Response<()>, Status> {
            unimplemented!()
        }

    }

    #[tokio::test]
    async fn test() -> Result<(), anyhow::Error> {

        let addr = "127.0.0.1:50051".parse()?;

        let server = Server::builder()
            .add_service(ResourceMonitorServer::new(MyServer{}))
            .serve(addr);

        tokio::spawn(server);

        let mut pulumi_state = PulumiState::new(addr.to_string());

        let output_id_1 = OutputId::new("1".into());
        let output_id_2 = OutputId::new("2".into());
        let output_id_3 = OutputId::new("3".into());

        pulumi_state.send_request(output_id_1, create_request("test1"));
        pulumi_state.send_request(output_id_2, create_request("test2"));
        pulumi_state.send_request(output_id_3, create_request("test3"));

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 2);

        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 1);

        let result = pulumi_state.get_created_resources().await;
        assert_eq!(result.len(), 0);

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