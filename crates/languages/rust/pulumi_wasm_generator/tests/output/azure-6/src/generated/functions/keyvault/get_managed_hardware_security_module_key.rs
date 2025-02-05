pub mod get_managed_hardware_security_module_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleKeyArgs {
        /// Specifies the ID of the Managed Hardware Security Module instance where the Secret resides, available on the `azure.keyvault.ManagedHardwareSecurityModuleKey` Data Source / Resource.
        ///
        /// **NOTE:** The Managed Hardware Security Module must be in the same subscription as the provider. If the Managed Hardware Security Module is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub managed_hsm_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Managed Hardware Security Module Key.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleKeyResult {
        /// The EC Curve name of this Managed Hardware Security Module Key.
        pub curve: pulumi_wasm_rust::Output<String>,
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of JSON web key operations assigned to this Managed Hardware Security Module Key
        pub key_opts: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Size of this Managed Hardware Security Module Key.
        pub key_size: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Key Type of this Managed Hardware Security Module Key
        pub key_type: pulumi_wasm_rust::Output<String>,
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub not_before_date: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to this Managed Hardware Security Module Key.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The current version of the Managed Hardware Security Module Key.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The versioned ID of the Managed Hardware Security Module Key.
        pub versioned_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetManagedHardwareSecurityModuleKeyArgs,
    ) -> GetManagedHardwareSecurityModuleKeyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleKey:getManagedHardwareSecurityModuleKey"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedHardwareSecurityModuleKeyResult {
            curve: pulumi_wasm_rust::__private::into_domain(o.extract_field("curve")),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationDate"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            key_opts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyOpts"),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            managed_hsm_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedHsmId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            not_before_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notBeforeDate"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            versioned_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionedId"),
            ),
        }
    }
}
