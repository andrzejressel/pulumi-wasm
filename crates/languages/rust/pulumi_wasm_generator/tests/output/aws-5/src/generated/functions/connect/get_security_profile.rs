pub mod get_security_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityProfileArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific Security Profile by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific Security Profile by Security Profile id
        #[builder(into, default)]
        pub security_profile_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Security Profile.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecurityProfileResult {
        /// ARN of the Security Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the Security Profile.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization resource identifier for the security profile.
        pub organization_resource_id: pulumi_wasm_rust::Output<String>,
        /// List of permissions assigned to the security profile.
        pub permissions: pulumi_wasm_rust::Output<Vec<String>>,
        pub security_profile_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the Security Profile.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSecurityProfileArgs,
    ) -> GetSecurityProfileResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let security_profile_id_binding = args
            .security_profile_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getSecurityProfile:getSecurityProfile".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "securityProfileId".into(),
                    value: &security_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecurityProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization_resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organizationResourceId"),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            security_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityProfileId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
