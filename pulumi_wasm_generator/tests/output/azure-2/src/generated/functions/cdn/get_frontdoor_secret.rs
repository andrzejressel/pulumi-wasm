pub mod get_frontdoor_secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorSecretArgs {
        /// Specifies the name of the Front Door Secret.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Front Door Profile within which the Front Door Secret exists.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorSecretResult {
        /// Specifies the ID of the Front Door Profile within which this Front Door Secret exists.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub profile_name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `secret` block as defined below.
        pub secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorSecretSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFrontdoorSecretArgs) -> GetFrontdoorSecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let profile_name_binding = args.profile_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorSecret:getFrontdoorSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdnFrontdoorProfileId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "profileName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFrontdoorSecretResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorProfileId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
        }
    }
}
