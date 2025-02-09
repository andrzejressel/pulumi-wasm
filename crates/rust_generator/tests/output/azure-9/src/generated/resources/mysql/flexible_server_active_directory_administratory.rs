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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FlexibleServerActiveDirectoryAdministratoryArgs,
    ) -> FlexibleServerActiveDirectoryAdministratoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identity_id_binding_1 = args.identity_id.get_output(context);
        let identity_id_binding = identity_id_binding_1.get_inner();
        let login_binding_1 = args.login.get_output(context);
        let login_binding = login_binding_1.get_inner();
        let object_id_binding_1 = args.object_id.get_output(context);
        let object_id_binding = object_id_binding_1.get_inner();
        let server_id_binding_1 = args.server_id.get_output(context);
        let server_id_binding = server_id_binding_1.get_inner();
        let tenant_id_binding_1 = args.tenant_id.get_output(context);
        let tenant_id_binding = tenant_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerActiveDirectoryAdministratory:FlexibleServerActiveDirectoryAdministratory"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "login".into(),
                    value: &login_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FlexibleServerActiveDirectoryAdministratoryResult {
            identity_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityId"),
            ),
            login: pulumi_gestalt_rust::__private::into_domain(o.extract_field("login")),
            object_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("objectId"),
            ),
            server_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverId"),
            ),
            tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
