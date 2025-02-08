#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassifierCsvClassifier {
    /// Enables the processing of files that contain only one column.
    #[builder(into, default)]
    #[serde(rename = "allowSingleColumn")]
    pub r#allow_single_column: Box<Option<bool>>,
    /// Indicates whether the CSV file contains a header. This can be one of "ABSENT", "PRESENT", or "UNKNOWN".
    #[builder(into, default)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: Box<Option<String>>,
    /// Enables the custom datatype to be configured.
    #[builder(into, default)]
    #[serde(rename = "customDatatypeConfigured")]
    pub r#custom_datatype_configured: Box<Option<bool>>,
    /// A list of supported custom datatypes. Valid values are `BINARY`, `BOOLEAN`, `DATE`, `DECIMAL`, `DOUBLE`, `FLOAT`, `INT`, `LONG`, `SHORT`, `STRING`, `TIMESTAMP`.
    #[builder(into, default)]
    #[serde(rename = "customDatatypes")]
    pub r#custom_datatypes: Box<Option<Vec<String>>>,
    /// The delimiter used in the CSV to separate columns.
    #[builder(into, default)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<Option<String>>,
    /// Specifies whether to trim column values.
    #[builder(into, default)]
    #[serde(rename = "disableValueTrimming")]
    pub r#disable_value_trimming: Box<Option<bool>>,
    /// A list of strings representing column names.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    /// A custom symbol to denote what combines content into a single column value. It must be different from the column delimiter.
    #[builder(into, default)]
    #[serde(rename = "quoteSymbol")]
    pub r#quote_symbol: Box<Option<String>>,
    /// The SerDe for processing CSV. Valid values are `OpenCSVSerDe`, `LazySimpleSerDe`, `None`.
    #[builder(into, default)]
    #[serde(rename = "serde")]
    pub r#serde: Box<Option<String>>,
}
