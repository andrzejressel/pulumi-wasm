pub mod get_managed_hardware_security_module_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleKeyArgs {
        /// Specifies the ID of the Managed Hardware Security Module instance where the Secret resides, available on the `azure.keyvault.ManagedHardwareSecurityModuleKey` Data Source / Resource.
        ///
        /// **NOTE:** The Managed Hardware Security Module must be in the same subscription as the provider. If the Managed Hardware Security Module is in another subscription, you must create an aliased provider for that subscription.
        #[builder(into)]
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Managed Hardware Security Module Key.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
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
        args: GetManagedHardwareSecurityModuleKeyArgs,
    ) -> GetManagedHardwareSecurityModuleKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding = args.managed_hsm_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleKey:getManagedHardwareSecurityModuleKey"
                .into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "curve".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyOpts".into(),
                },
                register_interface::ResultField {
                    name: "keySize".into(),
                },
                register_interface::ResultField {
                    name: "keyType".into(),
                },
                register_interface::ResultField {
                    name: "managedHsmId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notBeforeDate".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionedId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedHardwareSecurityModuleKeyResult {
            curve: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("curve").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyOpts").unwrap(),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySize").unwrap(),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyType").unwrap(),
            ),
            managed_hsm_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedHsmId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            not_before_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBeforeDate").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            versioned_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionedId").unwrap(),
            ),
        }
    }
}
