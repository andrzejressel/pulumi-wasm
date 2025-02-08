#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct Powershell72ModuleModuleLink {
    /// A `hash` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "hash")]
    pub r#hash: Box<Option<super::super::types::automation::Powershell72ModuleModuleLinkHash>>,
    /// The URI of the module content (zip or nupkg).
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
