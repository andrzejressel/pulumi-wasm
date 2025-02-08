#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_delegated_administrators {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDelegatedAdministratorsArgs {
        /// Specifies a service principal name. If specified, then the operation lists the delegated administrators only for the specified service. If you don't specify a service principal, the operation lists all delegated administrators for all services in your organization.
        #[builder(into, default)]
        pub service_principal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDelegatedAdministratorsResult {
        /// The list of delegated administrators in your organization, which have the following attributes:
        pub delegated_administrators: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::organizations::GetDelegatedAdministratorsDelegatedAdministrator,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub service_principal: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDelegatedAdministratorsArgs,
    ) -> GetDelegatedAdministratorsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let service_principal_binding = args
            .service_principal
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getDelegatedAdministrators:getDelegatedAdministrators"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "servicePrincipal".into(),
                    value: &service_principal_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDelegatedAdministratorsResult {
            delegated_administrators: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delegatedAdministrators"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            service_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
        }
    }
}
