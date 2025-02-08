#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ElasticSanSku {
    /// The SKU name. Possible values are `Premium_LRS` and `Premium_ZRS`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE** `Premium_ZRS` SKU is only available in limited Azure regions including `France Central`, `North Europe`, `West Europe`, and `West US 2`. Please refer to this [document](https://azure.microsoft.com/updates/regional-expansion-azure-elastic-san-public-preview-is-now-available-in-more-regions) for more details.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The SKU tier. The only possible value is `Premium`. Defaults to `Premium`.
    #[builder(into, default)]
    #[serde(rename = "tier")]
    pub r#tier: Box<Option<String>>,
}
