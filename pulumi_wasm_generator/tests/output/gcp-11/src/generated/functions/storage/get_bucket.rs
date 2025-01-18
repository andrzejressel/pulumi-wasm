pub mod get_bucket {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it is not provided then the data source will use the Compute API to find the project id that corresponds to the project number returned from the Storage API, and if no Compute API permissions are available or if the Compute API is disabled it defaults to the provider value. Supplying a value for `project` doesn't influence retrieving data about the bucket but it can be used to prevent use of the Compute API. If you do provide a `project` value ensure that it is the correct value for that bucket; the data source will not check that the project id and project number match.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketResult {
        pub autoclasses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketAutoclass>,
        >,
        pub cors: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketCor>,
        >,
        pub custom_placement_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketCustomPlacementConfig>,
        >,
        pub default_event_based_hold: pulumi_wasm_rust::Output<bool>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_object_retention: pulumi_wasm_rust::Output<bool>,
        pub encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketEncryption>,
        >,
        pub force_destroy: pulumi_wasm_rust::Output<bool>,
        pub hierarchical_namespaces: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketHierarchicalNamespace>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub lifecycle_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketLifecycleRule>,
        >,
        pub location: pulumi_wasm_rust::Output<String>,
        pub loggings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketLogging>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub project_number: pulumi_wasm_rust::Output<i32>,
        pub public_access_prevention: pulumi_wasm_rust::Output<String>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub requester_pays: pulumi_wasm_rust::Output<bool>,
        pub retention_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketRetentionPolicy>,
        >,
        pub rpo: pulumi_wasm_rust::Output<String>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub soft_delete_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketSoftDeletePolicy>,
        >,
        pub storage_class: pulumi_wasm_rust::Output<String>,
        pub uniform_bucket_level_access: pulumi_wasm_rust::Output<bool>,
        pub url: pulumi_wasm_rust::Output<String>,
        pub versionings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketVersioning>,
        >,
        pub websites: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetBucketWebsite>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBucketArgs) -> GetBucketResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getBucket:getBucket".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "autoclasses".into(),
                },
                register_interface::ResultField {
                    name: "cors".into(),
                },
                register_interface::ResultField {
                    name: "customPlacementConfigs".into(),
                },
                register_interface::ResultField {
                    name: "defaultEventBasedHold".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableObjectRetention".into(),
                },
                register_interface::ResultField {
                    name: "encryptions".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "hierarchicalNamespaces".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleRules".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "loggings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "projectNumber".into(),
                },
                register_interface::ResultField {
                    name: "publicAccessPrevention".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "requesterPays".into(),
                },
                register_interface::ResultField {
                    name: "retentionPolicies".into(),
                },
                register_interface::ResultField {
                    name: "rpo".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "softDeletePolicies".into(),
                },
                register_interface::ResultField {
                    name: "storageClass".into(),
                },
                register_interface::ResultField {
                    name: "uniformBucketLevelAccess".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
                register_interface::ResultField {
                    name: "versionings".into(),
                },
                register_interface::ResultField {
                    name: "websites".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBucketResult {
            autoclasses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoclasses").unwrap(),
            ),
            cors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cors").unwrap(),
            ),
            custom_placement_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customPlacementConfigs").unwrap(),
            ),
            default_event_based_hold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEventBasedHold").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_object_retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableObjectRetention").unwrap(),
            ),
            encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptions").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            hierarchical_namespaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hierarchicalNamespaces").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            lifecycle_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleRules").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            loggings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            project_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectNumber").unwrap(),
            ),
            public_access_prevention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicAccessPrevention").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            requester_pays: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requesterPays").unwrap(),
            ),
            retention_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPolicies").unwrap(),
            ),
            rpo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rpo").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            soft_delete_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softDeletePolicies").unwrap(),
            ),
            storage_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageClass").unwrap(),
            ),
            uniform_bucket_level_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniformBucketLevelAccess").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("url").unwrap(),
            ),
            versionings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionings").unwrap(),
            ),
            websites: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websites").unwrap(),
            ),
        }
    }
}
