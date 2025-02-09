#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// Map of tags assigned to the user.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the user.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// Access permissions string used for this user.
        pub access_string: pulumi_gestalt_rust::Output<String>,
        /// ARN of the user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Denotes the user's authentication properties.
        pub authentication_modes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::memorydb::GetUserAuthenticationMode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Minimum engine version supported for the user.
        pub minimum_engine_version: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the user.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:memorydb/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserResult {
            access_string: o.get_field("accessString"),
            arn: o.get_field("arn"),
            authentication_modes: o.get_field("authenticationModes"),
            id: o.get_field("id"),
            minimum_engine_version: o.get_field("minimumEngineVersion"),
            tags: o.get_field("tags"),
            user_name: o.get_field("userName"),
        }
    }
}
