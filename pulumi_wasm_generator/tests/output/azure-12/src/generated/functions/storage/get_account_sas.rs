pub mod get_account_sas {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountSasArgs {
        /// The connection string for the storage account to which this SAS applies. Typically directly from the `primary_connection_string` attribute of a `azure.storage.Account` resource.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        ///
        /// > **NOTE:** The [ISO-8601 Time offset from UTC](https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC) is currently not supported by the service, which will result into 409 error.
        #[builder(into)]
        pub expiry: pulumi_wasm_rust::Output<String>,
        /// Only permit `https` access. If `false`, both `http` and `https` are permitted. Defaults to `true`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// IP address, or a range of IP addresses, from which to accept requests. When specifying a range, note that the range is inclusive.
        #[builder(into, default)]
        pub ip_addresses: pulumi_wasm_rust::Output<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into)]
        pub permissions: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasPermissions,
        >,
        /// A `resource_types` block as defined below.
        #[builder(into)]
        pub resource_types: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasResourceTypes,
        >,
        /// A `services` block as defined below.
        #[builder(into)]
        pub services: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasServices,
        >,
        /// Specifies the signed storage service version to use to authorize requests made with this account SAS. Defaults to `2017-07-29`.
        #[builder(into, default)]
        pub signed_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The starting time and date of validity of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub start: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountSasResult {
        pub connection_string: pulumi_wasm_rust::Output<String>,
        pub expiry: pulumi_wasm_rust::Output<String>,
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_addresses: pulumi_wasm_rust::Output<Option<String>>,
        pub permissions: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasPermissions,
        >,
        pub resource_types: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasResourceTypes,
        >,
        /// The computed Account Shared Access Signature (SAS).
        pub sas: pulumi_wasm_rust::Output<String>,
        pub services: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountSasServices,
        >,
        pub signed_version: pulumi_wasm_rust::Output<Option<String>>,
        pub start: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAccountSasArgs) -> GetAccountSasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_string_binding = args.connection_string.get_inner();
        let expiry_binding = args.expiry.get_inner();
        let https_only_binding = args.https_only.get_inner();
        let ip_addresses_binding = args.ip_addresses.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let resource_types_binding = args.resource_types.get_inner();
        let services_binding = args.services.get_inner();
        let signed_version_binding = args.signed_version.get_inner();
        let start_binding = args.start.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getAccountSAS:getAccountSAS".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "expiry".into(),
                    value: &expiry_binding,
                },
                register_interface::ObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddresses".into(),
                    value: &ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
                register_interface::ObjectField {
                    name: "services".into(),
                    value: &services_binding,
                },
                register_interface::ObjectField {
                    name: "signedVersion".into(),
                    value: &signed_version_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "expiry".into(),
                },
                register_interface::ResultField {
                    name: "httpsOnly".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddresses".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypes".into(),
                },
                register_interface::ResultField {
                    name: "sas".into(),
                },
                register_interface::ResultField {
                    name: "services".into(),
                },
                register_interface::ResultField {
                    name: "signedVersion".into(),
                },
                register_interface::ResultField {
                    name: "start".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAccountSasResult {
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            expiry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiry").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddresses").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypes").unwrap(),
            ),
            sas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sas").unwrap(),
            ),
            services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("services").unwrap(),
            ),
            signed_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signedVersion").unwrap(),
            ),
            start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("start").unwrap(),
            ),
        }
    }
}
