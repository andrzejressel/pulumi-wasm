/// Resource for managing an AWS VPC Lattice Service Network VPC Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service_network_vpc_association::create(
///         "example",
///         ServiceNetworkVpcAssociationArgs::builder()
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .service_network_identifier("${exampleAwsVpclatticeServiceNetwork.id}")
///             .vpc_identifier("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Service Network VPC Association using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/serviceNetworkVpcAssociation:ServiceNetworkVpcAssociation example snsa-05e2474658a88f6ba
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_network_vpc_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceNetworkVpcAssociationArgs {
        /// The IDs of the security groups.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network. You must use the ARN if the resources specified in the operation are in different accounts.
        /// The following arguments are optional:
        #[builder(into)]
        pub service_network_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the VPC.
        #[builder(into)]
        pub vpc_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceNetworkVpcAssociationResult {
        /// The ARN of the Association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The account that created the association.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// The IDs of the security groups.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network. You must use the ARN if the resources specified in the operation are in different accounts.
        /// The following arguments are optional:
        pub service_network_identifier: pulumi_gestalt_rust::Output<String>,
        /// The operations status. Valid Values are CREATE_IN_PROGRESS | ACTIVE | DELETE_IN_PROGRESS | CREATE_FAILED | DELETE_FAILED
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC.
        pub vpc_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceNetworkVpcAssociationArgs,
    ) -> ServiceNetworkVpcAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let service_network_identifier_binding = args
            .service_network_identifier
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_identifier_binding = args.vpc_identifier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/serviceNetworkVpcAssociation:ServiceNetworkVpcAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceNetworkIdentifier".into(),
                    value: service_network_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcIdentifier".into(),
                    value: vpc_identifier_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceNetworkVpcAssociationResult {
            arn: o.get_field("arn"),
            created_by: o.get_field("createdBy"),
            security_group_ids: o.get_field("securityGroupIds"),
            service_network_identifier: o.get_field("serviceNetworkIdentifier"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_identifier: o.get_field("vpcIdentifier"),
        }
    }
}
