use pulumi_gestalt_core::PulumiConnector;
use pulumi_gestalt_grpc_connection::sync_pulumi_state::PulumiStateSync;
use pulumi_gestalt_proto::grpc::{
    RegisterResourceOutputsRequest, RegisterResourceRequest, ResourceInvokeRequest,
};
use std::sync::RwLock;

pub(crate) struct NativePulumiConnector(RwLock<PulumiStateSync>);

impl NativePulumiConnector {
    pub(crate) fn new(
        monitor_url: String,
        engine_url: String,
        pulumi_project: String,
        pulumi_stack: String,
    ) -> Self {
        let pulumi_state =
            PulumiStateSync::new(monitor_url, engine_url, pulumi_project, pulumi_stack);
        NativePulumiConnector(RwLock::new(pulumi_state))
    }
}

impl PulumiConnector for NativePulumiConnector {
    fn resource_invoke(&self, output_id: String, req: ResourceInvokeRequest) {
        let mut state = self.0.write().unwrap();
        state.send_resource_invoke_request(output_id.into(), req);
    }
    fn register_resource(&self, output_id: String, req: RegisterResourceRequest) {
        let mut state = self.0.write().unwrap();
        state.send_register_resource_request(output_id.into(), req);
    }

    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)> {
        let mut state = self.0.write().unwrap();
        state
            .get_created_resources()
            .iter()
            .map(|(output_id, body)| (output_id.0.clone(), body.clone()))
            .collect()
    }

    fn register_outputs(&self, req: RegisterResourceOutputsRequest) {
        let mut state = self.0.write().unwrap();
        state.register_resource_outputs(req);
    }
}
