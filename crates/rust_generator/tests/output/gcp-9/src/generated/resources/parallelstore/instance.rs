/// A Parallelstore Instance.
///
///
///
/// ## Example Usage
///
/// ### Parallelstore Instance Basic
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:parallelstore:Instance
///     properties:
///       instanceId: instance
///       location: us-central1-a
///       description: test instance
///       capacityGib: 12000
///       network: ${network.name}
///       fileStripeLevel: FILE_STRIPE_LEVEL_MIN
///       directoryStripeLevel: DIRECTORY_STRIPE_LEVEL_MIN
///       labels:
///         test: value
///     options:
///       dependsOn:
///         - ${default}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: network
///       autoCreateSubnetworks: true
///       mtu: 8896
///   # Create an IP address
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 24
///       network: ${network.id}
///   # Create a private connection
///   default:
///     type: gcp:servicenetworking:Connection
///     properties:
///       network: ${network.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{location}}/{{instance_id}}`
///
/// * `{{location}}/{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:parallelstore/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:parallelstore/instance:Instance default {{project}}/{{location}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:parallelstore/instance:Instance default {{location}}/{{instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Required. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB).
        #[builder(into)]
        pub capacity_gib: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the instance. 2048 characters or less.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Stripe level for directories.
        /// MIN when directory has a small number of files.
        /// MAX when directory has a large number of files.
        /// Possible values:
        /// DIRECTORY_STRIPE_LEVEL_UNSPECIFIED
        /// DIRECTORY_STRIPE_LEVEL_MIN
        /// DIRECTORY_STRIPE_LEVEL_BALANCED
        /// DIRECTORY_STRIPE_LEVEL_MAX
        #[builder(into, default)]
        pub directory_stripe_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Stripe level for files.
        /// MIN better suited for small size files.
        /// MAX higher throughput performance for larger files.
        /// Possible values:
        /// FILE_STRIPE_LEVEL_UNSPECIFIED
        /// FILE_STRIPE_LEVEL_MIN
        /// FILE_STRIPE_LEVEL_BALANCED
        /// FILE_STRIPE_LEVEL_MAX
        #[builder(into, default)]
        pub file_stripe_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The logical name of the Parallelstore instance in the user project with the following restrictions:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the customer project/ location
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Cloud Labels are a flexible and lightweight mechanism for
        /// organizing cloud resources into groups that reflect a customer's organizational
        /// needs and deployment strategies. Cloud Labels can be used to filter collections
        /// of resources. They can be used to control how resource metrics are aggregated.
        /// And they can be used as arguments to policy management rules (e.g. route, firewall,
        /// load balancing, etc.).
        /// * Label keys must be between 1 and 63 characters long and must conform to
        /// the following regular expression: `a-z{0,62}`.
        /// * Label values must be between 0 and 63 characters long and must conform
        /// to the regular expression `[a-z0-9_-]{0,63}`.
        /// * No more than 64 labels can be associated with a given resource.
        /// See https://goo.gl/xmQnxf for more information on and examples of labels.
        /// If you plan to use labels in your own code, please note that additional
        /// characters may be allowed in the future. Therefore, you are advised to use
        /// an internal label representation, such as JSON, which doesn't rely upon
        /// specific characters being disallowed.  For example, representing labels
        /// as the string:  `name + "_" + value` would prove problematic if we were to
        /// allow `"_"` in a future release. "
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of `parent`. See documentation of `projectsId`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)
        /// to which the instance is connected.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. Contains the id of the allocated IP address range
        /// associated with the private service access connection for example, \"test-default\"
        /// associated with IP range 10.0.0.0/29. If no range id is provided all ranges will
        /// be considered.
        #[builder(into, default)]
        pub reserved_ip_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Output only. List of access_points.
        /// Contains a list of IPv4 addresses used for client side configuration.
        pub access_points: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Required. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB).
        pub capacity_gib: pulumi_gestalt_rust::Output<String>,
        /// The time when the instance was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The version of DAOS software running in the instance.
        pub daos_version: pulumi_gestalt_rust::Output<String>,
        /// The description of the instance. 2048 characters or less.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Stripe level for directories.
        /// MIN when directory has a small number of files.
        /// MAX when directory has a large number of files.
        /// Possible values:
        /// DIRECTORY_STRIPE_LEVEL_UNSPECIFIED
        /// DIRECTORY_STRIPE_LEVEL_MIN
        /// DIRECTORY_STRIPE_LEVEL_BALANCED
        /// DIRECTORY_STRIPE_LEVEL_MAX
        pub directory_stripe_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. Contains the id of the allocated IP address
        /// range associated with the private service access connection for example, \"test-default\"
        /// associated with IP range 10.0.0.0/29. This field is populated by the service
        /// and contains the value currently used by the service.
        pub effective_reserved_ip_range: pulumi_gestalt_rust::Output<String>,
        /// Stripe level for files.
        /// MIN better suited for small size files.
        /// MAX higher throughput performance for larger files.
        /// Possible values:
        /// FILE_STRIPE_LEVEL_UNSPECIFIED
        /// FILE_STRIPE_LEVEL_MIN
        /// FILE_STRIPE_LEVEL_BALANCED
        /// FILE_STRIPE_LEVEL_MAX
        pub file_stripe_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The logical name of the Parallelstore instance in the user project with the following restrictions:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the customer project/ location
        ///
        ///
        /// - - -
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Cloud Labels are a flexible and lightweight mechanism for
        /// organizing cloud resources into groups that reflect a customer's organizational
        /// needs and deployment strategies. Cloud Labels can be used to filter collections
        /// of resources. They can be used to control how resource metrics are aggregated.
        /// And they can be used as arguments to policy management rules (e.g. route, firewall,
        /// load balancing, etc.).
        /// * Label keys must be between 1 and 63 characters long and must conform to
        /// the following regular expression: `a-z{0,62}`.
        /// * Label values must be between 0 and 63 characters long and must conform
        /// to the regular expression `[a-z0-9_-]{0,63}`.
        /// * No more than 64 labels can be associated with a given resource.
        /// See https://goo.gl/xmQnxf for more information on and examples of labels.
        /// If you plan to use labels in your own code, please note that additional
        /// characters may be allowed in the future. Therefore, you are advised to use
        /// an internal label representation, such as JSON, which doesn't rely upon
        /// specific characters being disallowed.  For example, representing labels
        /// as the string:  `name + "_" + value` would prove problematic if we were to
        /// allow `"_"` in a future release. "
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Part of `parent`. See documentation of `projectsId`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The resource name of the instance, in the format
        /// `projects/{project}/locations/{location}/instances/{instance_id}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)
        /// to which the instance is connected.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. Contains the id of the allocated IP address range
        /// associated with the private service access connection for example, \"test-default\"
        /// associated with IP range 10.0.0.0/29. If no range id is provided all ranges will
        /// be considered.
        pub reserved_ip_range: pulumi_gestalt_rust::Output<Option<String>>,
        /// The instance state.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// CREATING
        /// ACTIVE
        /// DELETING
        /// FAILED
        /// UPGRADING
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The time when the instance was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let capacity_gib_binding_1 = args.capacity_gib.get_output(context);
        let capacity_gib_binding = capacity_gib_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let directory_stripe_level_binding_1 = args
            .directory_stripe_level
            .get_output(context);
        let directory_stripe_level_binding = directory_stripe_level_binding_1
            .get_inner();
        let file_stripe_level_binding_1 = args.file_stripe_level.get_output(context);
        let file_stripe_level_binding = file_stripe_level_binding_1.get_inner();
        let instance_id_binding_1 = args.instance_id.get_output(context);
        let instance_id_binding = instance_id_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let network_binding_1 = args.network.get_output(context);
        let network_binding = network_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let reserved_ip_range_binding_1 = args.reserved_ip_range.get_output(context);
        let reserved_ip_range_binding = reserved_ip_range_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:parallelstore/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityGib".into(),
                    value: &capacity_gib_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "directoryStripeLevel".into(),
                    value: &directory_stripe_level_binding,
                },
                register_interface::ObjectField {
                    name: "fileStripeLevel".into(),
                    value: &file_stripe_level_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reservedIpRange".into(),
                    value: &reserved_ip_range_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            access_points: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPoints"),
            ),
            capacity_gib: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacityGib"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            daos_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("daosVersion"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            directory_stripe_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryStripeLevel"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            effective_reserved_ip_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveReservedIpRange"),
            ),
            file_stripe_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileStripeLevel"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reserved_ip_range: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservedIpRange"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
