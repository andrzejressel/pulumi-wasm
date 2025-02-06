pub mod get_instances {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstancesArgs {
        /// One or more name/value pairs to use as filters. There are
        /// several valid keys, for a full reference, check out
        /// [describe-instances in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstancesFilter>>,
        >,
        /// List of instance states that should be applicable to the desired instances. The permitted values are: `pending, running, shutting-down, stopped, stopping, terminated`. The default value is `running`.
        #[builder(into, default)]
        pub instance_state_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Map of tags, each pair of which must
        /// exactly match a pair on desired instances.
        #[builder(into, default)]
        pub instance_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstancesResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstancesFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IDs of instances found through the filter
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub instance_state_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub instance_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// IPv6 addresses of instances found through the filter
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Private IP addresses of instances found through the filter
        pub private_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Public IP addresses of instances found through the filter
        pub public_ips: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstancesArgs,
    ) -> GetInstancesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstancesResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ids")),
            instance_state_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceStateNames"),
            ),
            instance_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceTags"),
            ),
            ipv6_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Addresses"),
            ),
            private_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIps"),
            ),
            public_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIps"),
            ),
        }
    }
}
