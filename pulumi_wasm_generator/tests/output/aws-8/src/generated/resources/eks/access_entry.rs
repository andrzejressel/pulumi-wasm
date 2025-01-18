/// Access Entry Configurations for an EKS Cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_entry::create(
///         "example",
///         AccessEntryArgs::builder()
///             .cluster_name("${exampleAwsEksCluster.name}")
///             .kubernetes_groups(vec!["group-1", "group-2",])
///             .principal_arn("${exampleAwsIamRole.arn}")
///             .type_("STANDARD")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS access entry using the `cluster_name` and `principal_arn` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/accessEntry:AccessEntry my_eks_access_entry my_cluster_name:my_principal_arn
/// ```
pub mod access_entry {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessEntryArgs {
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// List of string which can optionally specify the Kubernetes groups the user would belong to when creating an access entry.
        #[builder(into, default)]
        pub kubernetes_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Defaults to STANDARD which provides the standard workflow. EC2_LINUX, EC2_WINDOWS, FARGATE_LINUX types disallow users to input a username or groups, and prevent associations.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Defaults to principal ARN if user is principal else defaults to assume-role/session-name is role is used.
        #[builder(into, default)]
        pub user_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessEntryResult {
        /// Amazon Resource Name (ARN) of the Access Entry.
        pub access_entry_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// List of string which can optionally specify the Kubernetes groups the user would belong to when creating an access entry.
        pub kubernetes_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was updated.
        pub modified_at: pulumi_wasm_rust::Output<String>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        ///
        /// The following arguments are optional:
        pub principal_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (Optional) Key-value map of resource tags, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defaults to STANDARD which provides the standard workflow. EC2_LINUX, EC2_WINDOWS, FARGATE_LINUX types disallow users to input a username or groups, and prevent associations.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Defaults to principal ARN if user is principal else defaults to assume-role/session-name is role is used.
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessEntryArgs) -> AccessEntryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let kubernetes_groups_binding = args.kubernetes_groups.get_inner();
        let principal_arn_binding = args.principal_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/accessEntry:AccessEntry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesGroups".into(),
                    value: &kubernetes_groups_binding,
                },
                register_interface::ObjectField {
                    name: "principalArn".into(),
                    value: &principal_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessEntryArn".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesGroups".into(),
                },
                register_interface::ResultField {
                    name: "modifiedAt".into(),
                },
                register_interface::ResultField {
                    name: "principalArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessEntryResult {
            access_entry_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessEntryArn").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            kubernetes_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesGroups").unwrap(),
            ),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedAt").unwrap(),
            ),
            principal_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
