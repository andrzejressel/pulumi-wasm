pub mod get_bot_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotAssociationArgs {
        /// Identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Configuration information of an Amazon Lex (V1) bot. Detailed below.
        #[builder(into)]
        pub lex_bot: pulumi_wasm_rust::Output<
            super::super::super::types::connect::GetBotAssociationLexBot,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBotAssociationResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        pub lex_bot: pulumi_wasm_rust::Output<
            super::super::super::types::connect::GetBotAssociationLexBot,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBotAssociationArgs) -> GetBotAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_inner();
        let lex_bot_binding = args.lex_bot.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getBotAssociation:getBotAssociation".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "lexBot".into(),
                    value: &lex_bot_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "lexBot".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBotAssociationResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            lex_bot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lexBot").unwrap(),
            ),
        }
    }
}
