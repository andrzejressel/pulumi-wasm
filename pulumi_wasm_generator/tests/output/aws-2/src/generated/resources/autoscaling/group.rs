/// Provides an Auto Scaling Group resource.
///
/// > **Note:** You must specify either `launch_configuration`, `launch_template`, or `mixed_instances_policy`.
///
/// > **NOTE on Auto Scaling Groups, Attachments and Traffic Source Attachments:** Pulumi provides standalone Attachment (for attaching Classic Load Balancers and Application Load Balancer, Gateway Load Balancer, or Network Load Balancer target groups) and Traffic Source Attachment (for attaching Load Balancers and VPC Lattice target groups) resources and an Auto Scaling Group resource with `load_balancers`, `target_group_arns` and `traffic_source` attributes. Do not use the same traffic source in more than one of these resources. Doing so will cause a conflict of attachments. A `lifecycle` configuration block can be used to suppress differences if necessary.
///
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:PlacementGroup
///     properties:
///       name: test
///       strategy: cluster
///   bar:
///     type: aws:autoscaling:Group
///     properties:
///       name: foobar3-test
///       maxSize: 5
///       minSize: 2
///       healthCheckGracePeriod: 300
///       healthCheckType: ELB
///       desiredCapacity: 4
///       forceDelete: true
///       placementGroup: ${test.id}
///       launchConfiguration: ${foobar.name}
///       vpcZoneIdentifiers:
///         - ${example1.id}
///         - ${example2.id}
///       instanceMaintenancePolicy:
///         minHealthyPercentage: 90
///         maxHealthyPercentage: 120
///       initialLifecycleHooks:
///         - name: foobar
///           defaultResult: CONTINUE
///           heartbeatTimeout: 2000
///           lifecycleTransition: autoscaling:EC2_INSTANCE_LAUNCHING
///           notificationMetadata:
///             fn::toJSON:
///               foo: bar
///           notificationTargetArn: arn:aws:sqs:us-east-1:444455556666:queue1*
///           roleArn: arn:aws:iam::123456789012:role/S3Access
///       tags:
///         - key: foo
///           value: bar
///           propagateAtLaunch: true
///         - key: lorem
///           value: ipsum
///           propagateAtLaunch: false
/// ```
///
/// ### With Latest Version Of Launch Template
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = group::create(
///         "bar",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .desired_capacity(1)
///             .launch_template(
///                 GroupLaunchTemplate::builder()
///                     .id("${foobar.id}")
///                     .version("$Latest")
///                     .build_struct(),
///             )
///             .max_size(1)
///             .min_size(1)
///             .build_struct(),
///     );
///     let foobar = launch_template::create(
///         "foobar",
///         LaunchTemplateArgs::builder()
///             .image_id("ami-1a2b3c")
///             .instance_type("t2.micro")
///             .name_prefix("foobar")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Mixed Instances Policy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = launch_template::create(
///         "example",
///         LaunchTemplateArgs::builder()
///             .image_id("${exampleAwsAmi.id}")
///             .instance_type("c5.large")
///             .name_prefix("example")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .desired_capacity(1)
///             .max_size(1)
///             .min_size(1)
///             .mixed_instances_policy(
///                 GroupMixedInstancesPolicy::builder()
///                     .launchTemplate(
///                         GroupMixedInstancesPolicyLaunchTemplate::builder()
///                             .launchTemplateSpecification(
///                                 GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification::builder()
///                                     .launchTemplateId("${example.id}")
///                                     .build_struct(),
///                             )
///                             .overrides(
///                                 vec![
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c4.large").weightedCapacity("3")
///                                     .build_struct(),
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c3.large").weightedCapacity("2")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Mixed Instances Policy with Spot Instances and Capacity Rebalance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = launch_template::create(
///         "example",
///         LaunchTemplateArgs::builder()
///             .image_id("${exampleAwsAmi.id}")
///             .instance_type("c5.large")
///             .name_prefix("example")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .capacity_rebalance(true)
///             .desired_capacity(12)
///             .max_size(15)
///             .min_size(12)
///             .mixed_instances_policy(
///                 GroupMixedInstancesPolicy::builder()
///                     .instancesDistribution(
///                         GroupMixedInstancesPolicyInstancesDistribution::builder()
///                             .onDemandBaseCapacity(0)
///                             .onDemandPercentageAboveBaseCapacity(25)
///                             .spotAllocationStrategy("capacity-optimized")
///                             .build_struct(),
///                     )
///                     .launchTemplate(
///                         GroupMixedInstancesPolicyLaunchTemplate::builder()
///                             .launchTemplateSpecification(
///                                 GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification::builder()
///                                     .launchTemplateId("${example.id}")
///                                     .build_struct(),
///                             )
///                             .overrides(
///                                 vec![
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c4.large").weightedCapacity("3")
///                                     .build_struct(),
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c3.large").weightedCapacity("2")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .vpc_zone_identifiers(vec!["${example1.id}", "${example2.id}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Mixed Instances Policy with Instance level LaunchTemplateSpecification Overrides
///
/// When using a diverse instance set, some instance types might require a launch template with configuration values unique to that instance type such as a different AMI (Graviton2), architecture specific user data script, different EBS configuration, or different networking configuration.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = launch_template::create(
///         "example",
///         LaunchTemplateArgs::builder()
///             .image_id("${exampleAwsAmi.id}")
///             .instance_type("c5.large")
///             .name_prefix("example")
///             .build_struct(),
///     );
///     let example2 = launch_template::create(
///         "example2",
///         LaunchTemplateArgs::builder()
///             .image_id("${example2AwsAmi.id}")
///             .name_prefix("example2")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .desired_capacity(1)
///             .max_size(1)
///             .min_size(1)
///             .mixed_instances_policy(
///                 GroupMixedInstancesPolicy::builder()
///                     .launchTemplate(
///                         GroupMixedInstancesPolicyLaunchTemplate::builder()
///                             .launchTemplateSpecification(
///                                 GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification::builder()
///                                     .launchTemplateId("${example.id}")
///                                     .build_struct(),
///                             )
///                             .overrides(
///                                 vec![
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c4.large").weightedCapacity("3")
///                                     .build_struct(),
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceType("c6g.large")
///                                     .launchTemplateSpecification(GroupMixedInstancesPolicyLaunchTemplateOverrideLaunchTemplateSpecification::builder()
///                                     .launchTemplateId("${example2.id}").build_struct())
///                                     .weightedCapacity("2").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Mixed Instances Policy with Attribute-based Instance Type Selection
///
/// As an alternative to manually choosing instance types when creating a mixed instances group, you can specify a set of instance attributes that describe your compute requirements.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = launch_template::create(
///         "example",
///         LaunchTemplateArgs::builder()
///             .image_id("${exampleAwsAmi.id}")
///             .instance_type("c5.large")
///             .name_prefix("example")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .desired_capacity(1)
///             .max_size(1)
///             .min_size(1)
///             .mixed_instances_policy(
///                 GroupMixedInstancesPolicy::builder()
///                     .launchTemplate(
///                         GroupMixedInstancesPolicyLaunchTemplate::builder()
///                             .launchTemplateSpecification(
///                                 GroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification::builder()
///                                     .launchTemplateId("${example.id}")
///                                     .build_struct(),
///                             )
///                             .overrides(
///                                 vec![
///                                     GroupMixedInstancesPolicyLaunchTemplateOverride::builder()
///                                     .instanceRequirements(GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements::builder()
///                                     .memoryMib(GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryMib::builder()
///                                     .min(1000).build_struct())
///                                     .vcpuCount(GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsVcpuCount::builder()
///                                     .min(4).build_struct()).build_struct()).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Dynamic tagging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = group::create(
///         "test",
///         GroupArgs::builder()
///             .launch_configuration("${foobar.name}")
///             .max_size(5)
///             .min_size(2)
///             .name("foobar3-test")
///             .tags(
///                 vec![
///                     GroupTag::builder().key("explicit1").propagateAtLaunch(true)
///                     .value("value1").build_struct(), GroupTag::builder().key("explicit2")
///                     .propagateAtLaunch(true).value("value2").build_struct(),
///                 ],
///             )
///             .vpc_zone_identifiers(vec!["${example1.id}", "${example2.id}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Automatically refresh all instances after the group is updated
///
/// ```yaml
/// resources:
///   exampleGroup:
///     type: aws:autoscaling:Group
///     name: example
///     properties:
///       availabilityZones:
///         - us-east-1a
///       desiredCapacity: 1
///       maxSize: 2
///       minSize: 1
///       launchTemplate:
///         id: ${exampleLaunchTemplate.id}
///         version: ${exampleLaunchTemplate.latestVersion}
///       tags:
///         - key: Key
///           value: Value
///           propagateAtLaunch: true
///       instanceRefresh:
///         strategy: Rolling
///         preferences:
///           minHealthyPercentage: 50
///         triggers:
///           - tag
///   exampleLaunchTemplate:
///     type: aws:ec2:LaunchTemplate
///     name: example
///     properties:
///       imageId: ${example.id}
///       instanceType: t3.nano
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         owners:
///           - amazon
///         filters:
///           - name: name
///             values:
///               - amzn-ami-hvm-*-x86_64-gp2
/// ```
///
/// ### Auto Scaling group with Warm Pool
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = launch_template::create(
///         "example",
///         LaunchTemplateArgs::builder()
///             .image_id("${exampleAwsAmi.id}")
///             .instance_type("c5.large")
///             .name_prefix("example")
///             .build_struct(),
///     );
///     let exampleGroup = group::create(
///         "exampleGroup",
///         GroupArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .desired_capacity(1)
///             .max_size(5)
///             .min_size(1)
///             .warm_pool(
///                 GroupWarmPool::builder()
///                     .instanceReusePolicy(
///                         GroupWarmPoolInstanceReusePolicy::builder()
///                             .reuseOnScaleIn(true)
///                             .build_struct(),
///                     )
///                     .maxGroupPreparedCapacity(10)
///                     .minSize(1)
///                     .poolState("Hibernated")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Auto Scaling group with Traffic Sources
///
///
/// ## Waiting for Capacity
///
/// A newly-created ASG is initially empty and begins to scale to `min_size` (or
/// `desired_capacity`, if specified) by launching instances using the provided
/// Launch Configuration. These instances take time to launch and boot.
///
/// On ASG Update, changes to these values also take time to result in the target
/// number of instances providing service.
///
/// This provider provides two mechanisms to help consistently manage ASG scale up
/// time across dependent resources.
///
/// #### Waiting for ASG Capacity
///
/// The first is default behavior. This provider waits after ASG creation for
/// `min_size` (or `desired_capacity`, if specified) healthy instances to show up
/// in the ASG before continuing.
///
/// If `min_size` or `desired_capacity` are changed in a subsequent update,
/// this provider will also wait for the correct number of healthy instances before
/// continuing.
///
/// This provider considers an instance "healthy" when the ASG reports `HealthStatus:
/// "Healthy"` and `LifecycleState: "InService"`. See the [AWS AutoScaling
/// Docs](https://docs.aws.amazon.com/AutoScaling/latest/DeveloperGuide/AutoScalingGroupLifecycle.html)
/// for more information on an ASG's lifecycle.
///
/// This provider will wait for healthy instances for up to
/// `wait_for_capacity_timeout`. If ASG creation is taking more than a few minutes,
/// it's worth investigating for scaling activity errors, which can be caused by
/// problems with the selected Launch Configuration.
///
/// Setting `wait_for_capacity_timeout` to `"0"` disables ASG Capacity waiting.
///
/// #### Waiting for ELB Capacity
///
/// The second mechanism is optional, and affects ASGs with attached ELBs specified
/// via the `load_balancers` attribute or with ALBs specified with `target_group_arns`.
///
/// The `min_elb_capacity` parameter causes the provider to wait for at least the
/// requested number of instances to show up `"InService"` in all attached ELBs
/// during ASG creation. It has no effect on ASG updates.
///
/// If `wait_for_elb_capacity` is set, the provider will wait for exactly that number
/// of Instances to be `"InService"` in all attached ELBs on both creation and
/// updates.
///
/// These parameters can be used to ensure that service is being provided before
/// the provider moves on. If new instances don't pass the ELB's health checks for any
/// reason, the apply will time out, and the ASG will be marked as
/// tainted (i.e., marked to be destroyed in a follow up run).
///
/// As with ASG Capacity, the provider will wait for up to `wait_for_capacity_timeout`
/// for the proper number of instances to be healthy.
///
/// #### Troubleshooting Capacity Waiting Timeouts
///
/// If ASG creation takes more than a few minutes, this could indicate one of a
/// number of configuration problems. See the [AWS Docs on Load Balancer
/// Troubleshooting](https://docs.aws.amazon.com/ElasticLoadBalancing/latest/DeveloperGuide/elb-troubleshooting.html)
/// for more information.
///
/// ## Import
///
/// Using `pulumi import`, import Auto Scaling Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:autoscaling/group:Group web web-asg
/// ```
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// The instance capacity distribution across Availability Zones. See Availability Zone Distribution below for more details.
        #[builder(into, default)]
        pub availability_zone_distribution: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupAvailabilityZoneDistribution>,
        >,
        /// A list of Availability Zones where instances in the Auto Scaling group can be created. Used for launching into the default VPC subnet in each Availability Zone when not using the `vpc_zone_identifier` attribute, or for attaching a network interface when an existing network interface ID is specified in a launch template. Conflicts with `vpc_zone_identifier`.
        #[builder(into, default)]
        pub availability_zones: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether capacity rebalance is enabled. Otherwise, capacity rebalance is disabled.
        #[builder(into, default)]
        pub capacity_rebalance: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Reserved.
        #[builder(into, default)]
        pub context: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Amount of time, in seconds, after a scaling activity completes before another scaling activity can start.
        #[builder(into, default)]
        pub default_cooldown: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Amount of time, in seconds, until a newly launched instance can contribute to the Amazon CloudWatch metrics. This delay lets an instance finish initializing before Amazon EC2 Auto Scaling aggregates instance metrics, resulting in more reliable usage data. Set this value equal to the amount of time that it takes for resource consumption to become stable after an instance reaches the InService state. (See [Set the default instance warmup for an Auto Scaling group](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-default-instance-warmup.html))
        #[builder(into, default)]
        pub default_instance_warmup: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Number of Amazon EC2 instances that
        /// should be running in the group. (See also Waiting for
        /// Capacity below.)
        #[builder(into, default)]
        pub desired_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The unit of measurement for the value specified for `desired_capacity`. Supported for attribute-based instance type selection only. Valid values: `"units"`, `"vcpu"`, `"memory-mib"`.
        #[builder(into, default)]
        pub desired_capacity_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of metrics to collect. The allowed values are defined by the [underlying AWS API](https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html).
        #[builder(into, default)]
        pub enabled_metrics: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Allows deleting the Auto Scaling Group without waiting
        /// for all instances in the pool to terminate. You can force an Auto Scaling Group to delete
        /// even if it's in the process of scaling a resource. Normally, this provider
        /// drains all the instances before deleting the group. This bypasses that
        /// behavior and potentially leaves resources dangling.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Allows deleting the Auto Scaling Group without waiting for all instances in the warm pool to terminate.
        #[builder(into, default)]
        pub force_delete_warm_pool: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Time (in seconds) after instance comes into service before checking health.
        #[builder(into, default)]
        pub health_check_grace_period: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// "EC2" or "ELB". Controls how health checking is done.
        #[builder(into, default)]
        pub health_check_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to ignore failed [Auto Scaling scaling activities](https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-verify-scaling-activity.html) while waiting for capacity. The default is `false` -- failed scaling activities cause errors to be returned.
        #[builder(into, default)]
        pub ignore_failed_scaling_activities: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more
        /// [Lifecycle Hooks](http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html)
        /// to attach to the Auto Scaling Group **before** instances are launched. The
        /// syntax is exactly the same as the separate
        /// `aws.autoscaling.LifecycleHook`
        /// resource, without the `autoscaling_group_name` attribute. Please note that this will only work when creating
        /// a new Auto Scaling Group. For all other use-cases, please use `aws.autoscaling.LifecycleHook` resource.
        #[builder(into, default)]
        pub initial_lifecycle_hooks: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::autoscaling::GroupInitialLifecycleHook>>,
        >,
        /// If this block is configured, add a instance maintenance policy to the specified Auto Scaling group. Defined below.
        #[builder(into, default)]
        pub instance_maintenance_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupInstanceMaintenancePolicy>,
        >,
        /// If this block is configured, start an
        /// [Instance Refresh](https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html)
        /// when this Auto Scaling Group is updated. Defined below.
        #[builder(into, default)]
        pub instance_refresh: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupInstanceRefresh>,
        >,
        /// Name of the launch configuration to use.
        #[builder(into, default)]
        pub launch_configuration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Nested argument with Launch template specification to use to launch instances. See Launch Template below for more details.
        #[builder(into, default)]
        pub launch_template: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupLaunchTemplate>,
        >,
        /// List of elastic load balancer names to add to the autoscaling
        /// group names. Only valid for classic load balancers. For ALBs, use `target_group_arns` instead. To remove all load balancer attachments an empty list should be specified.
        #[builder(into, default)]
        pub load_balancers: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Maximum amount of time, in seconds, that an instance can be in service, values must be either equal to 0 or between 86400 and 31536000 seconds.
        #[builder(into, default)]
        pub max_instance_lifetime: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Maximum size of the Auto Scaling Group.
        #[builder(into)]
        pub max_size: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Granularity to associate with the metrics to collect. The only valid value is `1Minute`. Default is `1Minute`.
        #[builder(into, default)]
        pub metrics_granularity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Setting this causes Pulumi to wait for
        /// this number of instances from this Auto Scaling Group to show up healthy in the
        /// ELB only on creation. Updates will not wait on ELB instance number changes.
        /// (See also Waiting for Capacity below.)
        #[builder(into, default)]
        pub min_elb_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Minimum size of the Auto Scaling Group.
        /// (See also Waiting for Capacity below.)
        #[builder(into)]
        pub min_size: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Configuration block containing settings to define launch targets for Auto Scaling groups. See Mixed Instances Policy below for more details.
        #[builder(into, default)]
        pub mixed_instances_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupMixedInstancesPolicy>,
        >,
        /// Name of the Auto Scaling Group. By default generated by Pulumi. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the placement group into which you'll launch your instances, if any.
        #[builder(into, default)]
        pub placement_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether newly launched instances
        /// are automatically protected from termination by Amazon EC2 Auto Scaling when
        /// scaling in. For more information about preventing instances from terminating
        /// on scale in, see [Using instance scale-in protection](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-instance-protection.html)
        /// in the Amazon EC2 Auto Scaling User Guide.
        #[builder(into, default)]
        pub protect_from_scale_in: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ARN of the service-linked role that the ASG will use to call other AWS services
        #[builder(into, default)]
        pub service_linked_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of processes to suspend for the Auto Scaling Group. The allowed values are `Launch`, `Terminate`, `HealthCheck`, `ReplaceUnhealthy`, `AZRebalance`, `AlarmNotification`, `ScheduledActions`, `AddToLoadBalancer`, `InstanceRefresh`.
        /// Note that if you suspend either the `Launch` or `Terminate` process types, it can prevent your Auto Scaling Group from functioning properly.
        #[builder(into, default)]
        pub suspended_processes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block(s) containing resource tags. See Tag below for more details.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::autoscaling::GroupTag>>,
        >,
        /// Set of `aws.alb.TargetGroup` ARNs, for use with Application or Network Load Balancing. To remove all target group attachments an empty list should be specified.
        #[builder(into, default)]
        pub target_group_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of policies to decide how the instances in the Auto Scaling Group should be terminated. The allowed values are `OldestInstance`, `NewestInstance`, `OldestLaunchConfiguration`, `ClosestToNextInstanceHour`, `OldestLaunchTemplate`, `AllocationStrategy`, `Default`. Additionally, the ARN of a Lambda function can be specified for custom termination policies.
        #[builder(into, default)]
        pub termination_policies: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Attaches one or more traffic sources to the specified Auto Scaling group.
        #[builder(into, default)]
        pub traffic_sources: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::autoscaling::GroupTrafficSource>>,
        >,
        /// List of subnet IDs to launch resources in. Subnets automatically determine which availability zones the group will reside. Conflicts with `availability_zones`.
        #[builder(into, default)]
        pub vpc_zone_identifiers: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that the provider should
        /// wait for ASG instances to be healthy before timing out. (See also Waiting
        /// for Capacity below.) Setting this to "0" causes
        /// the provider to skip all Capacity Waiting behavior.
        #[builder(into, default)]
        pub wait_for_capacity_timeout: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Setting this will cause Pulumi to wait
        /// for exactly this number of healthy instances from this Auto Scaling Group in
        /// all attached load balancers on both create and update operations. (Takes
        /// precedence over `min_elb_capacity` behavior.)
        /// (See also Waiting for Capacity below.)
        #[builder(into, default)]
        pub wait_for_elb_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// If this block is configured, add a [Warm Pool](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-warm-pools.html)
        /// to the specified Auto Scaling group. Defined below
        #[builder(into, default)]
        pub warm_pool: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::autoscaling::GroupWarmPool>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// ARN for this Auto Scaling Group
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The instance capacity distribution across Availability Zones. See Availability Zone Distribution below for more details.
        pub availability_zone_distribution: pulumi_wasm_rust::Output<
            super::super::types::autoscaling::GroupAvailabilityZoneDistribution,
        >,
        /// A list of Availability Zones where instances in the Auto Scaling group can be created. Used for launching into the default VPC subnet in each Availability Zone when not using the `vpc_zone_identifier` attribute, or for attaching a network interface when an existing network interface ID is specified in a launch template. Conflicts with `vpc_zone_identifier`.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether capacity rebalance is enabled. Otherwise, capacity rebalance is disabled.
        pub capacity_rebalance: pulumi_wasm_rust::Output<Option<bool>>,
        /// Reserved.
        pub context: pulumi_wasm_rust::Output<Option<String>>,
        /// Amount of time, in seconds, after a scaling activity completes before another scaling activity can start.
        pub default_cooldown: pulumi_wasm_rust::Output<i32>,
        /// Amount of time, in seconds, until a newly launched instance can contribute to the Amazon CloudWatch metrics. This delay lets an instance finish initializing before Amazon EC2 Auto Scaling aggregates instance metrics, resulting in more reliable usage data. Set this value equal to the amount of time that it takes for resource consumption to become stable after an instance reaches the InService state. (See [Set the default instance warmup for an Auto Scaling group](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-default-instance-warmup.html))
        pub default_instance_warmup: pulumi_wasm_rust::Output<Option<i32>>,
        /// Number of Amazon EC2 instances that
        /// should be running in the group. (See also Waiting for
        /// Capacity below.)
        pub desired_capacity: pulumi_wasm_rust::Output<i32>,
        /// The unit of measurement for the value specified for `desired_capacity`. Supported for attribute-based instance type selection only. Valid values: `"units"`, `"vcpu"`, `"memory-mib"`.
        pub desired_capacity_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of metrics to collect. The allowed values are defined by the [underlying AWS API](https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html).
        pub enabled_metrics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Allows deleting the Auto Scaling Group without waiting
        /// for all instances in the pool to terminate. You can force an Auto Scaling Group to delete
        /// even if it's in the process of scaling a resource. Normally, this provider
        /// drains all the instances before deleting the group. This bypasses that
        /// behavior and potentially leaves resources dangling.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Allows deleting the Auto Scaling Group without waiting for all instances in the warm pool to terminate.
        pub force_delete_warm_pool: pulumi_wasm_rust::Output<Option<bool>>,
        /// Time (in seconds) after instance comes into service before checking health.
        pub health_check_grace_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// "EC2" or "ELB". Controls how health checking is done.
        pub health_check_type: pulumi_wasm_rust::Output<String>,
        /// Whether to ignore failed [Auto Scaling scaling activities](https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-verify-scaling-activity.html) while waiting for capacity. The default is `false` -- failed scaling activities cause errors to be returned.
        pub ignore_failed_scaling_activities: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more
        /// [Lifecycle Hooks](http://docs.aws.amazon.com/autoscaling/latest/userguide/lifecycle-hooks.html)
        /// to attach to the Auto Scaling Group **before** instances are launched. The
        /// syntax is exactly the same as the separate
        /// `aws.autoscaling.LifecycleHook`
        /// resource, without the `autoscaling_group_name` attribute. Please note that this will only work when creating
        /// a new Auto Scaling Group. For all other use-cases, please use `aws.autoscaling.LifecycleHook` resource.
        pub initial_lifecycle_hooks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::autoscaling::GroupInitialLifecycleHook>>,
        >,
        /// If this block is configured, add a instance maintenance policy to the specified Auto Scaling group. Defined below.
        pub instance_maintenance_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::autoscaling::GroupInstanceMaintenancePolicy>,
        >,
        /// If this block is configured, start an
        /// [Instance Refresh](https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-refresh.html)
        /// when this Auto Scaling Group is updated. Defined below.
        pub instance_refresh: pulumi_wasm_rust::Output<
            Option<super::super::types::autoscaling::GroupInstanceRefresh>,
        >,
        /// Name of the launch configuration to use.
        pub launch_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Nested argument with Launch template specification to use to launch instances. See Launch Template below for more details.
        pub launch_template: pulumi_wasm_rust::Output<
            super::super::types::autoscaling::GroupLaunchTemplate,
        >,
        /// List of elastic load balancer names to add to the autoscaling
        /// group names. Only valid for classic load balancers. For ALBs, use `target_group_arns` instead. To remove all load balancer attachments an empty list should be specified.
        pub load_balancers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Maximum amount of time, in seconds, that an instance can be in service, values must be either equal to 0 or between 86400 and 31536000 seconds.
        pub max_instance_lifetime: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum size of the Auto Scaling Group.
        pub max_size: pulumi_wasm_rust::Output<i32>,
        /// Granularity to associate with the metrics to collect. The only valid value is `1Minute`. Default is `1Minute`.
        pub metrics_granularity: pulumi_wasm_rust::Output<Option<String>>,
        /// Setting this causes Pulumi to wait for
        /// this number of instances from this Auto Scaling Group to show up healthy in the
        /// ELB only on creation. Updates will not wait on ELB instance number changes.
        /// (See also Waiting for Capacity below.)
        pub min_elb_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum size of the Auto Scaling Group.
        /// (See also Waiting for Capacity below.)
        pub min_size: pulumi_wasm_rust::Output<i32>,
        /// Configuration block containing settings to define launch targets for Auto Scaling groups. See Mixed Instances Policy below for more details.
        pub mixed_instances_policy: pulumi_wasm_rust::Output<
            super::super::types::autoscaling::GroupMixedInstancesPolicy,
        >,
        /// Name of the Auto Scaling Group. By default generated by Pulumi. Conflicts with `name_prefix`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Name of the placement group into which you'll launch your instances, if any.
        pub placement_group: pulumi_wasm_rust::Output<Option<String>>,
        /// Predicted capacity of the group.
        pub predicted_capacity: pulumi_wasm_rust::Output<i32>,
        /// Whether newly launched instances
        /// are automatically protected from termination by Amazon EC2 Auto Scaling when
        /// scaling in. For more information about preventing instances from terminating
        /// on scale in, see [Using instance scale-in protection](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-instance-protection.html)
        /// in the Amazon EC2 Auto Scaling User Guide.
        pub protect_from_scale_in: pulumi_wasm_rust::Output<Option<bool>>,
        /// ARN of the service-linked role that the ASG will use to call other AWS services
        pub service_linked_role_arn: pulumi_wasm_rust::Output<String>,
        /// List of processes to suspend for the Auto Scaling Group. The allowed values are `Launch`, `Terminate`, `HealthCheck`, `ReplaceUnhealthy`, `AZRebalance`, `AlarmNotification`, `ScheduledActions`, `AddToLoadBalancer`, `InstanceRefresh`.
        /// Note that if you suspend either the `Launch` or `Terminate` process types, it can prevent your Auto Scaling Group from functioning properly.
        pub suspended_processes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block(s) containing resource tags. See Tag below for more details.
        pub tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::autoscaling::GroupTag>>,
        >,
        /// Set of `aws.alb.TargetGroup` ARNs, for use with Application or Network Load Balancing. To remove all target group attachments an empty list should be specified.
        pub target_group_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of policies to decide how the instances in the Auto Scaling Group should be terminated. The allowed values are `OldestInstance`, `NewestInstance`, `OldestLaunchConfiguration`, `ClosestToNextInstanceHour`, `OldestLaunchTemplate`, `AllocationStrategy`, `Default`. Additionally, the ARN of a Lambda function can be specified for custom termination policies.
        pub termination_policies: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Attaches one or more traffic sources to the specified Auto Scaling group.
        pub traffic_sources: pulumi_wasm_rust::Output<
            Vec<super::super::types::autoscaling::GroupTrafficSource>,
        >,
        /// List of subnet IDs to launch resources in. Subnets automatically determine which availability zones the group will reside. Conflicts with `availability_zones`.
        pub vpc_zone_identifiers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Maximum
        /// [duration](https://golang.org/pkg/time/#ParseDuration) that the provider should
        /// wait for ASG instances to be healthy before timing out. (See also Waiting
        /// for Capacity below.) Setting this to "0" causes
        /// the provider to skip all Capacity Waiting behavior.
        pub wait_for_capacity_timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// Setting this will cause Pulumi to wait
        /// for exactly this number of healthy instances from this Auto Scaling Group in
        /// all attached load balancers on both create and update operations. (Takes
        /// precedence over `min_elb_capacity` behavior.)
        /// (See also Waiting for Capacity below.)
        pub wait_for_elb_capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// If this block is configured, add a [Warm Pool](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-warm-pools.html)
        /// to the specified Auto Scaling group. Defined below
        pub warm_pool: pulumi_wasm_rust::Output<
            Option<super::super::types::autoscaling::GroupWarmPool>,
        >,
        /// Current size of the warm pool.
        pub warm_pool_size: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_distribution_binding = args
            .availability_zone_distribution
            .get_output(context)
            .get_inner();
        let availability_zones_binding = args
            .availability_zones
            .get_output(context)
            .get_inner();
        let capacity_rebalance_binding = args
            .capacity_rebalance
            .get_output(context)
            .get_inner();
        let context_binding = args.context.get_output(context).get_inner();
        let default_cooldown_binding = args
            .default_cooldown
            .get_output(context)
            .get_inner();
        let default_instance_warmup_binding = args
            .default_instance_warmup
            .get_output(context)
            .get_inner();
        let desired_capacity_binding = args
            .desired_capacity
            .get_output(context)
            .get_inner();
        let desired_capacity_type_binding = args
            .desired_capacity_type
            .get_output(context)
            .get_inner();
        let enabled_metrics_binding = args
            .enabled_metrics
            .get_output(context)
            .get_inner();
        let force_delete_binding = args.force_delete.get_output(context).get_inner();
        let force_delete_warm_pool_binding = args
            .force_delete_warm_pool
            .get_output(context)
            .get_inner();
        let health_check_grace_period_binding = args
            .health_check_grace_period
            .get_output(context)
            .get_inner();
        let health_check_type_binding = args
            .health_check_type
            .get_output(context)
            .get_inner();
        let ignore_failed_scaling_activities_binding = args
            .ignore_failed_scaling_activities
            .get_output(context)
            .get_inner();
        let initial_lifecycle_hooks_binding = args
            .initial_lifecycle_hooks
            .get_output(context)
            .get_inner();
        let instance_maintenance_policy_binding = args
            .instance_maintenance_policy
            .get_output(context)
            .get_inner();
        let instance_refresh_binding = args
            .instance_refresh
            .get_output(context)
            .get_inner();
        let launch_configuration_binding = args
            .launch_configuration
            .get_output(context)
            .get_inner();
        let launch_template_binding = args
            .launch_template
            .get_output(context)
            .get_inner();
        let load_balancers_binding = args.load_balancers.get_output(context).get_inner();
        let max_instance_lifetime_binding = args
            .max_instance_lifetime
            .get_output(context)
            .get_inner();
        let max_size_binding = args.max_size.get_output(context).get_inner();
        let metrics_granularity_binding = args
            .metrics_granularity
            .get_output(context)
            .get_inner();
        let min_elb_capacity_binding = args
            .min_elb_capacity
            .get_output(context)
            .get_inner();
        let min_size_binding = args.min_size.get_output(context).get_inner();
        let mixed_instances_policy_binding = args
            .mixed_instances_policy
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let placement_group_binding = args
            .placement_group
            .get_output(context)
            .get_inner();
        let protect_from_scale_in_binding = args
            .protect_from_scale_in
            .get_output(context)
            .get_inner();
        let service_linked_role_arn_binding = args
            .service_linked_role_arn
            .get_output(context)
            .get_inner();
        let suspended_processes_binding = args
            .suspended_processes
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_group_arns_binding = args
            .target_group_arns
            .get_output(context)
            .get_inner();
        let termination_policies_binding = args
            .termination_policies
            .get_output(context)
            .get_inner();
        let traffic_sources_binding = args
            .traffic_sources
            .get_output(context)
            .get_inner();
        let vpc_zone_identifiers_binding = args
            .vpc_zone_identifiers
            .get_output(context)
            .get_inner();
        let wait_for_capacity_timeout_binding = args
            .wait_for_capacity_timeout
            .get_output(context)
            .get_inner();
        let wait_for_elb_capacity_binding = args
            .wait_for_elb_capacity
            .get_output(context)
            .get_inner();
        let warm_pool_binding = args.warm_pool.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZoneDistribution".into(),
                    value: &availability_zone_distribution_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "capacityRebalance".into(),
                    value: &capacity_rebalance_binding,
                },
                register_interface::ObjectField {
                    name: "context".into(),
                    value: &context_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCooldown".into(),
                    value: &default_cooldown_binding,
                },
                register_interface::ObjectField {
                    name: "defaultInstanceWarmup".into(),
                    value: &default_instance_warmup_binding,
                },
                register_interface::ObjectField {
                    name: "desiredCapacity".into(),
                    value: &desired_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "desiredCapacityType".into(),
                    value: &desired_capacity_type_binding,
                },
                register_interface::ObjectField {
                    name: "enabledMetrics".into(),
                    value: &enabled_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "forceDeleteWarmPool".into(),
                    value: &force_delete_warm_pool_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckGracePeriod".into(),
                    value: &health_check_grace_period_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckType".into(),
                    value: &health_check_type_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreFailedScalingActivities".into(),
                    value: &ignore_failed_scaling_activities_binding,
                },
                register_interface::ObjectField {
                    name: "initialLifecycleHooks".into(),
                    value: &initial_lifecycle_hooks_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMaintenancePolicy".into(),
                    value: &instance_maintenance_policy_binding,
                },
                register_interface::ObjectField {
                    name: "instanceRefresh".into(),
                    value: &instance_refresh_binding,
                },
                register_interface::ObjectField {
                    name: "launchConfiguration".into(),
                    value: &launch_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "launchTemplate".into(),
                    value: &launch_template_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancers".into(),
                    value: &load_balancers_binding,
                },
                register_interface::ObjectField {
                    name: "maxInstanceLifetime".into(),
                    value: &max_instance_lifetime_binding,
                },
                register_interface::ObjectField {
                    name: "maxSize".into(),
                    value: &max_size_binding,
                },
                register_interface::ObjectField {
                    name: "metricsGranularity".into(),
                    value: &metrics_granularity_binding,
                },
                register_interface::ObjectField {
                    name: "minElbCapacity".into(),
                    value: &min_elb_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "minSize".into(),
                    value: &min_size_binding,
                },
                register_interface::ObjectField {
                    name: "mixedInstancesPolicy".into(),
                    value: &mixed_instances_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "placementGroup".into(),
                    value: &placement_group_binding,
                },
                register_interface::ObjectField {
                    name: "protectFromScaleIn".into(),
                    value: &protect_from_scale_in_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLinkedRoleArn".into(),
                    value: &service_linked_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "suspendedProcesses".into(),
                    value: &suspended_processes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupArns".into(),
                    value: &target_group_arns_binding,
                },
                register_interface::ObjectField {
                    name: "terminationPolicies".into(),
                    value: &termination_policies_binding,
                },
                register_interface::ObjectField {
                    name: "trafficSources".into(),
                    value: &traffic_sources_binding,
                },
                register_interface::ObjectField {
                    name: "vpcZoneIdentifiers".into(),
                    value: &vpc_zone_identifiers_binding,
                },
                register_interface::ObjectField {
                    name: "waitForCapacityTimeout".into(),
                    value: &wait_for_capacity_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "waitForElbCapacity".into(),
                    value: &wait_for_elb_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "warmPool".into(),
                    value: &warm_pool_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone_distribution: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZoneDistribution"),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            capacity_rebalance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityRebalance"),
            ),
            context: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("context"),
            ),
            default_cooldown: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultCooldown"),
            ),
            default_instance_warmup: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultInstanceWarmup"),
            ),
            desired_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredCapacity"),
            ),
            desired_capacity_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredCapacityType"),
            ),
            enabled_metrics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabledMetrics"),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            force_delete_warm_pool: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDeleteWarmPool"),
            ),
            health_check_grace_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckGracePeriod"),
            ),
            health_check_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthCheckType"),
            ),
            ignore_failed_scaling_activities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ignoreFailedScalingActivities"),
            ),
            initial_lifecycle_hooks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("initialLifecycleHooks"),
            ),
            instance_maintenance_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceMaintenancePolicy"),
            ),
            instance_refresh: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceRefresh"),
            ),
            launch_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchConfiguration"),
            ),
            launch_template: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchTemplate"),
            ),
            load_balancers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancers"),
            ),
            max_instance_lifetime: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxInstanceLifetime"),
            ),
            max_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxSize"),
            ),
            metrics_granularity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metricsGranularity"),
            ),
            min_elb_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minElbCapacity"),
            ),
            min_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minSize"),
            ),
            mixed_instances_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mixedInstancesPolicy"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            placement_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placementGroup"),
            ),
            predicted_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predictedCapacity"),
            ),
            protect_from_scale_in: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectFromScaleIn"),
            ),
            service_linked_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceLinkedRoleArn"),
            ),
            suspended_processes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("suspendedProcesses"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_group_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetGroupArns"),
            ),
            termination_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("terminationPolicies"),
            ),
            traffic_sources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficSources"),
            ),
            vpc_zone_identifiers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcZoneIdentifiers"),
            ),
            wait_for_capacity_timeout: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForCapacityTimeout"),
            ),
            wait_for_elb_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForElbCapacity"),
            ),
            warm_pool: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("warmPool"),
            ),
            warm_pool_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("warmPoolSize"),
            ),
        }
    }
}
