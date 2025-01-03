pub mod foo {
    //! test new feature with resoruces
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FooArgs {
        #[builder(into, default)]
        pub argument: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into)]
        pub backup_kube_client_settings: pulumi_wasm_rust::Output<
            super::types::KubeClientSettings,
        >,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into, default)]
        pub kube_client_settings: pulumi_wasm_rust::Output<
            Option<super::types::KubeClientSettings>,
        >,
        /// describing things
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<Option<super::types::LayeredType>>,
    }
    #[allow(dead_code)]
    pub struct FooResult {
        /// A test for plain types
        pub default_kube_client_settings: pulumi_wasm_rust::Output<
            Option<super::types::KubeClientSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FooArgs) -> FooResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let argument_binding = args.argument.get_inner();
        let backup_kube_client_settings_binding = args
            .backup_kube_client_settings
            .get_inner();
        let kube_client_settings_binding = args.kube_client_settings.get_inner();
        let settings_binding = args.settings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:Foo".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "argument".into(),
                    value: &argument_binding,
                },
                register_interface::ObjectField {
                    name: "backupKubeClientSettings".into(),
                    value: &backup_kube_client_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kubeClientSettings".into(),
                    value: &kube_client_settings_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultKubeClientSettings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FooResult {
            default_kube_client_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultKubeClientSettings").unwrap(),
            ),
        }
    }
}
pub mod module_test {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct moduleTestArgs {
        #[builder(into, default)]
        pub mod1: pulumi_wasm_rust::Output<Option<super::types::mod1::Typ>>,
        #[builder(into, default)]
        pub val: pulumi_wasm_rust::Output<Option<super::types::Typ>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: moduleTestArgs) {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mod1_binding = args.mod1.get_inner();
        let val_binding = args.val.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:moduleTest".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mod1".into(),
                    value: &mod1_binding,
                },
                register_interface::ObjectField {
                    name: "val".into(),
                    value: &val_binding,
                },
            ]),
            results: Vec::from([]),
        };
        register_interface::register(&request);
    }
}
pub mod functions {
    pub mod func_with_all_optional_inputs {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithAllOptionalInputsArgs {
            /// Property A
            #[builder(into, default)]
            pub a: pulumi_wasm_rust::Output<
                Option<super::super::types::HelmReleaseSettings>,
            >,
            /// Property B
            #[builder(into, default)]
            pub b: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct FuncWithAllOptionalInputsResult {
            pub r: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(
            args: FuncWithAllOptionalInputsArgs,
        ) -> FuncWithAllOptionalInputsResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let a_binding = args.a.get_inner();
            let b_binding = args.b.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithAllOptionalInputs".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "a".into(),
                        value: &a_binding,
                    },
                    register_interface::ObjectField {
                        name: "b".into(),
                        value: &b_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "r".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            FuncWithAllOptionalInputsResult {
                r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
            }
        }
    }
}
pub mod types {
    pub mod mod1 {
        include!("types/mod1/typ.rs");
    }
    pub mod mod2 {
        include!("types/mod2/typ.rs");
    }
    include!("types/helm_release_settings.rs");
    include!("types/kube_client_settings.rs");
    include!("types/layered_type.rs");
    include!("types/typ.rs");
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
