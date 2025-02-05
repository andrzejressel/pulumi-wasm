pub mod get_code_signing_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCodeSigningConfigArgs {
        /// ARN of the code signing configuration.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCodeSigningConfigResult {
        /// List of allowed publishers as signing profiles for this code signing configuration.
        pub allowed_publishers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetCodeSigningConfigAllowedPublisher>,
        >,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the code signing configuration.
        pub config_id: pulumi_wasm_rust::Output<String>,
        /// Code signing configuration description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date and time that the code signing configuration was last modified.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        /// List of code signing policies that control the validation failure action for signature mismatch or expiry.
        pub policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::lambda::GetCodeSigningConfigPolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCodeSigningConfigArgs,
    ) -> GetCodeSigningConfigResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getCodeSigningConfig:getCodeSigningConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCodeSigningConfigResult {
            allowed_publishers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedPublishers"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            config_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policies"),
            ),
        }
    }
}
