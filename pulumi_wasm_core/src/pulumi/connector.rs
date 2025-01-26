pub trait PulumiConnector {
    fn resource_invoke(&self, output_id: String, req: Vec<u8>);
    fn register_resource(&self, output_id: String, req: Vec<u8>);
    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)>;
    fn register_outputs(&self, req: Vec<u8>);
}
