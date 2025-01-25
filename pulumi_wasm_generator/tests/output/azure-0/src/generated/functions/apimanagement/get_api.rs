pub mod get_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApiArgs {
        /// The name of the API Management Service in which the API Management API exists.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the API Management API.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Revision of the API Management API.
        #[builder(into)]
        pub revision: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApiResult {
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// A description of the API Management API, which may include HTML formatting tags.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of the API.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Is this the current API Revision?
        pub is_current: pulumi_wasm_rust::Output<bool>,
        /// Is this API Revision online/accessible via the Gateway?
        pub is_online: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Path for this API Management API.
        pub path: pulumi_wasm_rust::Output<String>,
        /// A list of protocols the operations in this API can be invoked.
        pub protocols: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub revision: pulumi_wasm_rust::Output<String>,
        /// Absolute URL of the backend service implementing this API.
        pub service_url: pulumi_wasm_rust::Output<String>,
        /// Should this API expose a SOAP frontend, rather than a HTTP frontend?
        pub soap_pass_through: pulumi_wasm_rust::Output<bool>,
        /// A `subscription_key_parameter_names` block as documented below.
        pub subscription_key_parameter_names: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::apimanagement::GetApiSubscriptionKeyParameterName,
            >,
        >,
        /// Should this API require a subscription key?
        pub subscription_required: pulumi_wasm_rust::Output<bool>,
        /// The Version number of this API, if this API is versioned.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Version Set which this API is associated with.
        pub version_set_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetApiArgs,
    ) -> GetApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let revision_binding = args.revision.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getApi:getApi".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "revision".into(),
                    value: &revision_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "isCurrent".into(),
                },
                register_interface::ResultField {
                    name: "isOnline".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "protocols".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "revision".into(),
                },
                register_interface::ResultField {
                    name: "serviceUrl".into(),
                },
                register_interface::ResultField {
                    name: "soapPassThrough".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionKeyParameterNames".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionRequired".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionSetId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApiResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            is_current: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isCurrent").unwrap(),
            ),
            is_online: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isOnline").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocols").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revision").unwrap(),
            ),
            service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUrl").unwrap(),
            ),
            soap_pass_through: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("soapPassThrough").unwrap(),
            ),
            subscription_key_parameter_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionKeyParameterNames").unwrap(),
            ),
            subscription_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionRequired").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionSetId").unwrap(),
            ),
        }
    }
}
