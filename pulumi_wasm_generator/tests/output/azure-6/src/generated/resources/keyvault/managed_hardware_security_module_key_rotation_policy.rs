/// Manages a Managed HSM Key rotation policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = managed_hardware_security_module_key::create(
///         "example",
///         ManagedHardwareSecurityModuleKeyArgs::builder()
///             .curve("P-521")
///             .key_opts(vec!["sign",])
///             .key_type("EC-HSM")
///             .managed_hsm_id("${exampleAzurermKeyVaultManagedHardwareSecurityModule.id}")
///             .name("example-key")
///             .build_struct(),
///     );
///     let exampleManagedHardwareSecurityModuleKeyRotationPolicy = managed_hardware_security_module_key_rotation_policy::create(
///         "exampleManagedHardwareSecurityModuleKeyRotationPolicy",
///         ManagedHardwareSecurityModuleKeyRotationPolicyArgs::builder()
///             .expire_after("P60D")
///             .managed_hsm_key_id("${example.id}")
///             .time_before_expiry("P30D")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Managed HSM Key rotation policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModuleKeyRotationPolicy:ManagedHardwareSecurityModuleKeyRotationPolicy example https://example-hsm.managedhsm.azure.net/keys/example
/// ```
///
pub mod managed_hardware_security_module_key_rotation_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyRotationPolicyArgs {
        /// Specify the expiration duration on a newly rotated key as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `P28D`.
        #[builder(into)]
        pub expire_after: pulumi_wasm_rust::Output<String>,
        /// The ID of the Managed HSM Key. Changing this forces a new Managed HSM Key rotation policy to be created.
        #[builder(into)]
        pub managed_hsm_key_id: pulumi_wasm_rust::Output<String>,
        /// Rotate automatically at a duration after key creation as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        #[builder(into, default)]
        pub time_after_creation: pulumi_wasm_rust::Output<Option<String>>,
        /// Rotate automatically at a duration before key expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        #[builder(into, default)]
        pub time_before_expiry: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyRotationPolicyResult {
        /// Specify the expiration duration on a newly rotated key as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `P28D`.
        pub expire_after: pulumi_wasm_rust::Output<String>,
        /// The ID of the Managed HSM Key. Changing this forces a new Managed HSM Key rotation policy to be created.
        pub managed_hsm_key_id: pulumi_wasm_rust::Output<String>,
        /// Rotate automatically at a duration after key creation as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        pub time_after_creation: pulumi_wasm_rust::Output<Option<String>>,
        /// Rotate automatically at a duration before key expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        pub time_before_expiry: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedHardwareSecurityModuleKeyRotationPolicyArgs,
    ) -> ManagedHardwareSecurityModuleKeyRotationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let expire_after_binding = args.expire_after.get_inner();
        let managed_hsm_key_id_binding = args.managed_hsm_key_id.get_inner();
        let time_after_creation_binding = args.time_after_creation.get_inner();
        let time_before_expiry_binding = args.time_before_expiry.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleKeyRotationPolicy:ManagedHardwareSecurityModuleKeyRotationPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expireAfter".into(),
                    value: &expire_after_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmKeyId".into(),
                    value: &managed_hsm_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeAfterCreation".into(),
                    value: &time_after_creation_binding,
                },
                register_interface::ObjectField {
                    name: "timeBeforeExpiry".into(),
                    value: &time_before_expiry_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "expireAfter".into(),
                },
                register_interface::ResultField {
                    name: "managedHsmKeyId".into(),
                },
                register_interface::ResultField {
                    name: "timeAfterCreation".into(),
                },
                register_interface::ResultField {
                    name: "timeBeforeExpiry".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedHardwareSecurityModuleKeyRotationPolicyResult {
            expire_after: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireAfter").unwrap(),
            ),
            managed_hsm_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedHsmKeyId").unwrap(),
            ),
            time_after_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeAfterCreation").unwrap(),
            ),
            time_before_expiry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeBeforeExpiry").unwrap(),
            ),
        }
    }
}
