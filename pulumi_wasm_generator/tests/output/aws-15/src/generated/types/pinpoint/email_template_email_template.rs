#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EmailTemplateEmailTemplate {
    /// JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.
    #[builder(into, default)]
    #[serde(rename = "defaultSubstitutions")]
    pub r#default_substitutions: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplateHeader>>>,
    /// The message body, in HTML format, to use in email messages that are based on the message template. We recommend using HTML format for email clients that render HTML content. You can include links, formatted text, and more in an HTML message.
    #[builder(into, default)]
    #[serde(rename = "htmlPart")]
    pub r#html_part: Box<Option<String>>,
    /// The unique identifier for the recommender model to use for the message template. Amazon Pinpoint uses this value to determine how to retrieve and process data from a recommender model when it sends messages that use the template, if the template contains message variables for recommendation data.
    #[builder(into, default)]
    #[serde(rename = "recommenderId")]
    pub r#recommender_id: Box<Option<String>>,
    /// Subject line, or title, to use in email messages that are based on the message template.
    #[builder(into, default)]
    #[serde(rename = "subject")]
    pub r#subject: Box<Option<String>>,
    /// Message body, in plain text format, to use in email messages that are based on the message template. We recommend using plain text format for email clients that don't render HTML content and clients that are connected to high-latency networks, such as mobile devices.
    #[builder(into, default)]
    #[serde(rename = "textPart")]
    pub r#text_part: Box<Option<String>>,
}
