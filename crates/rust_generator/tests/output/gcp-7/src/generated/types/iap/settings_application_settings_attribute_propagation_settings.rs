#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SettingsApplicationSettingsAttributePropagationSettings {
    /// Whether the provided attribute propagation settings should be evaluated on user requests.
    /// If set to true, attributes returned from the expression will be propagated in the set output credentials.
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
    /// Raw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can
    /// be selected. Expressions can select different attribute types from attributes:
    /// attributes.saml_attributes, attributes.iap_attributes.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Which output credentials attributes selected by the CEL expression should be propagated in.
    /// All attributes will be fully duplicated in each selected output credential.
    /// Possible values are:
    /// * `HEADER`: Propagate attributes in the headers with "x-goog-iap-attr-" prefix.
    /// * `JWT`: Propagate attributes in the JWT of the form:
    /// "additional_claims": { "my_attribute": ["value1", "value2"] }
    /// * `RCTOKEN`: Propagate attributes in the RCToken of the form: "
    /// additional_claims": { "my_attribute": ["value1", "value2"] }
    /// Each value may be one of: `HEADER`, `JWT`, `RCTOKEN`.
    #[builder(into, default)]
    #[serde(rename = "outputCredentials")]
    pub r#output_credentials: Box<Option<Vec<String>>>,
}
