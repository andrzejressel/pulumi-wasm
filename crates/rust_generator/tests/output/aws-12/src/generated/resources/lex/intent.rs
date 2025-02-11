/// Provides an Amazon Lex Intent resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   orderFlowersIntent:
///     type: aws:lex:Intent
///     name: order_flowers_intent
///     properties:
///       confirmationPrompt:
///         maxAttempts: 2
///         messages:
///           - content: Okay, your {FlowerType} will be ready for pickup by {PickupTime} on {PickupDate}.  Does this sound okay?
///             contentType: PlainText
///       createVersion: false
///       name: OrderFlowers
///       description: Intent to order a bouquet of flowers for pick up
///       fulfillmentActivity:
///         type: ReturnIntent
///       rejectionStatement:
///         messages:
///           - content: Okay, I will not place your order.
///             contentType: PlainText
///       sampleUtterances:
///         - I would like to order some flowers
///         - I would like to pick up flowers
///       slots:
///         - description: The type of flowers to pick up
///           name: FlowerType
///           priority: 1
///           sampleUtterances:
///             - I would like to order {FlowerType}
///           slotConstraint: Required
///           slotType: FlowerTypes
///           slotTypeVersion: $$LATEST
///           valueElicitationPrompt:
///             maxAttempts: 2
///             messages:
///               - content: What type of flowers would you like to order?
///                 contentType: PlainText
///         - description: The date to pick up the flowers
///           name: PickupDate
///           priority: 2
///           sampleUtterances:
///             - I would like to order {FlowerType}
///           slotConstraint: Required
///           slotType: AMAZON.DATE
///           slotTypeVersion: $$LATEST
///           valueElicitationPrompt:
///             maxAttempts: 2
///             messages:
///               - content: What day do you want the {FlowerType} to be picked up?
///                 contentType: PlainText
///         - description: The time to pick up the flowers
///           name: PickupTime
///           priority: 3
///           sampleUtterances:
///             - I would like to order {FlowerType}
///           slotConstraint: Required
///           slotType: AMAZON.TIME
///           slotTypeVersion: $$LATEST
///           valueElicitationPrompt:
///             maxAttempts: 2
///             messages:
///               - content: Pick up the {FlowerType} at what time on {PickupDate}?
///                 contentType: PlainText
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import intents using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lex/intent:Intent order_flowers_intent OrderFlowers
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod intent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntentArgs {
        /// The statement that you want Amazon Lex to convey to the user
        /// after the intent is successfully fulfilled by the Lambda function. This element is relevant only if
        /// you provide a Lambda function in the `fulfillment_activity`. If you return the intent to the client
        /// application, you can't specify this element. The `follow_up_prompt` and `conclusion_statement` are
        /// mutually exclusive. You can specify only one. Attributes are documented under statement.
        #[builder(into, default)]
        pub conclusion_statement: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::IntentConclusionStatement>,
        >,
        /// Prompts the user to confirm the intent. This question should
        /// have a yes or no answer. You you must provide both the `rejection_statement` and `confirmation_prompt`,
        /// or neither. Attributes are documented under prompt.
        #[builder(into, default)]
        pub confirmation_prompt: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::IntentConfirmationPrompt>,
        >,
        /// Determines if a new slot type version is created when the initial
        /// resource is created and on each update. Defaults to `false`.
        #[builder(into, default)]
        pub create_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of the intent. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a Lambda function to invoke for each user input. You can
        /// invoke this Lambda function to personalize user interaction. Attributes are documented under code_hook.
        #[builder(into, default)]
        pub dialog_code_hook: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::IntentDialogCodeHook>,
        >,
        /// Amazon Lex uses this prompt to solicit additional activity after
        /// fulfilling an intent. For example, after the OrderPizza intent is fulfilled, you might prompt the
        /// user to order a drink. The `follow_up_prompt` field and the `conclusion_statement` field are mutually
        /// exclusive. You can specify only one. Attributes are documented under follow_up_prompt.
        #[builder(into, default)]
        pub follow_up_prompt: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::IntentFollowUpPrompt>,
        >,
        /// Describes how the intent is fulfilled. For example, after a
        /// user provides all of the information for a pizza order, `fulfillment_activity` defines how the bot
        /// places an order with a local pizza store. Attributes are documented under fulfillment_activity.
        #[builder(into)]
        pub fulfillment_activity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::lex::IntentFulfillmentActivity,
        >,
        /// The name of the intent, not case sensitive. Must be less than or equal to 100 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique identifier for the built-in intent to base this
        /// intent on. To find the signature for an intent, see
        /// [Standard Built-in Intents](https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents)
        /// in the Alexa Skills Kit.
        #[builder(into, default)]
        pub parent_intent_signature: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When the user answers "no" to the question defined in
        /// `confirmation_prompt`, Amazon Lex responds with this statement to acknowledge that the intent was
        /// canceled. You must provide both the `rejection_statement` and the `confirmation_prompt`, or neither.
        /// Attributes are documented under statement.
        #[builder(into, default)]
        pub rejection_statement: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::IntentRejectionStatement>,
        >,
        /// An array of utterances (strings) that a user might say to signal
        /// the intent. For example, "I want {PizzaSize} pizza", "Order {Quantity} {PizzaSize} pizzas".
        /// In each utterance, a slot name is enclosed in curly braces. Must have between 1 and 10 items in the list, and each item must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub sample_utterances: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An list of intent slots. At runtime, Amazon Lex elicits required slot values
        /// from the user using prompts defined in the slots. Attributes are documented under slot.
        #[builder(into, default)]
        pub slots: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lex::IntentSlot>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntentResult {
        /// The ARN of the Lex intent.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Checksum identifying the version of the intent that was created. The checksum is not
        /// included as an argument because the resource will add it automatically when updating the intent.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// The statement that you want Amazon Lex to convey to the user
        /// after the intent is successfully fulfilled by the Lambda function. This element is relevant only if
        /// you provide a Lambda function in the `fulfillment_activity`. If you return the intent to the client
        /// application, you can't specify this element. The `follow_up_prompt` and `conclusion_statement` are
        /// mutually exclusive. You can specify only one. Attributes are documented under statement.
        pub conclusion_statement: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::IntentConclusionStatement>,
        >,
        /// Prompts the user to confirm the intent. This question should
        /// have a yes or no answer. You you must provide both the `rejection_statement` and `confirmation_prompt`,
        /// or neither. Attributes are documented under prompt.
        pub confirmation_prompt: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::IntentConfirmationPrompt>,
        >,
        /// Determines if a new slot type version is created when the initial
        /// resource is created and on each update. Defaults to `false`.
        pub create_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The date when the intent version was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// A description of the intent. Must be less than or equal to 200 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a Lambda function to invoke for each user input. You can
        /// invoke this Lambda function to personalize user interaction. Attributes are documented under code_hook.
        pub dialog_code_hook: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::IntentDialogCodeHook>,
        >,
        /// Amazon Lex uses this prompt to solicit additional activity after
        /// fulfilling an intent. For example, after the OrderPizza intent is fulfilled, you might prompt the
        /// user to order a drink. The `follow_up_prompt` field and the `conclusion_statement` field are mutually
        /// exclusive. You can specify only one. Attributes are documented under follow_up_prompt.
        pub follow_up_prompt: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::IntentFollowUpPrompt>,
        >,
        /// Describes how the intent is fulfilled. For example, after a
        /// user provides all of the information for a pizza order, `fulfillment_activity` defines how the bot
        /// places an order with a local pizza store. Attributes are documented under fulfillment_activity.
        pub fulfillment_activity: pulumi_gestalt_rust::Output<
            super::super::types::lex::IntentFulfillmentActivity,
        >,
        /// The date when the $LATEST version of this intent was updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the intent, not case sensitive. Must be less than or equal to 100 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the built-in intent to base this
        /// intent on. To find the signature for an intent, see
        /// [Standard Built-in Intents](https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents)
        /// in the Alexa Skills Kit.
        pub parent_intent_signature: pulumi_gestalt_rust::Output<Option<String>>,
        /// When the user answers "no" to the question defined in
        /// `confirmation_prompt`, Amazon Lex responds with this statement to acknowledge that the intent was
        /// canceled. You must provide both the `rejection_statement` and the `confirmation_prompt`, or neither.
        /// Attributes are documented under statement.
        pub rejection_statement: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::IntentRejectionStatement>,
        >,
        /// An array of utterances (strings) that a user might say to signal
        /// the intent. For example, "I want {PizzaSize} pizza", "Order {Quantity} {PizzaSize} pizzas".
        /// In each utterance, a slot name is enclosed in curly braces. Must have between 1 and 10 items in the list, and each item must be less than or equal to 200 characters in length.
        pub sample_utterances: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An list of intent slots. At runtime, Amazon Lex elicits required slot values
        /// from the user using prompts defined in the slots. Attributes are documented under slot.
        pub slots: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lex::IntentSlot>>,
        >,
        /// The version of the bot.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntentArgs,
    ) -> IntentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let conclusion_statement_binding = args.conclusion_statement.get_output(context);
        let confirmation_prompt_binding = args.confirmation_prompt.get_output(context);
        let create_version_binding = args.create_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let dialog_code_hook_binding = args.dialog_code_hook.get_output(context);
        let follow_up_prompt_binding = args.follow_up_prompt.get_output(context);
        let fulfillment_activity_binding = args.fulfillment_activity.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_intent_signature_binding = args
            .parent_intent_signature
            .get_output(context);
        let rejection_statement_binding = args.rejection_statement.get_output(context);
        let sample_utterances_binding = args.sample_utterances.get_output(context);
        let slots_binding = args.slots.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/intent:Intent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conclusionStatement".into(),
                    value: &conclusion_statement_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confirmationPrompt".into(),
                    value: &confirmation_prompt_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createVersion".into(),
                    value: &create_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dialogCodeHook".into(),
                    value: &dialog_code_hook_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "followUpPrompt".into(),
                    value: &follow_up_prompt_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fulfillmentActivity".into(),
                    value: &fulfillment_activity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentIntentSignature".into(),
                    value: &parent_intent_signature_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rejectionStatement".into(),
                    value: &rejection_statement_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sampleUtterances".into(),
                    value: &sample_utterances_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slots".into(),
                    value: &slots_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntentResult {
            arn: o.get_field("arn"),
            checksum: o.get_field("checksum"),
            conclusion_statement: o.get_field("conclusionStatement"),
            confirmation_prompt: o.get_field("confirmationPrompt"),
            create_version: o.get_field("createVersion"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            dialog_code_hook: o.get_field("dialogCodeHook"),
            follow_up_prompt: o.get_field("followUpPrompt"),
            fulfillment_activity: o.get_field("fulfillmentActivity"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
            parent_intent_signature: o.get_field("parentIntentSignature"),
            rejection_statement: o.get_field("rejectionStatement"),
            sample_utterances: o.get_field("sampleUtterances"),
            slots: o.get_field("slots"),
            version: o.get_field("version"),
        }
    }
}
