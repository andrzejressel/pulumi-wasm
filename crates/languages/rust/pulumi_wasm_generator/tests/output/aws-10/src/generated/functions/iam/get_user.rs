pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// Map of key-value pairs associated with the user.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Friendly IAM user name to match.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// ARN assigned by AWS for this user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Path in which this user was created.
        pub path: pulumi_wasm_rust::Output<String>,
        /// The ARN of the policy that is used to set the permissions boundary for the user.
        pub permissions_boundary: pulumi_wasm_rust::Output<String>,
        /// Map of key-value pairs associated with the user.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Unique ID assigned by AWS for this user.
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// Name associated to this User
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            permissions_boundary: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissionsBoundary"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
