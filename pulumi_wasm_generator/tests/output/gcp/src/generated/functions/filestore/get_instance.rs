pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of the location of the instance. This
        /// can be a region for ENTERPRISE tier instances. If it is not provided,
        /// the provider region or zone is used.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a Filestore instance.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection_enabled: pulumi_wasm_rust::Output<bool>,
        pub deletion_protection_reason: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub file_shares: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::filestore::GetInstanceFileShare>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub networks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::filestore::GetInstanceNetwork>,
        >,
        pub performance_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::filestore::GetInstancePerformanceConfig>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tier: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceArgs) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:filestore/getInstance:getInstance".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionReason".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "fileShares".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
                register_interface::ResultField {
                    name: "performanceConfigs".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletion_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionEnabled").unwrap(),
            ),
            deletion_protection_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionReason").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            file_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileShares").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
            performance_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("performanceConfigs").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
