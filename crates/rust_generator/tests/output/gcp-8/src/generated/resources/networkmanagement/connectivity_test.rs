/// A connectivity test are a static analysis of your resource configurations
/// that enables you to evaluate connectivity to and from Google Cloud
/// resources in your Virtual Private Cloud (VPC) network.
///
///
/// To get more information about ConnectivityTest, see:
///
/// * [API documentation](https://cloud.google.com/network-intelligence-center/docs/connectivity-tests/reference/networkmanagement/rest/v1/projects.locations.global.connectivityTests)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/network-intelligence-center/docs)
///
/// ## Example Usage
///
/// ### Network Management Connectivity Test Instances
///
///
/// ```yaml
/// resources:
///   instance-test:
///     type: gcp:networkmanagement:ConnectivityTest
///     properties:
///       name: conn-test-instances
///       source:
///         instance: ${source.id}
///       destination:
///         instance: ${destination.id}
///       protocol: TCP
///       labels:
///         env: test
///   source:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: ${vpc.id}
///       name: source-vm
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: ${debian9.id}
///   destination:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: ${vpc.id}
///       name: dest-vm
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: ${debian9.id}
///   vpc:
///     type: gcp:compute:Network
///     properties:
///       name: conn-test-net
/// variables:
///   debian9:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Network Management Connectivity Test Addresses
///
///
/// ```yaml
/// resources:
///   address-test:
///     type: gcp:networkmanagement:ConnectivityTest
///     properties:
///       name: conn-test-addr
///       source:
///         ipAddress: ${["source-addr"].address}
///         projectId: ${["source-addr"].project}
///         network: ${vpc.id}
///         networkType: GCP_NETWORK
///       destination:
///         ipAddress: ${["dest-addr"].address}
///         projectId: ${["dest-addr"].project}
///         network: ${vpc.id}
///       protocol: UDP
///   vpc:
///     type: gcp:compute:Network
///     properties:
///       name: connectivity-vpc
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: connectivity-vpc-subnet
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${vpc.id}
///   source-addr:
///     type: gcp:compute:Address
///     properties:
///       name: src-addr
///       subnetwork: ${subnet.id}
///       addressType: INTERNAL
///       address: 10.0.42.42
///       region: us-central1
///   dest-addr:
///     type: gcp:compute:Address
///     properties:
///       name: dest-addr
///       subnetwork: ${subnet.id}
///       addressType: INTERNAL
///       address: 10.0.43.43
///       region: us-central1
/// ```
///
/// ## Import
///
/// ConnectivityTest can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/connectivityTests/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ConnectivityTest can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/connectivityTest:ConnectivityTest default projects/{{project}}/locations/global/connectivityTests/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/connectivityTest:ConnectivityTest default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkmanagement/connectivityTest:ConnectivityTest default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connectivity_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectivityTestArgs {
        /// The user-supplied description of the Connectivity Test. Maximum of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Destination specification of the Connectivity Test.
        /// You can use a combination of destination IP address, Compute
        /// Engine VM instance, or VPC network to uniquely identify the
        /// destination location.
        /// Even if the destination IP address is not unique, the source IP
        /// location is unique. Usually, the analysis can infer the destination
        /// endpoint from route information.
        /// If the destination you specify is a VM instance and the instance has
        /// multiple network interfaces, then you must also specify either a
        /// destination IP address or VPC network to identify the destination
        /// interface.
        /// A reachability analysis proceeds even if the destination location
        /// is ambiguous. However, the result can include endpoints that you
        /// don't intend to test.
        /// Structure is documented below.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkmanagement::ConnectivityTestDestination,
        >,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique name for the connectivity test.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP Protocol of the test. When not provided, "TCP" is assumed.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross
        /// project boundaries.
        #[builder(into, default)]
        pub related_projects: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Required. Source specification of the Connectivity Test.
        /// You can use a combination of source IP address, virtual machine
        /// (VM) instance, or Compute Engine network to uniquely identify the
        /// source location.
        /// Examples: If the source IP address is an internal IP address within
        /// a Google Cloud Virtual Private Cloud (VPC) network, then you must
        /// also specify the VPC network. Otherwise, specify the VM instance,
        /// which already contains its internal IP address and VPC network
        /// information.
        /// If the source of the test is within an on-premises network, then
        /// you must provide the destination VPC network.
        /// If the source endpoint is a Compute Engine VM instance with multiple
        /// network interfaces, the instance itself is not sufficient to
        /// identify the endpoint. So, you must also specify the source IP
        /// address or VPC network.
        /// A reachability analysis proceeds even if the source location is
        /// ambiguous. However, the test result may include endpoints that
        /// you don't intend to test.
        /// Structure is documented below.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkmanagement::ConnectivityTestSource,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectivityTestResult {
        /// The user-supplied description of the Connectivity Test. Maximum of 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. Destination specification of the Connectivity Test.
        /// You can use a combination of destination IP address, Compute
        /// Engine VM instance, or VPC network to uniquely identify the
        /// destination location.
        /// Even if the destination IP address is not unique, the source IP
        /// location is unique. Usually, the analysis can infer the destination
        /// endpoint from route information.
        /// If the destination you specify is a VM instance and the instance has
        /// multiple network interfaces, then you must also specify either a
        /// destination IP address or VPC network to identify the destination
        /// interface.
        /// A reachability analysis proceeds even if the destination location
        /// is ambiguous. However, the result can include endpoints that you
        /// don't intend to test.
        /// Structure is documented below.
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::networkmanagement::ConnectivityTestDestination,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Unique name for the connectivity test.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// IP Protocol of the test. When not provided, "TCP" is assumed.
        pub protocol: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross
        /// project boundaries.
        pub related_projects: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Required. Source specification of the Connectivity Test.
        /// You can use a combination of source IP address, virtual machine
        /// (VM) instance, or Compute Engine network to uniquely identify the
        /// source location.
        /// Examples: If the source IP address is an internal IP address within
        /// a Google Cloud Virtual Private Cloud (VPC) network, then you must
        /// also specify the VPC network. Otherwise, specify the VM instance,
        /// which already contains its internal IP address and VPC network
        /// information.
        /// If the source of the test is within an on-premises network, then
        /// you must provide the destination VPC network.
        /// If the source endpoint is a Compute Engine VM instance with multiple
        /// network interfaces, the instance itself is not sufficient to
        /// identify the endpoint. So, you must also specify the source IP
        /// address or VPC network.
        /// A reachability analysis proceeds even if the source location is
        /// ambiguous. However, the test result may include endpoints that
        /// you don't intend to test.
        /// Structure is documented below.
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::networkmanagement::ConnectivityTestSource,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectivityTestArgs,
    ) -> ConnectivityTestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let related_projects_binding = args.related_projects.get_output(context);
        let source_binding = args.source.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkmanagement/connectivityTest:ConnectivityTest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: &destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relatedProjects".into(),
                    value: &related_projects_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectivityTestResult {
            description: o.get_field("description"),
            destination: o.get_field("destination"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            protocol: o.get_field("protocol"),
            pulumi_labels: o.get_field("pulumiLabels"),
            related_projects: o.get_field("relatedProjects"),
            source: o.get_field("source"),
        }
    }
}
