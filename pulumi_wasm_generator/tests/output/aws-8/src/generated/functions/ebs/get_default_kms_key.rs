pub mod get_default_kms_key {
    #[allow(dead_code)]
    pub struct GetDefaultKmsKeyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the default KMS key uses to encrypt an EBS volume in this region when no key is specified in an API call that creates the volume and encryption by default is enabled.
        pub key_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetDefaultKmsKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ebs/getDefaultKmsKey:getDefaultKmsKey".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDefaultKmsKeyResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("keyArn")),
        }
    }
}
