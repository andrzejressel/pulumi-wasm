/// Provides an Amazon Lex Bot Alias resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let orderFlowersProd = bot_alias::create(
///         "orderFlowersProd",
///         BotAliasArgs::builder()
///             .bot_name("OrderFlowers")
///             .bot_version("1")
///             .description("Production Version of the OrderFlowers Bot.")
///             .name("OrderFlowersProd")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import bot aliases using an ID with the format `bot_name:bot_alias_name`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/botAlias:BotAlias order_flowers_prod OrderFlowers:OrderFlowersProd
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bot_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotAliasArgs {
        /// The name of the bot.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version of the bot.
        #[builder(into)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The settings that determine how Amazon Lex uses conversation logs for the alias. Attributes are documented under conversation_logs.
        #[builder(into, default)]
        pub conversation_logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::BotAliasConversationLogs>,
        >,
        /// A description of the alias. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the alias. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BotAliasResult {
        /// The ARN of the bot alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the bot.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// The version of the bot.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Checksum of the bot alias.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// The settings that determine how Amazon Lex uses conversation logs for the alias. Attributes are documented under conversation_logs.
        pub conversation_logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::BotAliasConversationLogs>,
        >,
        /// The date that the bot alias was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// A description of the alias. Must be less than or equal to 200 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the alias. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BotAliasArgs,
    ) -> BotAliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let bot_version_binding = args.bot_version.get_output(context);
        let conversation_logs_binding = args.conversation_logs.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/botAlias:BotAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botVersion".into(),
                    value: bot_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conversationLogs".into(),
                    value: conversation_logs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BotAliasResult {
            arn: o.get_field("arn"),
            bot_name: o.get_field("botName"),
            bot_version: o.get_field("botVersion"),
            checksum: o.get_field("checksum"),
            conversation_logs: o.get_field("conversationLogs"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
        }
    }
}
