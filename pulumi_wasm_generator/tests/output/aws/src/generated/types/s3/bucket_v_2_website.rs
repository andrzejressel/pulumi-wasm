#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2Website {
    /// Absolute path to the document to return in case of a 4XX error.
    #[builder(into, default)]
    #[serde(rename = "errorDocument")]
    pub r#error_document: Box<Option<String>>,
    /// Amazon S3 returns this index document when requests are made to the root domain or any of the subfolders.
    #[builder(into, default)]
    #[serde(rename = "indexDocument")]
    pub r#index_document: Box<Option<String>>,
    /// Hostname to redirect all website requests for this bucket to. Hostname can optionally be prefixed with a protocol (`http://` or `https://`) to use when redirecting requests. The default is the protocol that is used in the original request.
    #[builder(into, default)]
    #[serde(rename = "redirectAllRequestsTo")]
    pub r#redirect_all_requests_to: Box<Option<String>>,
    /// JSON array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
    /// describing redirect behavior and when redirects are applied.
    #[builder(into, default)]
    #[serde(rename = "routingRules")]
    pub r#routing_rules: Box<Option<String>>,
}