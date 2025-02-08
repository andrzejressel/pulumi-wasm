#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_region_network_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionNetworkEndpointGroupArgs {
        /// The Network Endpoint Group name. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project to list versions in. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the Serverless REGs Reside. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group self_link.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionNetworkEndpointGroupResult {
        pub app_engines: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupAppEngine,
            >,
        >,
        pub cloud_functions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupCloudFunction,
            >,
        >,
        pub cloud_runs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupCloudRun,
            >,
        >,
        /// The RNEG description.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The network to which all network endpoints in the RNEG belong.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        pub network_endpoint_type: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub psc_datas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupPscData,
            >,
        >,
        /// The target service url used to set up private service connection to a Google API or a PSC Producer Service Attachment.
        pub psc_target_service: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        pub serverless_deployments: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupServerlessDeployment,
            >,
        >,
        /// subnetwork to which all network endpoints in the RNEG belong.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRegionNetworkEndpointGroupArgs,
    ) -> GetRegionNetworkEndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let self_link_binding = args.self_link.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getRegionNetworkEndpointGroup:getRegionNetworkEndpointGroup"
                .into(),
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
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegionNetworkEndpointGroupResult {
            app_engines: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appEngines"),
            ),
            cloud_functions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudFunctions"),
            ),
            cloud_runs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudRuns"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_endpoint_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkEndpointType"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_datas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscDatas"),
            ),
            psc_target_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pscTargetService"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            serverless_deployments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverlessDeployments"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
        }
    }
}
