#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceNetworkAccessControl {
    /// An array of prefix list IDs.
    #[builder(into)]
    #[serde(rename = "prefixListIds")]
    pub r#prefix_list_ids: Box<Vec<String>>,
    /// An array of Amazon VPC endpoint IDs for the workspace. The only VPC endpoints that can be specified here are interface VPC endpoints for Grafana workspaces (using the com.amazonaws.[region].grafana-workspace service endpoint). Other VPC endpoints will be ignored.
    #[builder(into)]
    #[serde(rename = "vpceIds")]
    pub r#vpce_ids: Box<Vec<String>>,
}
