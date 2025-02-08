/// Creates a AWS Batch compute environment. Compute environments contain the Amazon ECS container instances that are used to run containerized batch jobs.
///
/// For information about AWS Batch, see [What is AWS Batch?](http://docs.aws.amazon.com/batch/latest/userguide/what-is-batch.html) .
/// For information about compute environment, see [Compute Environments](http://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html) .
///
/// > **Note:** To prevent a race condition during environment deletion, make sure to set `depends_on` to the related `aws.iam.RolePolicyAttachment`;
/// otherwise, the policy may be destroyed too soon and the compute environment will then get stuck in the `DELETING` state, see [Troubleshooting AWS Batch](http://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html) .
///
/// ## Example Usage
///
/// ### EC2 Type
///
/// ```yaml
/// resources:
///   ecsInstanceRole:
///     type: aws:iam:Role
///     name: ecs_instance_role
///     properties:
///       name: ecs_instance_role
///       assumeRolePolicy: ${ec2AssumeRole.json}
///   ecsInstanceRoleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: ecs_instance_role
///     properties:
///       role: ${ecsInstanceRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role
///   ecsInstanceRoleInstanceProfile:
///     type: aws:iam:InstanceProfile
///     name: ecs_instance_role
///     properties:
///       name: ecs_instance_role
///       role: ${ecsInstanceRole.name}
///   awsBatchServiceRole:
///     type: aws:iam:Role
///     name: aws_batch_service_role
///     properties:
///       name: aws_batch_service_role
///       assumeRolePolicy: ${batchAssumeRole.json}
///   awsBatchServiceRoleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: aws_batch_service_role
///     properties:
///       role: ${awsBatchServiceRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSBatchServiceRole
///   sample:
///     type: aws:ec2:SecurityGroup
///     properties:
///       name: aws_batch_compute_environment_security_group
///       egress:
///         - fromPort: 0
///           toPort: 0
///           protocol: '-1'
///           cidrBlocks:
///             - 0.0.0.0/0
///   sampleVpc:
///     type: aws:ec2:Vpc
///     name: sample
///     properties:
///       cidrBlock: 10.1.0.0/16
///   sampleSubnet:
///     type: aws:ec2:Subnet
///     name: sample
///     properties:
///       vpcId: ${sampleVpc.id}
///       cidrBlock: 10.1.1.0/24
///   samplePlacementGroup:
///     type: aws:ec2:PlacementGroup
///     name: sample
///     properties:
///       name: sample
///       strategy: cluster
///   sampleComputeEnvironment:
///     type: aws:batch:ComputeEnvironment
///     name: sample
///     properties:
///       computeEnvironmentName: sample
///       computeResources:
///         instanceRole: ${ecsInstanceRoleInstanceProfile.arn}
///         instanceTypes:
///           - c4.large
///         maxVcpus: 16
///         minVcpus: 0
///         placementGroup: ${samplePlacementGroup.name}
///         securityGroupIds:
///           - ${sample.id}
///         subnets:
///           - ${sampleSubnet.id}
///         type: EC2
///       serviceRole: ${awsBatchServiceRole.arn}
///       type: MANAGED
///     options:
///       dependsOn:
///         - ${awsBatchServiceRoleRolePolicyAttachment}
/// variables:
///   ec2AssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - ec2.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   batchAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - batch.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Fargate Type
///
/// ```yaml
/// resources:
///   sample:
///     type: aws:batch:ComputeEnvironment
///     properties:
///       computeEnvironmentName: sample
///       computeResources:
///         maxVcpus: 16
///         securityGroupIds:
///           - ${sampleAwsSecurityGroup.id}
///         subnets:
///           - ${sampleAwsSubnet.id}
///         type: FARGATE
///       serviceRole: ${awsBatchServiceRoleAwsIamRole.arn}
///       type: MANAGED
///     options:
///       dependsOn:
///         - ${awsBatchServiceRole}
/// ```
///
/// ### Setting Update Policy
///
/// ```yaml
/// resources:
///   sample:
///     type: aws:batch:ComputeEnvironment
///     properties:
///       computeEnvironmentName: sample
///       computeResources:
///         allocationStrategy: BEST_FIT_PROGRESSIVE
///         instanceRole: ${ecsInstance.arn}
///         instanceTypes:
///           - optimal
///         maxVcpus: 4
///         minVcpus: 0
///         securityGroupIds:
///           - ${sampleAwsSecurityGroup.id}
///         subnets:
///           - ${sampleAwsSubnet.id}
///         type: EC2
///       updatePolicy:
///         jobExecutionTimeoutMinutes: 30
///         terminateJobsOnUpdate: false
///       type: MANAGED
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Batch compute using the `compute_environment_name`. For example:
///
/// ```sh
/// $ pulumi import aws:batch/computeEnvironment:ComputeEnvironment sample sample
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod compute_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ComputeEnvironmentArgs {
        /// The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, and underscores are allowed. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub compute_environment_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique compute environment name beginning with the specified prefix. Conflicts with `compute_environment_name`.
        #[builder(into, default)]
        pub compute_environment_name_prefix: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Details of the compute resources managed by the compute environment. This parameter is required for managed compute environments. See details below.
        #[builder(into, default)]
        pub compute_resources: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::ComputeEnvironmentComputeResources>,
        >,
        /// Details for the Amazon EKS cluster that supports the compute environment. See details below.
        #[builder(into, default)]
        pub eks_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::ComputeEnvironmentEksConfiguration>,
        >,
        /// The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.
        #[builder(into, default)]
        pub service_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state of the compute environment. If the state is `ENABLED`, then the compute environment accepts jobs from a queue and can scale out automatically based on queues. Valid items are `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the compute environment. Valid items are `MANAGED` or `UNMANAGED`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the infrastructure update policy for the compute environment. See details below.
        #[builder(into, default)]
        pub update_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::ComputeEnvironmentUpdatePolicy>,
        >,
    }
    #[allow(dead_code)]
    pub struct ComputeEnvironmentResult {
        /// The Amazon Resource Name (ARN) of the compute environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, and underscores are allowed. If omitted, the provider will assign a random, unique name.
        pub compute_environment_name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique compute environment name beginning with the specified prefix. Conflicts with `compute_environment_name`.
        pub compute_environment_name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Details of the compute resources managed by the compute environment. This parameter is required for managed compute environments. See details below.
        pub compute_resources: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::ComputeEnvironmentComputeResources>,
        >,
        /// The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster used by the compute environment.
        pub ecs_cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// Details for the Amazon EKS cluster that supports the compute environment. See details below.
        pub eks_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::ComputeEnvironmentEksConfiguration>,
        >,
        /// The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// The state of the compute environment. If the state is `ENABLED`, then the compute environment accepts jobs from a queue and can scale out automatically based on queues. Valid items are `ENABLED` or `DISABLED`. Defaults to `ENABLED`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current status of the compute environment (for example, CREATING or VALID).
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A short, human-readable string to provide additional details about the current status of the compute environment.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the compute environment. Valid items are `MANAGED` or `UNMANAGED`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies the infrastructure update policy for the compute environment. See details below.
        pub update_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::ComputeEnvironmentUpdatePolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ComputeEnvironmentArgs,
    ) -> ComputeEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let compute_environment_name_binding = args
            .compute_environment_name
            .get_output(context)
            .get_inner();
        let compute_environment_name_prefix_binding = args
            .compute_environment_name_prefix
            .get_output(context)
            .get_inner();
        let compute_resources_binding = args
            .compute_resources
            .get_output(context)
            .get_inner();
        let eks_configuration_binding = args
            .eks_configuration
            .get_output(context)
            .get_inner();
        let service_role_binding = args.service_role.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let update_policy_binding = args.update_policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:batch/computeEnvironment:ComputeEnvironment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computeEnvironmentName".into(),
                    value: &compute_environment_name_binding,
                },
                register_interface::ObjectField {
                    name: "computeEnvironmentNamePrefix".into(),
                    value: &compute_environment_name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "computeResources".into(),
                    value: &compute_resources_binding,
                },
                register_interface::ObjectField {
                    name: "eksConfiguration".into(),
                    value: &eks_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRole".into(),
                    value: &service_role_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
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
                    name: "updatePolicy".into(),
                    value: &update_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ComputeEnvironmentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            compute_environment_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeEnvironmentName"),
            ),
            compute_environment_name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeEnvironmentNamePrefix"),
            ),
            compute_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeResources"),
            ),
            ecs_cluster_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ecsClusterArn"),
            ),
            eks_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eksConfiguration"),
            ),
            service_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceRole"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            update_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updatePolicy"),
            ),
        }
    }
}
