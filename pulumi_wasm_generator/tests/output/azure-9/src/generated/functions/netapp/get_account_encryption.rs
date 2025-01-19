pub mod get_account_encryption {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountEncryptionArgs {
        /// The ID of the NetApp account where customer managed keys-based encryption is enabled.
        #[builder(into)]
        pub netapp_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountEncryptionResult {
        pub encryption_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub netapp_account_id: pulumi_wasm_rust::Output<String>,
        pub system_assigned_identity_principal_id: pulumi_wasm_rust::Output<String>,
        pub user_assigned_identity_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountEncryptionArgs) -> GetAccountEncryptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let netapp_account_id_binding = args.netapp_account_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getAccountEncryption:getAccountEncryption".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "netappAccountId".into(),
                    value: &netapp_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "netappAccountId".into(),
                },
                register_interface::ResultField {
                    name: "systemAssignedIdentityPrincipalId".into(),
                },
                register_interface::ResultField {
                    name: "userAssignedIdentityId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountEncryptionResult {
            encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            netapp_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("netappAccountId").unwrap(),
            ),
            system_assigned_identity_principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemAssignedIdentityPrincipalId").unwrap(),
            ),
            user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userAssignedIdentityId").unwrap(),
            ),
        }
    }
}
