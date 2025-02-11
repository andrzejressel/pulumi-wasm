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
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       function: aws:getCallerIdentity
///       arguments: {}
///   key:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicy
///       arguments:
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lifecycle_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// A description for the DLM lifecycle policy.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of an IAM role that is able to be assumed by the DLM service.
        #[builder(into)]
        pub execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// See the `policy_details` configuration block. Max of 1.
        #[builder(into)]
        pub policy_details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dlm::LifecyclePolicyPolicyDetails,
        >,
        /// Whether the lifecycle policy should be enabled or disabled. `ENABLED` or `DISABLED` are valid values. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// Amazon Resource Name (ARN) of the DLM Lifecycle Policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description for the DLM lifecycle policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an IAM role that is able to be assumed by the DLM service.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// See the `policy_details` configuration block. Max of 1.
        pub policy_details: pulumi_gestalt_rust::Output<
            super::super::types::dlm::LifecyclePolicyPolicyDetails,
        >,
        /// Whether the lifecycle policy should be enabled or disabled. `ENABLED` or `DISABLED` are valid values. Defaults to `ENABLED`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LifecyclePolicyArgs,
    ) -> LifecyclePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let execution_role_arn_binding = args.execution_role_arn.get_output(context);
        let policy_details_binding = args.policy_details.get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dlm/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRoleArn".into(),
                    value: &execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDetails".into(),
                    value: &policy_details_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LifecyclePolicyResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            execution_role_arn: o.get_field("executionRoleArn"),
            policy_details: o.get_field("policyDetails"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
