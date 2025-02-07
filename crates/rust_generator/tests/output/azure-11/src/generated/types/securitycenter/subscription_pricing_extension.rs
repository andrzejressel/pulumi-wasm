#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriptionPricingExtension {
    /// Key/Value pairs that are required for some extensions.
    /// 
    /// > **NOTE:** If an extension is not defined, it will not be enabled.
    /// 
    /// > **NOTE:** Changing the pricing tier to `Standard` affects all resources of the given type in the subscription and could be quite costly.
    #[builder(into, default)]
    #[serde(rename = "additionalExtensionProperties")]
    pub r#additional_extension_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of extension.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
