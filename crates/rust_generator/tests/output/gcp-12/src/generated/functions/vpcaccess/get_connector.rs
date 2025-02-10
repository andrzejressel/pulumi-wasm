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
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectorArgs,
    ) -> GetConnectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vpcaccess/getConnector:getConnector".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectorResult {
            connected_projects: o.get_field("connectedProjects"),
            id: o.get_field("id"),
            ip_cidr_range: o.get_field("ipCidrRange"),
            machine_type: o.get_field("machineType"),
            max_instances: o.get_field("maxInstances"),
            max_throughput: o.get_field("maxThroughput"),
            min_instances: o.get_field("minInstances"),
            min_throughput: o.get_field("minThroughput"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            state: o.get_field("state"),
            subnets: o.get_field("subnets"),
        }
    }
}
