/// Manages an EKS Fargate Profile.
///
/// ## Example Usage
///
///
/// ### Example IAM Role for EKS Fargate Profile
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: eks-fargate-profile-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: eks-fargate-pods.amazonaws.com
///           Version: 2012-10-17
///   example-AmazonEKSFargatePodExecutionRolePolicy:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSFargatePodExecutionRolePolicy
///       role: ${example.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Fargate Profiles using the `cluster_name` and `fargate_profile_name` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/fargateProfile:FargateProfile my_fargate_profile my_cluster:my_fargate_profile
/// ```
pub mod fargate_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FargateProfileArgs {
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Fargate Profile.
        #[builder(into, default)]
        pub fargate_profile_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Fargate Profile.
        #[builder(into)]
        pub pod_execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) for selecting Kubernetes Pods to execute with this EKS Fargate Profile. Detailed below.
        #[builder(into)]
        pub selectors: pulumi_wasm_rust::Output<
            Vec<super::super::types::eks::FargateProfileSelector>,
        >,
        /// Identifiers of private EC2 Subnets to associate with the EKS Fargate Profile. These subnets must have the following resource tag: `kubernetes.io/cluster/CLUSTER_NAME` (where `CLUSTER_NAME` is replaced with the name of the EKS Cluster).
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub subnet_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FargateProfileResult {
        /// Amazon Resource Name (ARN) of the EKS Fargate Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Fargate Profile.
        pub fargate_profile_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Fargate Profile.
        pub pod_execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block(s) for selecting Kubernetes Pods to execute with this EKS Fargate Profile. Detailed below.
        pub selectors: pulumi_wasm_rust::Output<
            Vec<super::super::types::eks::FargateProfileSelector>,
        >,
        /// Status of the EKS Fargate Profile.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Identifiers of private EC2 Subnets to associate with the EKS Fargate Profile. These subnets must have the following resource tag: `kubernetes.io/cluster/CLUSTER_NAME` (where `CLUSTER_NAME` is replaced with the name of the EKS Cluster).
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
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
    pub fn create(name: &str, args: FargateProfileArgs) -> FargateProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let fargate_profile_name_binding = args.fargate_profile_name.get_inner();
        let pod_execution_role_arn_binding = args.pod_execution_role_arn.get_inner();
        let selectors_binding = args.selectors.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/fargateProfile:FargateProfile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "fargateProfileName".into(),
                    value: &fargate_profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "podExecutionRoleArn".into(),
                    value: &pod_execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "selectors".into(),
                    value: &selectors_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "fargateProfileName".into(),
                },
                register_interface::ResultField {
                    name: "podExecutionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "selectors".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
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
        FargateProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            fargate_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fargateProfileName").unwrap(),
            ),
            pod_execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("podExecutionRoleArn").unwrap(),
            ),
            selectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectors").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
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