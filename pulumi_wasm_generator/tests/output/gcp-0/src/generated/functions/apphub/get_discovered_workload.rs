pub mod get_discovered_workload {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiscoveredWorkloadArgs {
        /// The location of the discovered workload.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The host project of the discovered workload.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The uri of the workload (instance group managed by the Instance Group Manager). Example: "//compute.googleapis.com/projects/1/regions/us-east1/instanceGroups/id1"
        #[builder(into)]
        pub workload_uri: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiscoveredWorkloadResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location that the underlying resource resides in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Resource name of a Workload. Format: "projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}".
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Properties of an underlying compute resource that can comprise a Workload. Structure is documented below
        pub workload_properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::apphub::GetDiscoveredWorkloadWorkloadProperty,
            >,
        >,
        /// Reference to an underlying networking resource that can comprise a Workload. Structure is documented below
        pub workload_references: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::apphub::GetDiscoveredWorkloadWorkloadReference,
            >,
        >,
        pub workload_uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDiscoveredWorkloadArgs,
    ) -> GetDiscoveredWorkloadResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let workload_uri_binding = args.workload_uri.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:apphub/getDiscoveredWorkload:getDiscoveredWorkload".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "workloadUri".into(),
                    value: &workload_uri_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDiscoveredWorkloadResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            workload_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workloadProperties"),
            ),
            workload_references: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workloadReferences"),
            ),
            workload_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workloadUri"),
            ),
        }
    }
}
