pub mod functions {
    pub mod func_with_all_optional_inputs {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithAllOptionalInputsArgs {
            /// Property A
            #[builder(into, default)]
            pub a: pulumi_wasm_rust::Output<Option<String>>,
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
    pub mod func_with_const_input {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithConstInputArgs {
            #[builder(into, default)]
            pub plain_input: pulumi_wasm_rust::Output<
                Option<super::super::constants::ConstStringFixed>,
            >,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: FuncWithConstInputArgs) {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let plain_input_binding = args.plain_input.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithConstInput".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "plainInput".into(),
                        value: &plain_input_binding,
                    },
                ]),
                results: Vec::from([]),
            };
            register_interface::invoke(&request);
        }
    }
    pub mod func_with_default_value {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithDefaultValueArgs {
            #[builder(into)]
            pub a: pulumi_wasm_rust::Output<String>,
            #[builder(into, default)]
            pub b: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct FuncWithDefaultValueResult {
            pub r: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: FuncWithDefaultValueArgs) -> FuncWithDefaultValueResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let a_binding = args.a.get_inner();
            let b_binding = args.b.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithDefaultValue".into(),
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
            FuncWithDefaultValueResult {
                r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
            }
        }
    }
    pub mod func_with_dict_param {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithDictParamArgs {
            #[builder(into, default)]
            pub a: pulumi_wasm_rust::Output<
                Option<std::collections::HashMap<String, String>>,
            >,
            #[builder(into, default)]
            pub b: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct FuncWithDictParamResult {
            pub r: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: FuncWithDictParamArgs) -> FuncWithDictParamResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let a_binding = args.a.get_inner();
            let b_binding = args.b.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithDictParam".into(),
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
            FuncWithDictParamResult {
                r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
            }
        }
    }
    pub mod func_with_empty_outputs {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithEmptyOutputsArgs {
            /// The Name of the FeatureGroup.
            #[builder(into)]
            pub name: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: FuncWithEmptyOutputsArgs) {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let name_binding = args.name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithEmptyOutputs".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "name".into(),
                        value: &name_binding,
                    },
                ]),
                results: Vec::from([]),
            };
            register_interface::invoke(&request);
        }
    }
    pub mod func_with_list_param {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct FuncWithListParamArgs {
            #[builder(into, default)]
            pub a: pulumi_wasm_rust::Output<Option<Vec<String>>>,
            #[builder(into, default)]
            pub b: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct FuncWithListParamResult {
            pub r: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: FuncWithListParamArgs) -> FuncWithListParamResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let a_binding = args.a.get_inner();
            let b_binding = args.b.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::funcWithListParam".into(),
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
            FuncWithListParamResult {
                r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
            }
        }
    }
    pub mod get_bastion_shareable_link {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetBastionShareableLinkArgs {
            /// The name of the Bastion Host.
            #[builder(into)]
            pub bastion_host_name: pulumi_wasm_rust::Output<String>,
            /// The name of the resource group.
            #[builder(into)]
            pub resource_group_name: pulumi_wasm_rust::Output<String>,
            /// List of VM references.
            #[builder(into, default)]
            pub vms: pulumi_wasm_rust::Output<
                Option<Vec<super::super::types::BastionShareableLink>>,
            >,
        }
        #[allow(dead_code)]
        pub struct GetBastionShareableLinkResult {
            /// The URL to get the next set of results.
            pub next_link: pulumi_wasm_rust::Output<Option<String>>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(
            args: GetBastionShareableLinkArgs,
        ) -> GetBastionShareableLinkResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let bastion_host_name_binding = args.bastion_host_name.get_inner();
            let resource_group_name_binding = args.resource_group_name.get_inner();
            let vms_binding = args.vms.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::getBastionShareableLink".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "bastionHostName".into(),
                        value: &bastion_host_name_binding,
                    },
                    register_interface::ObjectField {
                        name: "resourceGroupName".into(),
                        value: &resource_group_name_binding,
                    },
                    register_interface::ObjectField {
                        name: "vms".into(),
                        value: &vms_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "nextLink".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetBastionShareableLinkResult {
                next_link: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("nextLink").unwrap(),
                ),
            }
        }
    }
    pub mod get_client_config {
        #[allow(dead_code)]
        pub struct GetClientConfigResult {
            /// Azure Client ID (Application Object ID).
            pub client_id: pulumi_wasm_rust::Output<String>,
            /// Azure Object ID of the current user or service principal.
            pub object_id: pulumi_wasm_rust::Output<String>,
            /// Azure Subscription ID
            pub subscription_id: pulumi_wasm_rust::Output<String>,
            /// Azure Tenant ID
            pub tenant_id: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke() -> GetClientConfigResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::getClientConfig".into(),
                object: Vec::from([]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "clientId".into(),
                    },
                    register_interface::ResultField {
                        name: "objectId".into(),
                    },
                    register_interface::ResultField {
                        name: "subscriptionId".into(),
                    },
                    register_interface::ResultField {
                        name: "tenantId".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetClientConfigResult {
                client_id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("clientId").unwrap(),
                ),
                object_id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("objectId").unwrap(),
                ),
                subscription_id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("subscriptionId").unwrap(),
                ),
                tenant_id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("tenantId").unwrap(),
                ),
            }
        }
    }
    pub mod get_integration_runtime_object_metadatum {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetIntegrationRuntimeObjectMetadatumArgs {
            /// The factory name.
            #[builder(into)]
            pub factory_name: pulumi_wasm_rust::Output<String>,
            /// The integration runtime name.
            #[builder(into)]
            pub integration_runtime_name: pulumi_wasm_rust::Output<String>,
            /// Metadata path.
            #[builder(into, default)]
            pub metadata_path: pulumi_wasm_rust::Output<Option<String>>,
            /// The resource group name.
            #[builder(into)]
            pub resource_group_name: pulumi_wasm_rust::Output<String>,
        }
        #[allow(dead_code)]
        pub struct GetIntegrationRuntimeObjectMetadatumResult {
            /// The link to the next page of results, if any remaining results exist.
            pub next_link: pulumi_wasm_rust::Output<Option<String>>,
            /// List of SSIS object metadata.
            pub value: pulumi_wasm_rust::Output<
                Option<
                    Vec<
                        pulumi_wasm_rust::OneOf4<
                            super::super::types::SsisEnvironmentResponse,
                            super::super::types::SsisFolderResponse,
                            super::super::types::SsisPackageResponse,
                            super::super::types::SsisProjectResponse,
                        >,
                    >,
                >,
            >,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(
            args: GetIntegrationRuntimeObjectMetadatumArgs,
        ) -> GetIntegrationRuntimeObjectMetadatumResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let factory_name_binding = args.factory_name.get_inner();
            let integration_runtime_name_binding = args
                .integration_runtime_name
                .get_inner();
            let metadata_path_binding = args.metadata_path.get_inner();
            let resource_group_name_binding = args.resource_group_name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::getIntegrationRuntimeObjectMetadatum".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "factoryName".into(),
                        value: &factory_name_binding,
                    },
                    register_interface::ObjectField {
                        name: "integrationRuntimeName".into(),
                        value: &integration_runtime_name_binding,
                    },
                    register_interface::ObjectField {
                        name: "metadataPath".into(),
                        value: &metadata_path_binding,
                    },
                    register_interface::ObjectField {
                        name: "resourceGroupName".into(),
                        value: &resource_group_name_binding,
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
            GetIntegrationRuntimeObjectMetadatumResult {
                next_link: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("nextLink").unwrap(),
                ),
                value: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("value").unwrap(),
                ),
            }
        }
    }
    pub mod list_storage_account_keys {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct ListStorageAccountKeysArgs {
            /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
            #[builder(into)]
            pub account_name: pulumi_wasm_rust::Output<String>,
            /// Specifies type of the key to be listed. Possible value is kerb.
            #[builder(into, default)]
            pub expand: pulumi_wasm_rust::Output<Option<String>>,
            /// The name of the resource group within the user's subscription. The name is case insensitive.
            #[builder(into)]
            pub resource_group_name: pulumi_wasm_rust::Output<String>,
        }
        #[allow(dead_code)]
        pub struct ListStorageAccountKeysResult {
            /// Gets the list of storage account keys and their properties for the specified storage account.
            pub keys: pulumi_wasm_rust::Output<
                Vec<super::super::types::StorageAccountKeyResponse>,
            >,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: ListStorageAccountKeysArgs) -> ListStorageAccountKeysResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let account_name_binding = args.account_name.get_inner();
            let expand_binding = args.expand.get_inner();
            let resource_group_name_binding = args.resource_group_name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "mypkg::listStorageAccountKeys".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "accountName".into(),
                        value: &account_name_binding,
                    },
                    register_interface::ObjectField {
                        name: "expand".into(),
                        value: &expand_binding,
                    },
                    register_interface::ObjectField {
                        name: "resourceGroupName".into(),
                        value: &resource_group_name_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "keys".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            ListStorageAccountKeysResult {
                keys: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("keys").unwrap(),
                ),
            }
        }
    }
}
pub mod types {
    include!("types/bastion_shareable_link.rs");
    include!("types/ssis_environment_reference_response.rs");
    include!("types/ssis_environment_response.rs");
    include!("types/ssis_folder_response.rs");
    include!("types/ssis_package_response.rs");
    include!("types/ssis_parameter_response.rs");
    include!("types/ssis_project_response.rs");
    include!("types/ssis_variable_response.rs");
    include!("types/storage_account_key_response.rs");
}
#[doc(hidden)]
pub mod constants {
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringEnvironment, "Environment"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringFixed, "Fixed"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringFolder, "Folder"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringPackage, "Package"
    );
    pulumi_wasm_rust::__private::constant::generate_string_const!(
        ConstStringProject, "Project"
    );
}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-mypkg {
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
