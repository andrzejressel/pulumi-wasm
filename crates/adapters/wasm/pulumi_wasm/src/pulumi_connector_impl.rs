use crate::bindings::component::pulumi_wasm_external::external_world;
use crate::bindings::component::pulumi_wasm_external::external_world::{
    RegisterResourceRequest, ResourceInvokeRequest,
};
use prost::Message;
use pulumi_gestalt_core_core::PulumiConnector;
use pulumi_gestalt_core_proto::grpc::{
    RegisterResourceOutputsRequest as GrpcRegisterResourceOutputsRequest,
    RegisterResourceRequest as GrpcRegisterResourceRequest,
    ResourceInvokeRequest as GrpcResourceInvokeRequest,
};

pub(crate) struct PulumiConnectorImpl;

impl PulumiConnector for PulumiConnectorImpl {
    fn resource_invoke(&self, output_id: String, req: GrpcResourceInvokeRequest) {
        external_world::resource_invoke(&ResourceInvokeRequest {
            output_id,
            body: req.encode_to_vec(),
        });
    }
    fn register_resource(&self, output_id: String, req: GrpcRegisterResourceRequest) {
        external_world::register_resource(&RegisterResourceRequest {
            output_id,
            body: req.encode_to_vec(),
        });
    }

    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)> {
        let registered_resources = external_world::wait_for_resource_operations();
        registered_resources
            .into_iter()
            .map(|r| (r.output_id, r.body))
            .collect()
    }

    fn register_outputs(&self, req: GrpcRegisterResourceOutputsRequest) {
        external_world::register_resource_outputs(&req.encode_to_vec());
    }
}
