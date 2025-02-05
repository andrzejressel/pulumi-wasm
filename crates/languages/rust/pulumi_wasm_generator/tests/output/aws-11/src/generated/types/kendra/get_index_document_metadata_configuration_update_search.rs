#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIndexDocumentMetadataConfigurationUpdateSearch {
    /// Determines whether the field is returned in the query response. The default is `true`.
    #[builder(into)]
    #[serde(rename = "displayable")]
    pub r#displayable: Box<bool>,
    /// Whether the field can be used to create search facets, a count of results for each value in the field. The default is `false`.
    #[builder(into)]
    #[serde(rename = "facetable")]
    pub r#facetable: Box<bool>,
    /// Determines whether the field is used in the search. If the Searchable field is true, you can use relevance tuning to manually tune how Amazon Kendra weights the field in the search. The default is `true` for `string` fields and `false` for `number` and `date` fields.
    #[builder(into)]
    #[serde(rename = "searchable")]
    pub r#searchable: Box<bool>,
    /// Determines whether the field can be used to sort the results of a query. If you specify sorting on a field that does not have Sortable set to true, Amazon Kendra returns an exception. The default is `false`.
    #[builder(into)]
    #[serde(rename = "sortable")]
    pub r#sortable: Box<bool>,
}
