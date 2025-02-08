#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ca_certs {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCaCertsArgs {
        /// The name or self link of the instance.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCaCertsResult {
        /// SHA1 fingerprint of the currently active CA certificate.
        pub active_version: pulumi_gestalt_rust::Output<String>,
        /// A list of server CA certificates for the instance. Each contains:
        pub certs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetCaCertsCert>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCaCertsArgs,
    ) -> GetCaCertsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getCaCerts:getCaCerts".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCaCertsResult {
            active_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activeVersion"),
            ),
            certs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("certs")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
        }
    }
}
