pub mod get_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppArgs {
        /// The name of the Container App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where this Container App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAppResult {
        /// The ID of the Container App Environment this Container App is linked to.
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        pub custom_domain_verification_id: pulumi_wasm_rust::Output<String>,
        /// A `dapr` block as detailed below.
        pub daprs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppDapr>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Resource ID for the User Assigned Managed identity to use when pulling from the Container Registry.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppIdentity>,
        >,
        /// An `ingress` block as detailed below.
        pub ingresses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppIngress>,
        >,
        pub latest_revision_fqdn: pulumi_wasm_rust::Output<String>,
        pub latest_revision_name: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// The max inactive revisions for this Container App.
        pub max_inactive_revisions: pulumi_wasm_rust::Output<i32>,
        /// Name for the IP restriction rule.
        pub name: pulumi_wasm_rust::Output<String>,
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `registry` block as detailed below.
        pub registries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppRegistry>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The revision mode of the Container App.
        pub revision_mode: pulumi_wasm_rust::Output<String>,
        /// One or more `secret` block as detailed below.
        pub secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppSecret>,
        >,
        /// A mapping of tags to assign to the Container App.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A `template` block as detailed below.
        pub templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::containerapp::GetAppTemplate>,
        >,
        /// The name of the Workload Profile in the Container App Environment in which this Container App is running.
        pub workload_profile_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAppArgs) -> GetAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerapp/getApp:getApp".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerAppEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "customDomainVerificationId".into(),
                },
                register_interface::ResultField {
                    name: "daprs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "ingresses".into(),
                },
                register_interface::ResultField {
                    name: "latestRevisionFqdn".into(),
                },
                register_interface::ResultField {
                    name: "latestRevisionName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxInactiveRevisions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "registries".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "revisionMode".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "templates".into(),
                },
                register_interface::ResultField {
                    name: "workloadProfileName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAppResult {
            container_app_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentId").unwrap(),
            ),
            custom_domain_verification_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDomainVerificationId").unwrap(),
            ),
            daprs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("daprs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            ingresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingresses").unwrap(),
            ),
            latest_revision_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestRevisionFqdn").unwrap(),
            ),
            latest_revision_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestRevisionName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_inactive_revisions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxInactiveRevisions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundIpAddresses").unwrap(),
            ),
            registries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registries").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            revision_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionMode").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templates").unwrap(),
            ),
            workload_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadProfileName").unwrap(),
            ),
        }
    }
}
