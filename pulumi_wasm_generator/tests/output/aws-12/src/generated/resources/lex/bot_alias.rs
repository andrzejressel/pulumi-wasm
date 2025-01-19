/// Provides an Amazon Lex Bot Alias resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod bot_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotAliasArgs {
        /// The name of the bot.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The version of the bot.
        #[builder(into)]
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// The settings that determine how Amazon Lex uses conversation logs for the alias. Attributes are documented under conversation_logs.
        #[builder(into, default)]
        pub conversation_logs: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::BotAliasConversationLogs>,
        >,
        /// A description of the alias. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the alias. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BotAliasResult {
        /// The ARN of the bot alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the bot.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// The version of the bot.
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// Checksum of the bot alias.
        pub checksum: pulumi_wasm_rust::Output<String>,
        /// The settings that determine how Amazon Lex uses conversation logs for the alias. Attributes are documented under conversation_logs.
        pub conversation_logs: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::BotAliasConversationLogs>,
        >,
        /// The date that the bot alias was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// A description of the alias. Must be less than or equal to 200 characters in length.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// The name of the alias. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BotAliasArgs) -> BotAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let bot_version_binding = args.bot_version.get_inner();
        let conversation_logs_binding = args.conversation_logs.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/botAlias:BotAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding,
                },
                register_interface::ObjectField {
                    name: "conversationLogs".into(),
                    value: &conversation_logs_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "botName".into(),
                },
                register_interface::ResultField {
                    name: "botVersion".into(),
                },
                register_interface::ResultField {
                    name: "checksum".into(),
                },
                register_interface::ResultField {
                    name: "conversationLogs".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BotAliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botName").unwrap(),
            ),
            bot_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botVersion").unwrap(),
            ),
            checksum: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checksum").unwrap(),
            ),
            conversation_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conversationLogs").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
