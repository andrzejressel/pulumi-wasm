/// Access Keys Configuration defines the rotation policy for the keys
/// that access will use to sign data.
pub mod zero_trust_key_access_key_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustKeyAccessKeyConfigurationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Number of days to trigger a rotation of the keys.
        #[builder(into, default)]
        pub key_rotation_interval_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustKeyAccessKeyConfigurationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Number of days to trigger a rotation of the keys.
        pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustKeyAccessKeyConfigurationArgs,
    ) -> ZeroTrustKeyAccessKeyConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let key_rotation_interval_days_binding = args
            .key_rotation_interval_days
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustKeyAccessKeyConfiguration:ZeroTrustKeyAccessKeyConfiguration"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyRotationIntervalDays".into(),
                    value: &key_rotation_interval_days_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustKeyAccessKeyConfigurationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            key_rotation_interval_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyRotationIntervalDays"),
            ),
        }
    }
}
