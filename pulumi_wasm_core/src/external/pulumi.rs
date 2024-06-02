use rmpv::Value;

trait Pulumi {
    fn is_in_preview(&self) -> bool;
    fn get_root_resource(&self) -> String;
    fn register_outputs(&self, outputs: Vec<(String, Value)>);
}
