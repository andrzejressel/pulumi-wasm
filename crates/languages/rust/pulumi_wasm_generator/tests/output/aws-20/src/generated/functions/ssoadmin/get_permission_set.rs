pub mod get_permission_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionSetArgs {
        /// ARN of the permission set.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the SSO Instance associated with the permission set.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the SSO Permission Set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPermissionSetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the Permission Set.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Relay state URL used to redirect users within the application during the federation authentication process.
        pub relay_state: pulumi_wasm_rust::Output<String>,
        /// Length of time that the application user sessions are valid in the ISO-8601 standard.
        pub session_duration: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPermissionSetArgs,
    ) -> GetPermissionSetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getPermissionSet:getPermissionSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPermissionSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            relay_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("relayState"),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sessionDuration"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
