pub mod get_instances {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstancesArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [describe-instances in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstancesFilter>>,
        >,
        /// List of instance states that should be applicable to the desired instances. The permitted values are: `pending, running, shutting-down, stopped, stopping, terminated`. The default value is `running`.
        #[builder(into, default)]
        pub instance_state_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags, each pair of which must
        /// exactly match a pair on desired instances.
        #[builder(into, default)]
        pub instance_tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstancesResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstancesFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// IDs of instances found through the filter
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub instance_state_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub instance_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// IPv6 addresses of instances found through the filter
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Private IP addresses of instances found through the filter
        pub private_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// Public IP addresses of instances found through the filter
        pub public_ips: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstancesArgs,
    ) -> GetInstancesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let instance_state_names_binding = args
            .instance_state_names
            .get_output(context)
            .get_inner();
        let instance_tags_binding = args.instance_tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstances:getInstances".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "instanceStateNames".into(),
                    value: &instance_state_names_binding,
                },
                register_interface::ObjectField {
                    name: "instanceTags".into(),
                    value: &instance_tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "instanceStateNames".into(),
                },
                register_interface::ResultField {
                    name: "instanceTags".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Addresses".into(),
                },
                register_interface::ResultField {
                    name: "privateIps".into(),
                },
                register_interface::ResultField {
                    name: "publicIps".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstancesResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            instance_state_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceStateNames").unwrap(),
            ),
            instance_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTags").unwrap(),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Addresses").unwrap(),
            ),
            private_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIps").unwrap(),
            ),
            public_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIps").unwrap(),
            ),
        }
    }
}
