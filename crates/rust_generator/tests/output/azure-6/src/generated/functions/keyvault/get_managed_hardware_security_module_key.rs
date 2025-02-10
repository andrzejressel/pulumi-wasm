#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_hardware_security_module_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleKeyArgs {
        /// Specifies the ID of the Managed Hardware Security Module instance where the Secret resides, available on the `azure.keyvault.ManagedHardwareSecurityModuleKey` Data Source / Resource.
        ///
        /// **NOTE:** The Managed Hardware Security Module must be in the same subscription as the provider. If the Managed Hardware Security Module is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub managed_hsm_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Managed Hardware Security Module Key.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleKeyResult {
        /// The EC Curve name of this Managed Hardware Security Module Key.
        pub curve: pulumi_gestalt_rust::Output<String>,
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of JSON web key operations assigned to this Managed Hardware Security Module Key
        pub key_opts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the Size of this Managed Hardware Security Module Key.
        pub key_size: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the Key Type of this Managed Hardware Security Module Key
        pub key_type: pulumi_gestalt_rust::Output<String>,
        pub managed_hsm_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub not_before_date: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to this Managed Hardware Security Module Key.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The current version of the Managed Hardware Security Module Key.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The versioned ID of the Managed Hardware Security Module Key.
        pub versioned_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedHardwareSecurityModuleKeyArgs,
    ) -> GetManagedHardwareSecurityModuleKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleKey:getManagedHardwareSecurityModuleKey"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedHsmId".into(),
                    value: managed_hsm_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetManagedHardwareSecurityModuleKeyResult {
            curve: o.get_field("curve"),
            expiration_date: o.get_field("expirationDate"),
            id: o.get_field("id"),
            key_opts: o.get_field("keyOpts"),
            key_size: o.get_field("keySize"),
            key_type: o.get_field("keyType"),
            managed_hsm_id: o.get_field("managedHsmId"),
            name: o.get_field("name"),
            not_before_date: o.get_field("notBeforeDate"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
            versioned_id: o.get_field("versionedId"),
        }
    }
}
