pub mod get_account_blob_container_sas {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountBlobContainerSasArgs {
        /// The `Cache-Control` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string for the storage account to which this SAS applies. Typically directly from the `primary_connection_string` attribute of an `azure.storage.Account` resource.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// Name of the container.
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The `Content-Disposition` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_disposition: pulumi_wasm_rust::Output<Option<String>>,
        /// The `Content-Encoding` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_encoding: pulumi_wasm_rust::Output<Option<String>>,
        /// The `Content-Language` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_language: pulumi_wasm_rust::Output<Option<String>>,
        /// The `Content-Type` response header that is sent when this SAS token is used.
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        ///
        /// > **NOTE:** The [ISO-8601 Time offset from UTC](https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC) is currently not supported by the service, which will result into 409 error.
        #[builder(into)]
        pub expiry: pulumi_wasm_rust::Output<String>,
        /// Only permit `https` access. If `false`, both `http` and `https` are permitted. Defaults to `true`.
        #[builder(into, default)]
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Single IPv4 address or range (connected with a dash) of IPv4 addresses.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into)]
        pub permissions: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountBlobContainerSasPermissions,
        >,
        /// The starting time and date of validity of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub start: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountBlobContainerSasResult {
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        pub connection_string: pulumi_wasm_rust::Output<String>,
        pub container_name: pulumi_wasm_rust::Output<String>,
        pub content_disposition: pulumi_wasm_rust::Output<Option<String>>,
        pub content_encoding: pulumi_wasm_rust::Output<Option<String>>,
        pub content_language: pulumi_wasm_rust::Output<Option<String>>,
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        pub expiry: pulumi_wasm_rust::Output<String>,
        pub https_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        pub permissions: pulumi_wasm_rust::Output<
            super::super::super::types::storage::GetAccountBlobContainerSasPermissions,
        >,
        /// The computed Blob Container Shared Access Signature (SAS). The delimiter character ('?') for the query string is the prefix of `sas`.
        pub sas: pulumi_wasm_rust::Output<String>,
        pub start: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetAccountBlobContainerSasArgs,
    ) -> GetAccountBlobContainerSasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cache_control_binding = args.cache_control.get_inner();
        let connection_string_binding = args.connection_string.get_inner();
        let container_name_binding = args.container_name.get_inner();
        let content_disposition_binding = args.content_disposition.get_inner();
        let content_encoding_binding = args.content_encoding.get_inner();
        let content_language_binding = args.content_language.get_inner();
        let content_type_binding = args.content_type.get_inner();
        let expiry_binding = args.expiry.get_inner();
        let https_only_binding = args.https_only.get_inner();
        let ip_address_binding = args.ip_address.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let start_binding = args.start.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getAccountBlobContainerSAS:getAccountBlobContainerSAS"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheControl".into(),
                    value: &cache_control_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "contentDisposition".into(),
                    value: &content_disposition_binding,
                },
                register_interface::ObjectField {
                    name: "contentEncoding".into(),
                    value: &content_encoding_binding,
                },
                register_interface::ObjectField {
                    name: "contentLanguage".into(),
                    value: &content_language_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
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
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cacheControl".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "contentDisposition".into(),
                },
                register_interface::ResultField {
                    name: "contentEncoding".into(),
                },
                register_interface::ResultField {
                    name: "contentLanguage".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
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
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "sas".into(),
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
        GetAccountBlobContainerSasResult {
            cache_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheControl").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            content_disposition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentDisposition").unwrap(),
            ),
            content_encoding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentEncoding").unwrap(),
            ),
            content_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentLanguage").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            expiry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiry").unwrap(),
            ),
            https_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsOnly").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            sas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sas").unwrap(),
            ),
            start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("start").unwrap(),
            ),
        }
    }
}
