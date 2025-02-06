#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldIndexConfig {
    /// The indexes to configure on the field. Order or array contains must be specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "indexes")]
    pub r#indexes: Box<Option<Vec<super::super::types::firestore::FieldIndexConfigIndex>>>,
}
