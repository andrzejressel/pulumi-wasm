/// Provides a [Data Lifecycle Manager (DLM) lifecycle policy](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshot-lifecycle.html) for managing snapshots.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   dlmLifecycleRole:
///     type: aws:iam:Role
///     name: dlm_lifecycle_role
///     properties:
///       name: dlm-lifecycle-role
///       assumeRolePolicy: ${assumeRole.json}
///   dlmLifecycleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: dlm_lifecycle
///     properties:
///       name: dlm-lifecycle-policy
///       role: ${dlmLifecycleRole.id}
///       policy: ${dlmLifecycle.json}
///   example:
///     type: aws:dlm:LifecyclePolicy
///     properties:
///       description: example DLM lifecycle policy
///       executionRoleArn: ${dlmLifecycleRole.arn}
///       state: ENABLED
///       policyDetails:
///         resourceTypes: VOLUME
///         schedules:
///           - name: 2 weeks of daily snapshots
///             createRule:
///               interval: 24
///               intervalUnit: HOURS
///               times: 23:45
///             retainRule:
///               count: 14
///             tagsToAdd:
///               SnapshotCreator: DLM
///             copyTags: false
///         targetTags:
///           Snapshot: 'true'
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - dlm.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   dlmLifecycle:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:CreateSnapshot
///               - ec2:CreateSnapshots
///               - ec2:DeleteSnapshot
///               - ec2:DescribeInstances
///               - ec2:DescribeVolumes
///               - ec2:DescribeSnapshots
///             resources:
///               - '*'
///           - effect: Allow
///             actions:
///               - ec2:CreateTags
///             resources:
///               - arn:aws:ec2:*::snapshot/*
/// ```
///
/// ### Example Cross-Region Snapshot Copy Usage
///
/// ```yaml
/// resources:
///   dlmCrossRegionCopyCmk:
///     type: aws:kms:Key
///     name: dlm_cross_region_copy_cmk
///     properties:
///       description: Example Alternate Region KMS Key
///       policy: ${key.json}
///   example:
///     type: aws:dlm:LifecyclePolicy
///     properties:
///       description: example DLM lifecycle policy
///       executionRoleArn: ${dlmLifecycleRole.arn}
///       state: ENABLED
///       policyDetails:
///         resourceTypes: VOLUME
///         schedules:
///           - name: 2 weeks of daily snapshots
///             createRule:
///               interval: 24
///               intervalUnit: HOURS
///               times: 23:45
///             retainRule:
///               count: 14
///             tagsToAdd:
///               SnapshotCreator: DLM
///             copyTags: false
///             crossRegionCopyRules:
///               - target: us-west-2
///                 encrypted: true
///                 cmkArn: ${dlmCrossRegionCopyCmk.arn}
///                 copyTags: true
///                 retainRule:
///                   interval: 30
///                   intervalUnit: DAYS
///         targetTags:
///           Snapshot: 'true'
/// variables:
///   # ...other configuration...
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
///   key:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - sid: Enable IAM User Permissions
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${current.accountId}:root
///             actions:
///               - kms:*
///             resources:
///               - '*'
/// ```
///
/// ### Example Event Based Policy Usage
///
/// ```yaml
/// resources:
///   exampleLifecyclePolicy:
///     type: aws:dlm:LifecyclePolicy
///     name: example
///     properties:
///       description: tf-acc-basic
///       executionRoleArn: ${exampleAwsIamRole.arn}
///       policyDetails:
///         policyType: EVENT_BASED_POLICY
///         action:
///           name: tf-acc-basic
///           crossRegionCopies:
///             - encryptionConfiguration: {}
///               retainRule:
///                 interval: 15
///                 intervalUnit: MONTHS
///               target: us-east-1
///         eventSource:
///           type: MANAGED_CWE
///           parameters:
///             descriptionRegex: '^.*Created for policy: policy-1234567890abcdef0.*$'
///             eventType: shareSnapshot
///             snapshotOwners:
///               - ${current.accountId}
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleAwsIamRole.id}
///       policyArn: ${example.arn}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
///   example:
///     fn::invoke:
///       Function: aws:iam:getPolicy
///       Arguments:
///         name: AWSDataLifecycleManagerServiceRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DLM lifecycle policies using their policy ID. For example:
///
/// ```sh
/// $ pulumi import aws:dlm/lifecyclePolicy:LifecyclePolicy example policy-abcdef12345678901
/// ```
pub mod lifecycle_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// A description for the DLM lifecycle policy.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM role that is able to be assumed by the DLM service.
        #[builder(into)]
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// See the `policy_details` configuration block. Max of 1.
        #[builder(into)]
        pub policy_details: pulumi_wasm_rust::Output<
            super::super::types::dlm::LifecyclePolicyPolicyDetails,
        >,
        /// Whether the lifecycle policy should be enabled or disabled. `ENABLED` or `DISABLED` are valid values. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// Amazon Resource Name (ARN) of the DLM Lifecycle Policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description for the DLM lifecycle policy.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The ARN of an IAM role that is able to be assumed by the DLM service.
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// See the `policy_details` configuration block. Max of 1.
        pub policy_details: pulumi_wasm_rust::Output<
            super::super::types::dlm::LifecyclePolicyPolicyDetails,
        >,
        /// Whether the lifecycle policy should be enabled or disabled. `ENABLED` or `DISABLED` are valid values. Defaults to `ENABLED`.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn create(name: &str, args: LifecyclePolicyArgs) -> LifecyclePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let execution_role_arn_binding = args.execution_role_arn.get_inner();
        let policy_details_binding = args.policy_details.get_inner();
        let state_binding = args.state.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dlm/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "policyDetails".into(),
                    value: &policy_details_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "executionRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "policyDetails".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        LifecyclePolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRoleArn").unwrap(),
            ),
            policy_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDetails").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
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
