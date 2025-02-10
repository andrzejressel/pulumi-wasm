#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_addresses {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The google project in which addresses are listed.
        /// Defaults to provider's configuration if missing.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region that should be considered to search addresses.
        /// All regions are considered if missing.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAddressesResult {
        /// A list of addresses matching the filter. Structure is defined below.
        pub addresses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetAddressesAddress>,
        >,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region in which the address resides.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAddressesArgs,
    ) -> GetAddressesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getAddresses:getAddresses".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
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
        GetAddressesResult {
            addresses: o.get_field("addresses"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
