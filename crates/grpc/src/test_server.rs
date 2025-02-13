use pulumi_gestalt_proto::full::pulumirpc::engine_server::Engine;
use pulumi_gestalt_proto::full::pulumirpc::resource_monitor_server::ResourceMonitor;
use pulumi_gestalt_proto::full::pulumirpc::{
    CallResponse, Callback, GetRootResourceRequest, GetRootResourceResponse, InvokeResponse,
    LogRequest, ReadResourceRequest, ReadResourceResponse, RegisterPackageRequest,
    RegisterPackageResponse, RegisterResourceOutputsRequest, RegisterResourceRequest,
    RegisterResourceResponse, ResourceCallRequest, ResourceInvokeRequest, SetRootResourceRequest,
    SetRootResourceResponse, StartDebuggingRequest, SupportsFeatureRequest,
    SupportsFeatureResponse,
};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub(crate) struct MyResourceMonitorServer {}
pub(crate) struct MyResourceEngineServer {}

#[tonic::async_trait]
impl ResourceMonitor for MyResourceMonitorServer {
    async fn supports_feature(
        &self,
        _request: Request<SupportsFeatureRequest>,
    ) -> Result<Response<SupportsFeatureResponse>, Status> {
        unimplemented!("supports_feature")
    }

    async fn invoke(
        &self,
        _request: Request<ResourceInvokeRequest>,
    ) -> Result<Response<InvokeResponse>, Status> {
        unimplemented!("invoke")
    }

    type StreamInvokeStream = ReceiverStream<Result<InvokeResponse, Status>>;

    async fn stream_invoke(
        &self,
        _request: Request<ResourceInvokeRequest>,
    ) -> Result<Response<Self::StreamInvokeStream>, Status> {
        unimplemented!("stream_invoke")
    }

    async fn call(
        &self,
        _request: Request<ResourceCallRequest>,
    ) -> Result<Response<CallResponse>, Status> {
        unimplemented!("call")
    }

    async fn read_resource(
        &self,
        _request: Request<ReadResourceRequest>,
    ) -> Result<Response<ReadResourceResponse>, Status> {
        unimplemented!("read_resource")
    }

    async fn register_resource(
        &self,
        request: Request<RegisterResourceRequest>,
    ) -> Result<Response<RegisterResourceResponse>, Status> {
        let request = request.into_inner();
        match request.name.as_str() {
            "test1" => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let response = RegisterResourceResponse {
                    id: "1".to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            "test2" => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let response = RegisterResourceResponse {
                    id: "2".to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            "test3" => {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                let response = RegisterResourceResponse {
                    id: "3".to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            "project-stack" => {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                let response = RegisterResourceResponse {
                    id: "3".to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            _ => {
                return Err(Status::aborted(format!(
                    "unknown resource name: {}",
                    request.name
                )))
            }
        }
    }

    async fn register_resource_outputs(
        &self,
        _request: Request<RegisterResourceOutputsRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!("register_resource_outputs")
    }

    async fn register_stack_transform(
        &self,
        request: Request<Callback>,
    ) -> Result<Response<()>, Status> {
        unimplemented!("register_stack_transform")
    }

    async fn register_stack_invoke_transform(
        &self,
        request: Request<Callback>,
    ) -> Result<Response<()>, Status> {
        unimplemented!("register_stack_invoke_transform")
    }

    async fn register_package(
        &self,
        request: Request<RegisterPackageRequest>,
    ) -> Result<Response<RegisterPackageResponse>, Status> {
        unimplemented!("register_package")
    }
}

#[tonic::async_trait]
impl Engine for MyResourceEngineServer {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<()>, Status> {
        unimplemented!("log")
    }

    async fn get_root_resource(
        &self,
        request: Request<GetRootResourceRequest>,
    ) -> Result<Response<GetRootResourceResponse>, Status> {
        Ok(Response::new(GetRootResourceResponse {
            urn: "root".to_string(),
        }))
    }

    async fn set_root_resource(
        &self,
        request: Request<SetRootResourceRequest>,
    ) -> Result<Response<SetRootResourceResponse>, Status> {
        Ok(Response::new(SetRootResourceResponse {}))
    }

    async fn start_debugging(
        &self,
        request: Request<StartDebuggingRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!("start_debugging")
    }
}
