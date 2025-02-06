#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthorizedViewSubsetView {
    /// A group of column family subsets to be included in the authorized view. This can be specified multiple times. Structure is documented below.
    /// 
    /// -----
    #[builder(into, default)]
    #[serde(rename = "familySubsets")]
    pub r#family_subsets: Box<Option<Vec<super::super::types::bigtable::AuthorizedViewSubsetViewFamilySubset>>>,
    /// A list of Base64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix ("").
    #[builder(into, default)]
    #[serde(rename = "rowPrefixes")]
    pub r#row_prefixes: Box<Option<Vec<String>>>,
}
