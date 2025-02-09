/// Manages a Managed HSM Key rotation policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_hardware_security_module_key_rotation_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyRotationPolicyArgs {
        /// Specify the expiration duration on a newly rotated key as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `P28D`.
        #[builder(into)]
        pub expire_after: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Managed HSM Key. Changing this forces a new Managed HSM Key rotation policy to be created.
        #[builder(into)]
        pub managed_hsm_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Rotate automatically at a duration after key creation as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        #[builder(into, default)]
        pub time_after_creation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rotate automatically at a duration before key expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        #[builder(into, default)]
        pub time_before_expiry: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyRotationPolicyResult {
        /// Specify the expiration duration on a newly rotated key as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `P28D`.
        pub expire_after: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Managed HSM Key. Changing this forces a new Managed HSM Key rotation policy to be created.
        pub managed_hsm_key_id: pulumi_gestalt_rust::Output<String>,
        /// Rotate automatically at a duration after key creation as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        pub time_after_creation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Rotate automatically at a duration before key expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). Exactly one of `time_after_creation` or `time_before_expiry` should be specified.
        pub time_before_expiry: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHardwareSecurityModuleKeyRotationPolicyArgs,
    ) -> ManagedHardwareSecurityModuleKeyRotationPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let expire_after_binding = args.expire_after.get_output(context);
        let managed_hsm_key_id_binding = args.managed_hsm_key_id.get_output(context);
        let time_after_creation_binding = args.time_after_creation.get_output(context);
        let time_before_expiry_binding = args.time_before_expiry.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleKeyRotationPolicy:ManagedHardwareSecurityModuleKeyRotationPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expireAfter".into(),
                    value: expire_after_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedHsmKeyId".into(),
                    value: managed_hsm_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeAfterCreation".into(),
                    value: time_after_creation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeBeforeExpiry".into(),
                    value: time_before_expiry_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedHardwareSecurityModuleKeyRotationPolicyResult {
            expire_after: o.get_field("expireAfter"),
            managed_hsm_key_id: o.get_field("managedHsmKeyId"),
            time_after_creation: o.get_field("timeAfterCreation"),
            time_before_expiry: o.get_field("timeBeforeExpiry"),
        }
    }
}
