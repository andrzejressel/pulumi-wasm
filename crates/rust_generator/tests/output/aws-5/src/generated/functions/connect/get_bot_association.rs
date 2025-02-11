#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetBotAssociationArgs,
    ) -> GetBotAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let lex_bot_binding = args.lex_bot.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getBotAssociation:getBotAssociation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lexBot".into(),
                    value: &lex_bot_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBotAssociationResult {
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            lex_bot: o.get_field("lexBot"),
        }
    }
}
