pub mod get_addon {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAddonArgs {
        /// Name of the EKS add-on. The name must match one of
        /// the names returned by [list-addon](https://docs.aws.amazon.com/cli/latest/reference/eks/list-addons.html).
        #[builder(into)]
        pub addon_name: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAddonResult {
        pub addon_name: pulumi_wasm_rust::Output<String>,
        /// Version of EKS add-on.
        pub addon_version: pulumi_wasm_rust::Output<String>,
        /// ARN of the EKS add-on.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Configuration values for the addon with a single JSON string.
        pub configuration_values: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was updated.
        pub modified_at: pulumi_wasm_rust::Output<String>,
        /// Pod identity association for the EKS add-on.
        pub pod_identity_associations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetAddonPodIdentityAssociation>,
        >,
        /// ARN of IAM role used for EKS add-on. If value is empty -
        /// then add-on uses the IAM role assigned to the EKS Cluster node.
        pub service_account_role_arn: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAddonArgs) -> GetAddonResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addon_name_binding = args.addon_name.get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:eks/getAddon:getAddon".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonName".into(),
                    value: &addon_name_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addonName".into(),
                },
                register_interface::ResultField {
                    name: "addonVersion".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "configurationValues".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "modifiedAt".into(),
                },
                register_interface::ResultField {
                    name: "podIdentityAssociations".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAddonResult {
            addon_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonName").unwrap(),
            ),
            addon_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonVersion").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            configuration_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationValues").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedAt").unwrap(),
            ),
            pod_identity_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("podIdentityAssociations").unwrap(),
            ),
            service_account_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountRoleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
