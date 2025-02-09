#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// String for what access a user possesses within the associated ElastiCache replication groups or clusters.
        #[builder(into, default)]
        pub access_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub authentication_modes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::elasticache::GetUserAuthenticationMode>,
            >,
        >,
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub no_password_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub passwords: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Identifier for the user.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User name of the user.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// String for what access a user possesses within the associated ElastiCache replication groups or clusters.
        pub access_string: pulumi_gestalt_rust::Output<Option<String>>,
        pub authentication_modes: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::elasticache::GetUserAuthenticationMode>,
            >,
        >,
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub no_password_required: pulumi_gestalt_rust::Output<Option<bool>>,
        pub passwords: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Identifier for the user.
        pub user_id: pulumi_gestalt_rust::Output<String>,
        /// User name of the user.
        pub user_name: pulumi_gestalt_rust::Output<Option<String>>,
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
        let access_string_binding = args.access_string.get_output(context);
        let authentication_modes_binding = args.authentication_modes.get_output(context);
        let engine_binding = args.engine.get_output(context);
        let no_password_required_binding = args.no_password_required.get_output(context);
        let passwords_binding = args.passwords.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elasticache/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessString".into(),
                    value: access_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationModes".into(),
                    value: authentication_modes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noPasswordRequired".into(),
                    value: no_password_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwords".into(),
                    value: passwords_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
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
            authentication_modes: o.get_field("authenticationModes"),
            engine: o.get_field("engine"),
            id: o.get_field("id"),
            no_password_required: o.get_field("noPasswordRequired"),
            passwords: o.get_field("passwords"),
            user_id: o.get_field("userId"),
            user_name: o.get_field("userName"),
        }
    }
}
