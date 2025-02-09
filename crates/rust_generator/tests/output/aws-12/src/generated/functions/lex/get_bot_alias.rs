#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bot_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBotAliasArgs {
        /// Name of the bot.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the bot alias. The name is case sensitive.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBotAliasResult {
        /// ARN of the bot alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the bot.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// Version of the bot that the alias points to.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// Checksum of the bot alias.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// Date that the bot alias was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date that the bot alias was updated. When you create a resource, the creation date and the last updated date are the same.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Name of the alias. The name is not case sensitive.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBotAliasArgs,
    ) -> GetBotAliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lex/getBotAlias:getBotAlias".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBotAliasResult {
            arn: o.get_field("arn"),
            bot_name: o.get_field("botName"),
            bot_version: o.get_field("botVersion"),
            checksum: o.get_field("checksum"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
        }
    }
}
