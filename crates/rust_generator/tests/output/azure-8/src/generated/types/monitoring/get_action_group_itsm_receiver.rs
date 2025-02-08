#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetActionGroupItsmReceiver {
    /// The unique connection identifier of the ITSM connection.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<String>,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The region of the workspace.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// A JSON blob for the configurations of the ITSM action. CreateMultipleWorkItems option will be part of this blob as well.
    #[builder(into)]
    #[serde(rename = "ticketConfiguration")]
    pub r#ticket_configuration: Box<String>,
    /// The Azure Log Analytics workspace ID where this connection is defined.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Box<String>,
}
