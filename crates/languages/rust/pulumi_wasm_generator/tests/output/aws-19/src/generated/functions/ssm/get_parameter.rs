pub mod get_parameter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetParameterArgs {
        /// Name of the parameter.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether to return decrypted `SecureString` value. Defaults to `true`.
        ///
        /// In addition to all arguments above, the following attributes are exported:
        #[builder(into, default)]
        pub with_decryption: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetParameterResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub insecure_value: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<i32>,
        pub with_decryption: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetParameterArgs,
    ) -> GetParameterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let with_decryption_binding = args
            .with_decryption
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getParameter:getParameter".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "withDecryption".into(),
                    value: &with_decryption_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetParameterResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            insecure_value: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("insecureValue"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            with_decryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("withDecryption"),
            ),
        }
    }
}
