pub mod get_private_cloud {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateCloudArgs {
        /// Location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateCloudResult {
        pub deletion_delay_hours: pulumi_wasm_rust::Output<i32>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub hcxes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudHcx>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub management_clusters: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetPrivateCloudManagementCluster,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudNetworkConfig>,
        >,
        pub nsxes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudNsx>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub send_deletion_delay_hours_if_zero: pulumi_wasm_rust::Output<bool>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub vcenters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudVcenter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPrivateCloudArgs,
    ) -> GetPrivateCloudResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getPrivateCloud:getPrivateCloud".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deletionDelayHours".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "hcxes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementClusters".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfigs".into(),
                },
                register_interface::ResultField {
                    name: "nsxes".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "sendDeletionDelayHoursIfZero".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "vcenters".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPrivateCloudResult {
            deletion_delay_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionDelayHours").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            hcxes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hcxes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementClusters").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfigs").unwrap(),
            ),
            nsxes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nsxes").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            send_deletion_delay_hours_if_zero: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendDeletionDelayHoursIfZero").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            vcenters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcenters").unwrap(),
            ),
        }
    }
}
