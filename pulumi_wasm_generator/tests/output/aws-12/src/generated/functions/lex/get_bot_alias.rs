pub mod get_bot_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotAliasArgs {
        /// Name of the bot.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the bot alias. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBotAliasResult {
        /// ARN of the bot alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the bot.
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Version of the bot that the alias points to.
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// Checksum of the bot alias.
        pub checksum: pulumi_wasm_rust::Output<String>,
        /// Date that the bot alias was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Name of the alias. The name is not case sensitive.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBotAliasArgs,
    ) -> GetBotAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lex/getBotAlias:getBotAlias".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botName".into(),
                    value: &bot_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBotAliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            bot_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("botName"),
            ),
            bot_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("botVersion"),
            ),
            checksum: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("checksum"),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedDate"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
