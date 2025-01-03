pub mod impl_ {
    pub mod let_ {
        pub mod loop_ {
            pub mod type_ {
                #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
                #[builder(finish_fn = build_struct)]
                #[allow(dead_code)]
                pub struct TypeArgs {
                    #[builder(into, default)]
                    pub type_: pulumi_wasm_rust::Output<
                        Option<
                            super::super::super::super::types::impl_::let_::loop_::Type,
                        >,
                    >,
                }
                ///
                /// Registers a new resource with the given unique name and arguments
                ///
                #[allow(non_snake_case, unused_imports, dead_code)]
                pub fn create(name: &str, args: TypeArgs) {
                    use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
                    use std::collections::HashMap;
                    let type__binding = args.type_.get_inner();
                    let request = register_interface::RegisterResourceRequest {
                        type_: "example:impl/let/loop:Type".into(),
                        name: name.to_string(),
                        object: Vec::from([
                            register_interface::ObjectField {
                                name: "type".into(),
                                value: &type__binding,
                            },
                        ]),
                        results: Vec::from([]),
                    };
                    register_interface::register(&request);
                }
            }
        }
    }
}
pub mod functions {
    pub mod impl_ {
        pub mod let_ {
            pub mod loop_ {
                pub mod type_ {
                    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
                    #[builder(finish_fn = build_struct)]
                    #[allow(dead_code)]
                    pub struct TypeArgs {
                        #[builder(into, default)]
                        pub type_: pulumi_wasm_rust::Output<
                            Option<
                                super::super::super::super::super::types::impl_::let_::loop_::Type,
                            >,
                        >,
                    }
                    #[allow(dead_code)]
                    pub struct TypeResult {
                        pub type_: pulumi_wasm_rust::Output<Option<String>>,
                    }
                    ///
                    /// Registers a new resource with the given unique name and arguments
                    ///
                    #[allow(non_snake_case, unused_imports, dead_code)]
                    pub fn invoke(args: TypeArgs) -> TypeResult {
                        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
                        use std::collections::HashMap;
                        let type__binding = args.type_.get_inner();
                        let request = register_interface::ResourceInvokeRequest {
                            token: "example:impl/let/loop:Type".into(),
                            object: Vec::from([
                                register_interface::ObjectField {
                                    name: "type".into(),
                                    value: &type__binding,
                                },
                            ]),
                            results: Vec::from([
                                register_interface::ResultField {
                                    name: "type".into(),
                                },
                            ]),
                        };
                        let o = register_interface::invoke(&request);
                        let mut hashmap: HashMap<String, _> = o
                            .fields
                            .into_iter()
                            .map(|f| (f.name, f.output))
                            .collect();
                        TypeResult {
                            type_: pulumi_wasm_rust::__private::into_domain(
                                hashmap.remove("type").unwrap(),
                            ),
                        }
                    }
                }
            }
        }
    }
}
pub mod types {
    pub mod impl_ {
        pub mod let_ {
            pub mod loop_ {
                include!("types/impl_/let_/loop_/type.rs");
            }
        }
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-example {
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
