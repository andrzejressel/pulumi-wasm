#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeploymentSku {
    /// Tokens-per-Minute (TPM). The unit of measure for this field is in the thousands of Tokens-per-Minute. Defaults to `1` which means that the limitation is `1000` tokens per minute. If the resources SKU supports scale in/out then the capacity field should be included in the resources' configuration. If the scale in/out is not supported by the resources SKU then this field can be safely omitted. For more information about TPM please see the [product documentation](https://learn.microsoft.com/azure/ai-services/openai/how-to/quota?tabs=rest).
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<i32>>,
    /// If the service has different generations of hardware, for the same SKU, then that can be captured here. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "family")]
    pub r#family: Box<Option<String>>,
    /// The name of the SKU. Possible values include `Standard`, `DataZoneStandard`, `GlobalBatch`, `GlobalStandard` and `ProvisionedManaged`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<String>>,
    /// Possible values are `Free`, `Basic`, `Standard`, `Premium`, `Enterprise`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "tier")]
    pub r#tier: Box<Option<String>>,
}
