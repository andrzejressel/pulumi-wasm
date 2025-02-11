#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_server_active_directory_administratory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratoryArgs {
        #[builder(into)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub login: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub object_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratoryResult {
        pub identity_id: pulumi_gestalt_rust::Output<String>,
        pub login: pulumi_gestalt_rust::Output<String>,
        pub object_id: pulumi_gestalt_rust::Output<String>,
        pub server_id: pulumi_gestalt_rust::Output<String>,
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerActiveDirectoryAdministratoryArgs,
    ) -> FlexibleServerActiveDirectoryAdministratoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_id_binding = args.identity_id.get_output(context);
        let login_binding = args.login.get_output(context);
        let object_id_binding = args.object_id.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerActiveDirectoryAdministratory:FlexibleServerActiveDirectoryAdministratory"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "login".into(),
                    value: &login_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlexibleServerActiveDirectoryAdministratoryResult {
            identity_id: o.get_field("identityId"),
            login: o.get_field("login"),
            object_id: o.get_field("objectId"),
            server_id: o.get_field("serverId"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
