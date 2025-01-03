#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassifierXmlClassifier {
    /// An identifier of the data format that the classifier matches.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: Box<String>,
    /// The XML tag designating the element that contains each record in an XML document being parsed. Note that this cannot identify a self-closing element (closed by `/>`). An empty row element that contains only attributes can be parsed as long as it ends with a closing tag (for example, `<row item_a="A" item_b="B"></row>` is okay, but `<row item_a="A" item_b="B" />` is not).
    #[builder(into)]
    #[serde(rename = "rowTag")]
    pub r#row_tag: Box<String>,
}
