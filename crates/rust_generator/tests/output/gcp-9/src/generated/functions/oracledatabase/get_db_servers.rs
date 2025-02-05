pub mod get_db_servers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbServersArgs {
        /// The Exadata Infrastructure id.
        #[builder(into)]
        pub cloud_exadata_infrastructure: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project to which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDbServersResult {
        pub cloud_exadata_infrastructure: pulumi_wasm_rust::Output<String>,
        pub db_servers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oracledatabase::GetDbServersDbServer>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDbServersArgs,
    ) -> GetDbServersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_exadata_infrastructure_binding = args
            .cloud_exadata_infrastructure
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:oracledatabase/getDbServers:getDbServers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudExadataInfrastructure".into(),
                    value: &cloud_exadata_infrastructure_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDbServersResult {
            cloud_exadata_infrastructure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudExadataInfrastructure"),
            ),
            db_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbServers"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
