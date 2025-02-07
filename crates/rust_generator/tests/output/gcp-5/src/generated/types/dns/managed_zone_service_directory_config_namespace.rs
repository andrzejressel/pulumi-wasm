#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedZoneServiceDirectoryConfigNamespace {
    /// The fully qualified or partial URL of the service directory namespace that should be
    /// associated with the zone. This should be formatted like
    /// `https://servicedirectory.googleapis.com/v1/projects/{project}/locations/{location}/namespaces/{namespace_id}`
    /// or simply `projects/{project}/locations/{location}/namespaces/{namespace_id}`
    /// Ignored for `public` visibility zones.
    #[builder(into)]
    #[serde(rename = "namespaceUrl")]
    pub r#namespace_url: Box<String>,
}
