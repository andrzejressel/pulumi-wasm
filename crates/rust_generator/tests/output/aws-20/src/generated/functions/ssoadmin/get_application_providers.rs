#[allow(clippy::doc_lazy_continuation)]
pub mod get_application_providers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationProvidersArgs {
        /// A list of application providers available in the current region. See `application_providers` below.
        #[builder(into, default)]
        pub application_providers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationProvidersApplicationProvider,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApplicationProvidersResult {
        /// A list of application providers available in the current region. See `application_providers` below.
        pub application_providers: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationProvidersApplicationProvider,
                >,
            >,
        >,
        /// AWS region.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetApplicationProvidersArgs,
    ) -> GetApplicationProvidersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_providers_binding = args
            .application_providers
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getApplicationProviders:getApplicationProviders".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationProviders".into(),
                    value: &application_providers_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApplicationProvidersResult {
            application_providers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationProviders"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
