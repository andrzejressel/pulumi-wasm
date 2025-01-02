pub mod tree {
    pub mod v1 {
        pub mod nursery {
            #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
            #[builder(finish_fn = build_struct)]
            #[allow(dead_code)]
            pub struct NurseryArgs {
                /// The sizes of trees available
                #[builder(into, default)]
                pub sizes: pulumi_wasm_rust::Output<
                    Option<
                        std::collections::HashMap<
                            String,
                            super::super::super::types::tree::v1::TreeSize,
                        >,
                    >,
                >,
                /// The varieties available
                #[builder(into)]
                pub varieties: pulumi_wasm_rust::Output<
                    Vec<super::super::super::types::tree::v1::RubberTreeVariety>,
                >,
            }
            ///
            /// Registers a new resource with the given unique name and arguments
            ///
            #[allow(non_snake_case, unused_imports, dead_code)]
            pub fn create(name: &str, args: NurseryArgs) {
                use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
                use std::collections::HashMap;
                let sizes_binding = args.sizes.get_inner();
                let varieties_binding = args.varieties.get_inner();
                let request = register_interface::RegisterResourceRequest {
                    type_: "plant:tree/v1:Nursery".into(),
                    name: name.to_string(),
                    object: Vec::from([
                        register_interface::ObjectField {
                            name: "sizes".into(),
                            value: &sizes_binding,
                        },
                        register_interface::ObjectField {
                            name: "varieties".into(),
                            value: &varieties_binding,
                        },
                    ]),
                    results: Vec::from([]),
                };
                register_interface::register(&request);
            }
        }
        pub mod rubber_tree {
            #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
            #[builder(finish_fn = build_struct)]
            #[allow(dead_code)]
            pub struct RubberTreeArgs {
                #[builder(into, default)]
                pub container: pulumi_wasm_rust::Output<
                    Option<super::super::super::types::Container>,
                >,
                #[builder(into)]
                pub diameter: pulumi_wasm_rust::Output<
                    super::super::super::types::tree::v1::Diameter,
                >,
                #[builder(into, default)]
                pub farm: pulumi_wasm_rust::Output<
                    Option<
                        pulumi_wasm_rust::OneOf2<
                            super::super::super::types::tree::v1::Farm,
                            String,
                        >,
                    >,
                >,
                #[builder(into, default)]
                pub size: pulumi_wasm_rust::Output<
                    Option<super::super::super::types::tree::v1::TreeSize>,
                >,
                #[builder(into)]
                pub type_: pulumi_wasm_rust::Output<
                    super::super::super::types::tree::v1::RubberTreeVariety,
                >,
            }
            #[allow(dead_code)]
            pub struct RubberTreeResult {
                pub container: pulumi_wasm_rust::Output<
                    Option<super::super::super::types::Container>,
                >,
                pub diameter: pulumi_wasm_rust::Output<
                    super::super::super::types::tree::v1::Diameter,
                >,
                pub farm: pulumi_wasm_rust::Output<
                    Option<
                        pulumi_wasm_rust::OneOf2<
                            super::super::super::types::tree::v1::Farm,
                            String,
                        >,
                    >,
                >,
                pub size: pulumi_wasm_rust::Output<
                    Option<super::super::super::types::tree::v1::TreeSize>,
                >,
                pub type_: pulumi_wasm_rust::Output<
                    super::super::super::types::tree::v1::RubberTreeVariety,
                >,
            }
            ///
            /// Registers a new resource with the given unique name and arguments
            ///
            #[allow(non_snake_case, unused_imports, dead_code)]
            pub fn create(name: &str, args: RubberTreeArgs) -> RubberTreeResult {
                use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
                use std::collections::HashMap;
                let container_binding = args.container.get_inner();
                let diameter_binding = args.diameter.get_inner();
                let farm_binding = args.farm.get_inner();
                let size_binding = args.size.get_inner();
                let type__binding = args.type_.get_inner();
                let request = register_interface::RegisterResourceRequest {
                    type_: "plant:tree/v1:RubberTree".into(),
                    name: name.to_string(),
                    object: Vec::from([
                        register_interface::ObjectField {
                            name: "container".into(),
                            value: &container_binding,
                        },
                        register_interface::ObjectField {
                            name: "diameter".into(),
                            value: &diameter_binding,
                        },
                        register_interface::ObjectField {
                            name: "farm".into(),
                            value: &farm_binding,
                        },
                        register_interface::ObjectField {
                            name: "size".into(),
                            value: &size_binding,
                        },
                        register_interface::ObjectField {
                            name: "type".into(),
                            value: &type__binding,
                        },
                    ]),
                    results: Vec::from([
                        register_interface::ResultField {
                            name: "container".into(),
                        },
                        register_interface::ResultField {
                            name: "diameter".into(),
                        },
                        register_interface::ResultField {
                            name: "farm".into(),
                        },
                        register_interface::ResultField {
                            name: "size".into(),
                        },
                        register_interface::ResultField {
                            name: "type".into(),
                        },
                    ]),
                };
                let o = register_interface::register(&request);
                let mut hashmap: HashMap<String, _> = o
                    .fields
                    .into_iter()
                    .map(|f| (f.name, f.output))
                    .collect();
                RubberTreeResult {
                    container: pulumi_wasm_rust::__private::into_domain(
                        hashmap.remove("container").unwrap(),
                    ),
                    diameter: pulumi_wasm_rust::__private::into_domain(
                        hashmap.remove("diameter").unwrap(),
                    ),
                    farm: pulumi_wasm_rust::__private::into_domain(
                        hashmap.remove("farm").unwrap(),
                    ),
                    size: pulumi_wasm_rust::__private::into_domain(
                        hashmap.remove("size").unwrap(),
                    ),
                    type_: pulumi_wasm_rust::__private::into_domain(
                        hashmap.remove("type").unwrap(),
                    ),
                }
            }
        }
    }
}
pub mod functions {}
pub mod types {
    pub mod tree {
        pub mod v1 {
            include!("types/tree/v1/diameter.rs");
            include!("types/tree/v1/farm.rs");
            include!("types/tree/v1/rubber_tree_variety.rs");
            include!("types/tree/v1/tree_size.rs");
        }
    }
    include!("types/cloud_audit_options_log_name.rs");
    include!("types/container.rs");
    include!("types/container_brightness.rs");
    include!("types/container_color.rs");
    include!("types/container_size.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-plant {
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
