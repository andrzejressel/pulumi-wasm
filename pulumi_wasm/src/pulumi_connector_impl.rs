use crate::bindings::component::pulumi_wasm::external_world;
use crate::bindings::component::pulumi_wasm::external_world::RegisterResourceV2Request;
use pulumi_wasm_core::PulumiConnector;

pub(crate) struct PulumiConnectorImpl {}

impl PulumiConnector for PulumiConnectorImpl {
    fn is_in_preview(&self) -> bool {
        external_world::is_in_preview()
    }

    fn create_resource(&self, output_id: String, req: Vec<u8>) {
        external_world::register_resource_v2(&RegisterResourceV2Request {
            output_id,
            body: req,
        });
    }

    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)> {
        let registered_resources = external_world::wait_for_registered_resources();
        registered_resources
            .into_iter()
            .map(|r| (r.output_id, r.body))
            .collect()
    }
}
