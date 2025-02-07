#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTagKeysKey {
    /// Creation time.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// User-assigned description of the TagKey.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// an identifier for the resource with format `tagKeys/{{name}}`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Namespaced name of the TagKey which is in the format `{parentNamespace}/{shortName}`.
    #[builder(into)]
    #[serde(rename = "namespacedName")]
    pub r#namespaced_name: Box<String>,
    /// The resource name of the parent organization or project. It can be in format `organizations/{org_id}` or `projects/{project_id_or_number}`.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: Box<String>,
    /// A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. A purpose does not grant a policy engine exclusive rights to the Tag, and it may be referenced by other policy engines.
    #[builder(into)]
    #[serde(rename = "purpose")]
    pub r#purpose: Box<String>,
    /// Purpose data corresponds to the policy system that the tag is intended for. See documentation for Purpose for formatting of this field.
    #[builder(into)]
    #[serde(rename = "purposeData")]
    pub r#purpose_data: Box<std::collections::HashMap<String, String>>,
    /// The user friendly name for a TagKey. The short name should be unique for TagKeys wihting the same tag namespace.
    #[builder(into)]
    #[serde(rename = "shortName")]
    pub r#short_name: Box<String>,
    /// Update time.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}
