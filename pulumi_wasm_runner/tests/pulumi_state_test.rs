use anyhow::anyhow;
use tonic::{IntoRequest, Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use crate::grpc::resource_monitor_server::{ResourceMonitor, ResourceMonitorServer};
use crate::grpc::{CallRequest, CallResponse, InvokeResponse, ReadResourceRequest, ReadResourceResponse, RegisterResourceOutputsRequest, RegisterResourceRequest, RegisterResourceResponse, ResourceInvokeRequest, SupportsFeatureRequest, SupportsFeatureResponse};

mod grpc {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    tonic::include_proto!("pulumirpc");
}

