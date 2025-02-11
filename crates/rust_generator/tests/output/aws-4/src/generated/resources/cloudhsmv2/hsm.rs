/// Creates an HSM module in Amazon CloudHSM v2 cluster.
///
/// ## Example Usage
///
/// The following example below creates an HSM module in CloudHSM cluster.
///
/// ```yaml
/// resources:
///   cloudhsmV2Hsm:
///     type: aws:cloudhsmv2:Hsm
///     name: cloudhsm_v2_hsm
///     properties:
///       subnetId: ${cluster.subnetIds[0]}
///       clusterId: ${cluster.clusterId}
/// variables:
///   cluster:
///     fn::invoke:
///       function: aws:cloudhsmv2:getCluster
///       arguments:
///         clusterId: ${cloudhsmClusterId}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import HSM modules using their HSM ID. For example:
///
/// ```sh
/// $ pulumi import aws:cloudhsmv2/hsm:Hsm bar hsm-quo8dahtaca
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hsm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HsmArgs {
        /// The IDs of AZ in which HSM module will be located. Conflicts with `subnet_id`.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of Cloud HSM v2 cluster to which HSM will be added.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IP address of HSM module. Must be within the CIDR of selected subnet.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of subnet in which HSM module will be located. Conflicts with `availability_zone`.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HsmResult {
        /// The IDs of AZ in which HSM module will be located. Conflicts with `subnet_id`.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The ID of Cloud HSM v2 cluster to which HSM will be added.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the ENI interface allocated for HSM module.
        pub hsm_eni_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the HSM module.
        pub hsm_id: pulumi_gestalt_rust::Output<String>,
        /// The state of the HSM module.
        pub hsm_state: pulumi_gestalt_rust::Output<String>,
        /// The IP address of HSM module. Must be within the CIDR of selected subnet.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The ID of subnet in which HSM module will be located. Conflicts with `availability_zone`.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HsmArgs,
    ) -> HsmResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let cluster_id_binding = args.cluster_id.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudhsmv2/hsm:Hsm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HsmResult {
            availability_zone: o.get_field("availabilityZone"),
            cluster_id: o.get_field("clusterId"),
            hsm_eni_id: o.get_field("hsmEniId"),
            hsm_id: o.get_field("hsmId"),
            hsm_state: o.get_field("hsmState"),
            ip_address: o.get_field("ipAddress"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
