#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupExposedPort {
    /// The port number the container will expose. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The network protocol associated with port. Possible values are `TCP` & `UDP`. Changing this forces a new resource to be created. Defaults to `TCP`.
    /// 
    /// > **Note:** Removing all `exposed_port` blocks requires setting `exposed_port = []`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}