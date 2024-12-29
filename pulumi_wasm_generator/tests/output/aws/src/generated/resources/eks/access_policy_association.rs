/// Access Entry Policy Association for an EKS Cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:AccessPolicyAssociation
///     properties:
///       clusterName: ${exampleAwsEksCluster.name}
///       policyArn: arn:aws:eks::aws:cluster-access-policy/AmazonEKSViewPolicy
///       principalArn: ${exampleAwsIamUser.arn}
///       accessScope:
///         type: namespace
///         namespaces:
///           - example-namespace
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS access entry using the `cluster_name` `principal_arn` and `policy_arn` separated by a colon (`#`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/accessPolicyAssociation:AccessPolicyAssociation my_eks_access_entry my_cluster_name#my_principal_arn#my_policy_arn
/// ```
pub mod access_policy_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPolicyAssociationArgs {
        /// The configuration block to determine the scope of the access. See `access_scope` Block below.
        #[builder(into)]
        pub access_scope: pulumi_wasm_rust::Output<
            super::super::types::eks::AccessPolicyAssociationAccessScope,
        >,
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the access policy that you're associating.
        #[builder(into)]
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        #[builder(into)]
        pub principal_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPolicyAssociationResult {
        /// The configuration block to determine the scope of the access. See `access_scope` Block below.
        pub access_scope: pulumi_wasm_rust::Output<
            super::super::types::eks::AccessPolicyAssociationAccessScope,
        >,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the policy was associated.
        pub associated_at: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the policy was updated.
        pub modified_at: pulumi_wasm_rust::Output<String>,
        /// The ARN of the access policy that you're associating.
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// The IAM Principal ARN which requires Authentication access to the EKS cluster.
        pub principal_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessPolicyAssociationArgs,
    ) -> AccessPolicyAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_scope_binding = args.access_scope.get_inner();
        let cluster_name_binding = args.cluster_name.get_inner();
        let policy_arn_binding = args.policy_arn.get_inner();
        let principal_arn_binding = args.principal_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/accessPolicyAssociation:AccessPolicyAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessScope".into(),
                    value: &access_scope_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalArn".into(),
                    value: &principal_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessScope".into(),
                },
                register_interface::ResultField {
                    name: "associatedAt".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "modifiedAt".into(),
                },
                register_interface::ResultField {
                    name: "policyArn".into(),
                },
                register_interface::ResultField {
                    name: "principalArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessPolicyAssociationResult {
            access_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessScope").unwrap(),
            ),
            associated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedAt").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedAt").unwrap(),
            ),
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArn").unwrap(),
            ),
            principal_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalArn").unwrap(),
            ),
        }
    }
}
