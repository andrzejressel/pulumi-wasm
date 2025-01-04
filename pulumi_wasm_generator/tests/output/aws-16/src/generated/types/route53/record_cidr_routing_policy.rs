#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecordCidrRoutingPolicy {
    /// The CIDR collection ID. See the `aws.route53.CidrCollection` resource for more details.
    #[builder(into)]
    #[serde(rename = "collectionId")]
    pub r#collection_id: Box<String>,
    /// The CIDR collection location name. See the `aws.route53.CidrLocation` resource for more details. A `location_name` with an asterisk `"*"` can be used to create a default CIDR record. `collection_id` is still required for default record.
    #[builder(into)]
    #[serde(rename = "locationName")]
    pub r#location_name: Box<String>,
}
