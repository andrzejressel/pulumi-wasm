pub mod get_addresses {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAddressesArgs {
        /// A filter expression that
        /// filters resources listed in the response. The expression must specify
        /// the field name, an operator, and the value that you want to use for
        /// filtering. The value must be a string, a number, or a boolean. The
        /// operator must be either "=", "!=", ">", "<", "<=", ">=" or ":". For
        /// example, if you are filtering Compute Engine instances, you can
        /// exclude instances named "example-instance" by specifying "name !=
        /// example-instance". The ":" operator can be used with string fields to
        /// match substrings. For non-string fields it is equivalent to the "="
        /// operator. The ":*" comparison can be used to test whether a key has
        /// been defined. For example, to find all objects with "owner" label
        /// use: """ labels.owner:* """ You can also filter nested fields. For
        /// example, you could specify "scheduling.automaticRestart = false" to
        /// include instances only if they are not scheduled for automatic
        /// restarts. You can use filtering on nested fields to filter based on
        /// resource labels. To filter on multiple expressions, provide each
        /// separate expression within parentheses. For example: """
        /// (scheduling.automaticRestart = true) (cpuPlatform = "Intel Skylake")
        /// """ By default, each expression is an "AND" expression. However, you
        /// can include "AND" and "OR" expressions explicitly. For example: """
        /// (cpuPlatform = "Intel Skylake") OR (cpuPlatform = "Intel Broadwell")
        /// AND (scheduling.automaticRestart = true)
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The google project in which addresses are listed.
        /// Defaults to provider's configuration if missing.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region that should be considered to search addresses.
        /// All regions are considered if missing.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAddressesResult {
        /// A list of addresses matching the filter. Structure is defined below.
        pub addresses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetAddressesAddress>,
        >,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region in which the address resides.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAddressesArgs,
    ) -> GetAddressesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getAddresses:getAddresses".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
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
        GetAddressesResult {
            addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addresses"),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
        }
    }
}
