use pulumi_gestalt_proto::grpc::{
    RegisterResourceOutputsRequest, RegisterResourceRequest, ResourceInvokeRequest,
};

pub trait PulumiConnector {
    fn resource_invoke(&self, output_id: String, req: ResourceInvokeRequest);
    fn register_resource(&self, output_id: String, req: RegisterResourceRequest);
    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)>;
    fn register_outputs(&self, req: RegisterResourceOutputsRequest);
}
