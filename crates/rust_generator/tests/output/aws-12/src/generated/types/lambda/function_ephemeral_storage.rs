#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionEphemeralStorage {
    /// The size of the Lambda function Ephemeral storage(`/tmp`) represented in MB. The minimum supported `ephemeral_storage` value defaults to `512`MB and the maximum supported value is `10240`MB.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<i32>>,
}
