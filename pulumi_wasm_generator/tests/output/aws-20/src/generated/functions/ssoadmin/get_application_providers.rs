pub mod get_application_providers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationProvidersArgs {
        /// A list of application providers available in the current region. See `application_providers` below.
        #[builder(into, default)]
        pub application_providers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationProvidersApplicationProvider,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApplicationProvidersResult {
        /// A list of application providers available in the current region. See `application_providers` below.
        pub application_providers: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationProvidersApplicationProvider,
                >,
            >,
        >,
        /// AWS region.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApplicationProvidersArgs) -> GetApplicationProvidersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_providers_binding = args.application_providers.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getApplicationProviders:getApplicationProviders".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationProviders".into(),
                    value: &application_providers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationProviders".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApplicationProvidersResult {
            application_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationProviders").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
