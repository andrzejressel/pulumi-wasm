#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorizedViewSubsetViewFamilySubset {
    /// Name of the column family to be included in the authorized view. The specified column family must exist in the parent table of this authorized view.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: Box<String>,
    /// A list of Base64-encoded prefixes for qualifiers of the column family to be included in the authorized view.
    /// Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix ("").
    #[builder(into, default)]
    #[serde(rename = "qualifierPrefixes")]
    pub r#qualifier_prefixes: Box<Option<Vec<String>>>,
    /// A list of Base64-encoded individual exact column qualifiers of the column family to be included in the authorized view.
    #[builder(into, default)]
    #[serde(rename = "qualifiers")]
    pub r#qualifiers: Box<Option<Vec<String>>>,
}
