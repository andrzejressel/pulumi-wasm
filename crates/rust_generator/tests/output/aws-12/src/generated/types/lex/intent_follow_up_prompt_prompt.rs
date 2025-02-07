#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntentFollowUpPromptPrompt {
    /// The number of times to prompt the user for information. Must be a number between 1 and 5 (inclusive).
    #[builder(into)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<i32>,
    /// A set of messages, each of which provides a message string and its type.
    /// You can specify the message string in plain text or in Speech Synthesis Markup Language (SSML).
    /// Attributes are documented under message. Must contain between 1 and 15 messages.
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Vec<super::super::types::lex::IntentFollowUpPromptPromptMessage>>,
    /// The response card. Amazon Lex will substitute session attributes and
    /// slot values into the response card. For more information, see
    /// [Example: Using a Response Card](https://docs.aws.amazon.com/lex/latest/dg/ex-resp-card.html). Must be less than or equal to 50000 characters in length.
    #[builder(into, default)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Box<Option<String>>,
}
