/// An intent represents a user's intent to interact with a conversational agent.
///
///
/// To get more information about Intent, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.intents)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
/// ## Example Usage
///
/// ### Dialogflowcx Intent Full
///
///
/// ```yaml
/// resources:
///   agent:
///     type: gcp:diagflow:CxAgent
///     properties:
///       displayName: dialogflowcx-agent
///       location: global
///       defaultLanguageCode: en
///       supportedLanguageCodes:
///         - fr
///         - de
///         - es
///       timeZone: America/New_York
///       description: Example description.
///       avatarUri: https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png
///       enableStackdriverLogging: true
///       enableSpellCorrection: true
///       speechToTextSettings:
///         enableSpeechAdaptation: true
///   basicIntent:
///     type: gcp:diagflow:CxIntent
///     name: basic_intent
///     properties:
///       parent: ${agent.id}
///       displayName: Example
///       priority: 1
///       description: Intent example
///       trainingPhrases:
///         - parts:
///             - text: training
///             - text: phrase
///             - text: example
///           repeatCount: 1
///       parameters:
///         - id: param1
///           entityType: projects/-/locations/-/agents/-/entityTypes/sys.date
///       labels:
///         label1: value1
///         label2: value2
/// ```
///
/// ## Import
///
/// Intent can be imported using any of these accepted formats:
///
/// * `{{parent}}/intents/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, Intent can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxIntent:CxIntent default {{parent}}/intents/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxIntent:CxIntent default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cx_intent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxIntentArgs {
        /// Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The human-readable name of the intent, unique within the agent.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Marks this as the [Default Negative Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#negative) for an agent. When you create an agent, a Default Negative Intent is created automatically.
        /// The Default Negative Intent cannot be deleted; deleting the `gcp.diagflow.CxIntent` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxIntent` resources linked to the same agent with `is_default_negative_intent = true` because they will compete to control a single Default Negative Intent resource in GCP.
        #[builder(into, default)]
        pub is_default_negative_intent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Marks this as the [Default Welcome Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#welcome) for an agent. When you create an agent, a Default Welcome Intent is created automatically.
        /// The Default Welcome Intent cannot be deleted; deleting the `gcp.diagflow.CxIntent` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxIntent` resources linked to the same agent with `is_default_welcome_intent = true` because they will compete to control a single Default Welcome Intent resource in GCP.
        #[builder(into, default)]
        pub is_default_welcome_intent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation.
        /// Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.
        /// To manage the fallback intent, set `is_default_negative_intent = true`
        #[builder(into, default)]
        pub is_fallback: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes.
        /// Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The language of the following fields in intent:
        /// Intent.training_phrases.parts.text
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        #[builder(into, default)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The collection of parameters associated with the intent.
        /// Structure is documented below.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::CxIntentParameter>>,
        >,
        /// The agent to create an intent for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of this intent. Higher numbers represent higher priorities.
        /// If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the Normal priority in the console.
        /// If the supplied value is negative, the intent is ignored in runtime detect intent requests.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The collection of training phrases the agent is trained on to identify the intent.
        /// Structure is documented below.
        #[builder(into, default)]
        pub training_phrases: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::diagflow::CxIntentTrainingPhrase>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CxIntentResult {
        /// Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The human-readable name of the intent, unique within the agent.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Marks this as the [Default Negative Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#negative) for an agent. When you create an agent, a Default Negative Intent is created automatically.
        /// The Default Negative Intent cannot be deleted; deleting the `gcp.diagflow.CxIntent` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxIntent` resources linked to the same agent with `is_default_negative_intent = true` because they will compete to control a single Default Negative Intent resource in GCP.
        pub is_default_negative_intent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Marks this as the [Default Welcome Intent](https://cloud.google.com/dialogflow/cx/docs/concept/intent#welcome) for an agent. When you create an agent, a Default Welcome Intent is created automatically.
        /// The Default Welcome Intent cannot be deleted; deleting the `gcp.diagflow.CxIntent` resource does nothing to the underlying GCP resources.
        ///
        /// > Avoid having multiple `gcp.diagflow.CxIntent` resources linked to the same agent with `is_default_welcome_intent = true` because they will compete to control a single Default Welcome Intent resource in GCP.
        pub is_default_welcome_intent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation.
        /// Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event.
        /// To manage the fallback intent, set `is_default_negative_intent = true`
        pub is_fallback: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes.
        /// Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. "sys-head" means the intent is a head intent. "sys.contextual" means the intent is a contextual intent.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The language of the following fields in intent:
        /// Intent.training_phrases.parts.text
        /// If not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used.
        pub language_code: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier of the intent.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The collection of parameters associated with the intent.
        /// Structure is documented below.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::diagflow::CxIntentParameter>>,
        >,
        /// The agent to create an intent for.
        /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The priority of this intent. Higher numbers represent higher priorities.
        /// If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the Normal priority in the console.
        /// If the supplied value is negative, the intent is ignored in runtime detect intent requests.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The collection of training phrases the agent is trained on to identify the intent.
        /// Structure is documented below.
        pub training_phrases: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::diagflow::CxIntentTrainingPhrase>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CxIntentArgs,
    ) -> CxIntentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let is_default_negative_intent_binding = args
            .is_default_negative_intent
            .get_output(context);
        let is_default_welcome_intent_binding = args
            .is_default_welcome_intent
            .get_output(context);
        let is_fallback_binding = args.is_fallback.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let training_phrases_binding = args.training_phrases.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/cxIntent:CxIntent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefaultNegativeIntent".into(),
                    value: is_default_negative_intent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefaultWelcomeIntent".into(),
                    value: is_default_welcome_intent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isFallback".into(),
                    value: is_fallback_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trainingPhrases".into(),
                    value: training_phrases_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CxIntentResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            is_default_negative_intent: o.get_field("isDefaultNegativeIntent"),
            is_default_welcome_intent: o.get_field("isDefaultWelcomeIntent"),
            is_fallback: o.get_field("isFallback"),
            labels: o.get_field("labels"),
            language_code: o.get_field("languageCode"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            parent: o.get_field("parent"),
            priority: o.get_field("priority"),
            pulumi_labels: o.get_field("pulumiLabels"),
            training_phrases: o.get_field("trainingPhrases"),
        }
    }
}
