#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectorArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetConnectorResult {
        pub connected_projects: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        pub machine_type: pulumi_gestalt_rust::Output<String>,
        pub max_instances: pulumi_gestalt_rust::Output<i32>,
        pub max_throughput: pulumi_gestalt_rust::Output<i32>,
        pub min_instances: pulumi_gestalt_rust::Output<i32>,
        pub min_throughput: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub subnets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vpcaccess::GetConnectorSubnet>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetConnectorArgs,
    ) -> GetConnectorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vpcaccess/getConnector:getConnector".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConnectorResult {
            connected_projects: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectedProjects"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_cidr_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipCidrRange"),
            ),
            machine_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("machineType"),
            ),
            max_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxInstances"),
            ),
            max_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxThroughput"),
            ),
            min_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minInstances"),
            ),
            min_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minThroughput"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            subnets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
        }
    }
}
