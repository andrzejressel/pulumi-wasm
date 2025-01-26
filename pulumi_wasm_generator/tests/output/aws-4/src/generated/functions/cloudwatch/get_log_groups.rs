pub mod get_log_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogGroupsArgs {
        /// Group prefix of the Cloudwatch log groups to list
        #[builder(into, default)]
        pub log_group_name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogGroupsResult {
        /// Set of ARNs of the Cloudwatch log groups
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_group_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of names of the Cloudwatch log groups
        pub log_group_names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLogGroupsArgs,
    ) -> GetLogGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_group_name_prefix_binding = args
            .log_group_name_prefix
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudwatch/getLogGroups:getLogGroups".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logGroupNamePrefix".into(),
                    value: &log_group_name_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLogGroupsResult {
            arns: pulumi_wasm_rust::__private::into_domain(o.extract_field("arns")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            log_group_name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logGroupNamePrefix"),
            ),
            log_group_names: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logGroupNames"),
            ),
        }
    }
}
