#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BotAssociationLexBot {
    /// The Region that the Amazon Lex (V1) bot was created in. Defaults to current region.
    #[builder(into, default)]
    #[serde(rename = "lexRegion")]
    pub r#lex_region: Box<Option<String>>,
    /// The name of the Amazon Lex (V1) bot.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
