#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointPrivateServiceConnection {
    /// Does the Private Endpoint require Manual Approval from the remote resource owner? Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** If you are trying to connect the Private Endpoint to a remote resource without having the correct RBAC permissions on the remote resource set this value to `true`.
    #[builder(into)]
    #[serde(rename = "isManualConnection")]
    pub r#is_manual_connection: Box<bool>,
    /// Specifies the Name of the Private Service Connection. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Service Alias of the Private Link Enabled Remote Resource which this Private Endpoint should be connected to. One of `private_connection_resource_id` or `private_connection_resource_alias` must be specified. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "privateConnectionResourceAlias")]
    pub r#private_connection_resource_alias: Box<Option<String>>,
    /// The ID of the Private Link Enabled Remote Resource which this Private Endpoint should be connected to. One of `private_connection_resource_id` or `private_connection_resource_alias` must be specified. Changing this forces a new resource to be created. For a web app or function app slot, the parent web app should be used in this field instead of a reference to the slot itself.
    #[builder(into, default)]
    #[serde(rename = "privateConnectionResourceId")]
    pub r#private_connection_resource_id: Box<Option<String>>,
    /// (Required) The static IP address set by this configuration. It is recommended to use the private IP address exported in the `private_service_connection` block to obtain the address associated with the private endpoint.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// A message passed to the owner of the remote resource when the private endpoint attempts to establish the connection to the remote resource. The provider allows a maximum request message length of `140` characters, however the request message maximum length is dependent on the service the private endpoint is connected to. Only valid if `is_manual_connection` is set to `true`.
    /// 
    /// > **NOTE:** When connected to an SQL resource the `request_message` maximum length is `128`.
    #[builder(into, default)]
    #[serde(rename = "requestMessage")]
    pub r#request_message: Box<Option<String>>,
    /// A list of subresource names which the Private Endpoint is able to connect to. `subresource_names` corresponds to `group_id`. Possible values are detailed in the product [documentation](https://docs.microsoft.com/azure/private-link/private-endpoint-overview#private-link-resource) in the `Subresources` column. Changing this forces a new resource to be created. 
    /// 
    /// > **NOTE:** Some resource types (such as Storage Account) only support 1 subresource per private endpoint.
    /// 
    /// > **NOTE:** For most Private Links one or more `subresource_names` will need to be specified, please see the linked documentation for details.
    #[builder(into, default)]
    #[serde(rename = "subresourceNames")]
    pub r#subresource_names: Box<Option<Vec<String>>>,
}
