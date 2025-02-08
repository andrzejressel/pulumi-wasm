#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSharesDataShare {
    /// ARN (Amazon Resource Name) of the data share.
    #[builder(into)]
    #[serde(rename = "dataShareArn")]
    pub r#data_share_arn: Box<String>,
    /// Identifier of a datashare to show its managing entity.
    #[builder(into)]
    #[serde(rename = "managedBy")]
    pub r#managed_by: Box<String>,
    /// ARN (Amazon Resource Name) of the producer.
    #[builder(into)]
    #[serde(rename = "producerArn")]
    pub r#producer_arn: Box<String>,
}
