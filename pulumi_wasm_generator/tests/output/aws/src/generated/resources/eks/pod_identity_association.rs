/// Resource for managing an AWS EKS (Elastic Kubernetes) Pod Identity Association.
///
/// Creates an EKS Pod Identity association between a service account in an Amazon EKS cluster and an IAM role with EKS Pod Identity. Use EKS Pod Identity to give temporary IAM credentials to pods and the credentials are rotated automatically.
///
/// Amazon EKS Pod Identity associations provide the ability to manage credentials for your applications, similar to the way that EC2 instance profiles provide credentials to Amazon EC2 instances.
///
/// If a pod uses a service account that has an association, Amazon EKS sets environment variables in the containers of the pod. The environment variables configure the Amazon Web Services SDKs, including the Command Line Interface, to use the EKS Pod Identity credentials.
///
/// Pod Identity is a simpler method than IAM roles for service accounts, as this method doesn’t use OIDC identity providers. Additionally, you can configure a role for Pod Identity once, and reuse it across clusters.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["sts:AssumeRole",
///                     "sts:TagSession",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["pods.eks.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("eks-pod-identity-example")
///             .build_struct(),
///     );
///     let examplePodIdentityAssociation = pod_identity_association::create(
///         "examplePodIdentityAssociation",
///         PodIdentityAssociationArgs::builder()
///             .cluster_name("${exampleAwsEksCluster.name}")
///             .namespace("example")
///             .role_arn("${example.arn}")
///             .service_account("example-sa")
///             .build_struct(),
///     );
///     let exampleS3 = role_policy_attachment::create(
///         "exampleS3",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess")
///             .role("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS (Elastic Kubernetes) Pod Identity Association using the `cluster_name` and `association_id` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/podIdentityAssociation:PodIdentityAssociation example example,a-12345678
/// ```
pub mod pod_identity_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PodIdentityAssociationArgs {
        /// The name of the cluster to create the association in.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Kubernetes namespace inside the cluster to create the association in. The service account and the pods that use the service account must be in this namespace.
        #[builder(into)]
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to associate with the service account. The EKS Pod Identity agent manages credentials to assume this role for applications in the containers in the pods that use this service account.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Kubernetes service account inside the cluster to associate the IAM credentials with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PodIdentityAssociationResult {
        /// The Amazon Resource Name (ARN) of the association.
        pub association_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the association.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// The name of the cluster to create the association in.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Kubernetes namespace inside the cluster to create the association in. The service account and the pods that use the service account must be in this namespace.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role to associate with the service account. The EKS Pod Identity agent manages credentials to assume this role for applications in the containers in the pods that use this service account.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Kubernetes service account inside the cluster to associate the IAM credentials with.
        ///
        /// The following arguments are optional:
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PodIdentityAssociationArgs,
    ) -> PodIdentityAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/podIdentityAssociation:PodIdentityAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "associationArn".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PodIdentityAssociationResult {
            association_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationArn").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}