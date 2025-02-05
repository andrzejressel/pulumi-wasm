pub mod get_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Friendly IAM group name to match.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// User ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Stable and unique string identifying the group.
        pub group_id: pulumi_wasm_rust::Output<String>,
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Path to the IAM user.
        pub path: pulumi_wasm_rust::Output<String>,
        /// List of objects containing group member information. See below.
        pub users: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::iam::GetGroupUser>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_name_binding = args.group_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("groupId"),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("groupName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            users: pulumi_wasm_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
