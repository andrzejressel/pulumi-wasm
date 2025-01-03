pub mod functions {
    pub mod list_configurations {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct ListConfigurationsArgs {
            /// Holds details about product hierarchy information and filterable property.
            #[builder(into)]
            pub configuration_filters: pulumi_wasm_rust::Output<
                Vec<super::super::types::ConfigurationFilters>,
            >,
            /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
            #[builder(into, default)]
            pub customer_subscription_details: pulumi_wasm_rust::Output<
                Option<super::super::types::CustomerSubscriptionDetails>,
            >,
            /// $skipToken is supported on list of configurations, which provides the next page in the list of configurations.
            #[builder(into, default)]
            pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct ListConfigurationsResult {
            /// Link for the next set of configurations.
            pub next_link: pulumi_wasm_rust::Output<Option<String>>,
            /// List of configurations.
            pub value: pulumi_wasm_rust::Output<
                Vec<super::super::types::ConfigurationResponse>,
            >,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: ListConfigurationsArgs) -> ListConfigurationsResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let configuration_filters_binding = args.configuration_filters.get_inner();
            let customer_subscription_details_binding = args
                .customer_subscription_details
                .get_inner();
            let skip_token_binding = args.skip_token.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "myedgeorder::listConfigurations".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "configurationFilters".into(),
                        value: &configuration_filters_binding,
                    },
                    register_interface::ObjectField {
                        name: "customerSubscriptionDetails".into(),
                        value: &customer_subscription_details_binding,
                    },
                    register_interface::ObjectField {
                        name: "skipToken".into(),
                        value: &skip_token_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "nextLink".into(),
                    },
                    register_interface::ResultField {
                        name: "value".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            ListConfigurationsResult {
                next_link: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("nextLink").unwrap(),
                ),
                value: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("value").unwrap(),
                ),
            }
        }
    }
    pub mod list_product_families {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct ListProductFamiliesArgs {
            /// Customer subscription properties. Clients can display available products to unregistered customers by explicitly passing subscription details
            #[builder(into, default)]
            pub customer_subscription_details: pulumi_wasm_rust::Output<
                Option<super::super::types::CustomerSubscriptionDetails>,
            >,
            /// $expand is supported on configurations parameter for product, which provides details on the configurations for the product.
            #[builder(into, default)]
            pub expand: pulumi_wasm_rust::Output<Option<String>>,
            /// Dictionary of filterable properties on product family.
            #[builder(into)]
            pub filterable_properties: pulumi_wasm_rust::Output<
                std::collections::HashMap<
                    String,
                    Vec<super::super::types::FilterableProperty>,
                >,
            >,
            /// $skipToken is supported on list of product families, which provides the next page in the list of product families.
            #[builder(into, default)]
            pub skip_token: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct ListProductFamiliesResult {
            /// Link for the next set of product families.
            pub next_link: pulumi_wasm_rust::Output<Option<String>>,
            /// List of product families.
            pub value: pulumi_wasm_rust::Output<
                Vec<super::super::types::ProductFamilyResponse>,
            >,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: ListProductFamiliesArgs) -> ListProductFamiliesResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let customer_subscription_details_binding = args
                .customer_subscription_details
                .get_inner();
            let expand_binding = args.expand.get_inner();
            let filterable_properties_binding = args.filterable_properties.get_inner();
            let skip_token_binding = args.skip_token.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "myedgeorder::listProductFamilies".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "customerSubscriptionDetails".into(),
                        value: &customer_subscription_details_binding,
                    },
                    register_interface::ObjectField {
                        name: "expand".into(),
                        value: &expand_binding,
                    },
                    register_interface::ObjectField {
                        name: "filterableProperties".into(),
                        value: &filterable_properties_binding,
                    },
                    register_interface::ObjectField {
                        name: "skipToken".into(),
                        value: &skip_token_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "nextLink".into(),
                    },
                    register_interface::ResultField {
                        name: "value".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            ListProductFamiliesResult {
                next_link: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("nextLink").unwrap(),
                ),
                value: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("value").unwrap(),
                ),
            }
        }
    }
}
pub mod types {
    include!("types/availability_information_response.rs");
    include!("types/billing_meter_details_response.rs");
    include!("types/configuration_filters.rs");
    include!("types/configuration_response.rs");
    include!("types/cost_information_response.rs");
    include!("types/customer_subscription_details.rs");
    include!("types/customer_subscription_registered_features.rs");
    include!("types/description_response.rs");
    include!("types/dimensions_response.rs");
    include!("types/filterable_property.rs");
    include!("types/filterable_property_response.rs");
    include!("types/hierarchy_information.rs");
    include!("types/hierarchy_information_response.rs");
    include!("types/image_information_response.rs");
    include!("types/link_response.rs");
    include!("types/pav_2_meter_details_response.rs");
    include!("types/product_family_response.rs");
    include!("types/product_line_response.rs");
    include!("types/product_response.rs");
    include!("types/purchase_meter_details_response.rs");
    include!("types/specification_response.rs");
    include!("types/supported_filter_types.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPav2, "Pav2"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPurchase, "Purchase"
    );
}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-myedgeorder {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
