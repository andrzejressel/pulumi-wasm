pub mod get_sas {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSasArgs {
        /// The connection string for the Event Hub to which this SAS applies.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
        #[builder(into)]
        pub expiry: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSasResult {
        pub connection_string: pulumi_wasm_rust::Output<String>,
        pub expiry: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The computed Event Hub Shared Access Signature (SAS).
        pub sas: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSasArgs) -> GetSasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_string_binding = args.connection_string.get_inner();
        let expiry_binding = args.expiry.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getSas:getSas".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "expiry".into(),
                    value: &expiry_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "sas".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSasResult {
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            expiry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiry").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            sas: pulumi_wasm_rust::__private::into_domain(hashmap.remove("sas").unwrap()),
        }
    }
}
