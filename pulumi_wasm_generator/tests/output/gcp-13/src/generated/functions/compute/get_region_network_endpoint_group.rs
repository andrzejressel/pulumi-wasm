pub mod get_region_network_endpoint_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionNetworkEndpointGroupArgs {
        /// The Network Endpoint Group name. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project to list versions in. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the Serverless REGs Reside. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Network Endpoint Group self_link.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionNetworkEndpointGroupResult {
        pub app_engines: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupAppEngine,
            >,
        >,
        pub cloud_functions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupCloudFunction,
            >,
        >,
        pub cloud_runs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupCloudRun,
            >,
        >,
        /// The RNEG description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network to which all network endpoints in the RNEG belong.
        pub network: pulumi_wasm_rust::Output<String>,
        /// Type of network endpoints in this network endpoint group.
        pub network_endpoint_type: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub psc_datas: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupPscData,
            >,
        >,
        /// The target service url used to set up private service connection to a Google API or a PSC Producer Service Attachment.
        pub psc_target_service: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        pub serverless_deployments: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionNetworkEndpointGroupServerlessDeployment,
            >,
        >,
        /// subnetwork to which all network endpoints in the RNEG belong.
        pub subnetwork: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegionNetworkEndpointGroupArgs,
    ) -> GetRegionNetworkEndpointGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appEngines".into(),
                },
                register_interface::ResultField {
                    name: "cloudFunctions".into(),
                },
                register_interface::ResultField {
                    name: "cloudRuns".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkEndpointType".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pscDatas".into(),
                },
                register_interface::ResultField {
                    name: "pscTargetService".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serverlessDeployments".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegionNetworkEndpointGroupResult {
            app_engines: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appEngines").unwrap(),
            ),
            cloud_functions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudFunctions").unwrap(),
            ),
            cloud_runs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudRuns").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkEndpointType").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            psc_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscDatas").unwrap(),
            ),
            psc_target_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pscTargetService").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            serverless_deployments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverlessDeployments").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
        }
    }
}
