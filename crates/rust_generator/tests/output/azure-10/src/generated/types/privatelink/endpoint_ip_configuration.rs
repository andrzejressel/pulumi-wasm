#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointIpConfiguration {
    /// Specifies the member name this IP address applies to. If it is not specified, it will use the value of `subresource_name`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `member_name` will be required and will not take the value of `subresource_name` in the next major version.
    #[builder(into, default)]
    #[serde(rename = "memberName")]
    pub r#member_name: Box<Option<String>>,
    /// Specifies the Name of the IP Configuration. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the static IP address within the private endpoint's subnet to be used. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// Specifies the subresource this IP address applies to. `subresource_names` corresponds to `group_id`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "subresourceName")]
    pub r#subresource_name: Box<Option<String>>,
}
