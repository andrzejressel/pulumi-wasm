pub mod get_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAliasArgs {
        /// Display name of the alias. The name must start with the word "alias" followed by a forward slash (alias/)
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAliasResult {
        /// Amazon Resource Name(ARN) of the key alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the alias
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN pointed to by the alias.
        pub target_key_arn: pulumi_wasm_rust::Output<String>,
        /// Key identifier pointed to by the alias.
        pub target_key_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAliasArgs,
    ) -> GetAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getAlias:getAlias".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            target_key_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetKeyArn"),
            ),
            target_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetKeyId"),
            ),
        }
    }
}
