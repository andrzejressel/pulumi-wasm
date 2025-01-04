#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActionGroupItsmReceiver {
    /// The unique connection identifier of the ITSM connection.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Box<String>,
    /// The name of the ITSM receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The region of the workspace.
    /// 
    /// > **NOTE** `ticket_configuration` should be JSON blob with `PayloadRevision` and `WorkItemType` keys (e.g., `ticket_configuration="{\"PayloadRevision\":0,\"WorkItemType\":\"Incident\"}"`), and `ticket_configuration="{}"` will return an error, see more at this [REST API issue](https://github.com/Azure/azure-rest-api-specs/issues/20488)
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// A JSON blob for the configurations of the ITSM action. CreateMultipleWorkItems option will be part of this blob as well.
    #[builder(into)]
    #[serde(rename = "ticketConfiguration")]
    pub r#ticket_configuration: Box<String>,
    /// The Azure Log Analytics workspace ID where this connection is defined. Format is `<subscription id>|<workspace id>`, for example `00000000-0000-0000-0000-000000000000|00000000-0000-0000-0000-000000000000`.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Box<String>,
}
