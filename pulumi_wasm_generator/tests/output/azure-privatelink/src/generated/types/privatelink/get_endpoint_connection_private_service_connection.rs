#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEndpointConnectionPrivateServiceConnection {
    /// Specifies the Name of the private endpoint.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The private IP address associated with the private endpoint, note that you will have a private IP address assigned to the private endpoint even if the connection request was `Rejected`.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// Possible values are as follows:
    /// Value | Meaning
    /// -- | --
    /// `Auto-Approved` | The remote resource owner has added you to the `Auto-Approved` RBAC permission list for the remote resource, all private endpoint connection requests will be automatically `Approved`.
    /// `Deleted state` | The resource owner has `Rejected` the private endpoint connection request and has removed your private endpoint request from the remote resource.
    /// `request/response message` | If you submitted a manual private endpoint connection request, while in the `Pending` status the `request_response` will display the same text from your `request_message` in the `private_service_connection` block above. If the private endpoint connection request was `Rejected` by the owner of the remote resource, the text for the rejection will be displayed as the `request_response` text, if the private endpoint connection request was `Approved` by the owner of the remote resource, the text for the approval will be displayed as the `request_response` text
    #[builder(into)]
    #[serde(rename = "requestResponse")]
    pub r#request_response: Box<String>,
    /// The current status of the private endpoint request, possible values will be `Pending`, `Approved`, `Rejected`, or `Disconnected`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
