#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAmiProductCode {
    #[builder(into)]
    #[serde(rename = "productCodeId")]
    pub r#product_code_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "productCodeType")]
    pub r#product_code_type: Box<String>,
}
