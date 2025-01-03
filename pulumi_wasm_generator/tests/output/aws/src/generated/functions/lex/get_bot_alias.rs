pub mod get_bot_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotAliasArgs {
        /// Name of the bot.
        #[builder(into)]
        pub bot_name: pulumi_wasm_rust::Output<String>,
        /// Name of the bot alias. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetBotAliasArgs) -> GetBotAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_name_binding = args.bot_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lex/getBotAlias:getBotAlias".into(),
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
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBotAliasResult {
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
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
