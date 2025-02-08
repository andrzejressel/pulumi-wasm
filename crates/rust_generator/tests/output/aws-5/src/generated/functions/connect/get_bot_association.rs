#[allow(clippy::doc_lazy_continuation)]
pub mod get_bot_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotAssociationArgs {
        /// Identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration information of an Amazon Lex (V1) bot. Detailed below.
        #[builder(into)]
        pub lex_bot: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::connect::GetBotAssociationLexBot,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBotAssociationResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub lex_bot: pulumi_gestalt_rust::Output<
            super::super::super::types::connect::GetBotAssociationLexBot,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBotAssociationArgs,
    ) -> GetBotAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let lex_bot_binding = args.lex_bot.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getBotAssociation:getBotAssociation".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBotAssociationResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            lex_bot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lexBot"),
            ),
        }
    }
}
