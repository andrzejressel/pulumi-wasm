#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclAssociationConfigRequestBodyAppRunnerService {
    /// Specifies the maximum size of the web request body component that an associated Amazon App Runner services should send to AWS WAF for inspection. This applies to statements in the web ACL that inspect the body or JSON body. Valid values are `KB_16`, `KB_32`, `KB_48` and `KB_64`.
    #[builder(into)]
    #[serde(rename = "defaultSizeInspectionLimit")]
    pub r#default_size_inspection_limit: Box<String>,
}