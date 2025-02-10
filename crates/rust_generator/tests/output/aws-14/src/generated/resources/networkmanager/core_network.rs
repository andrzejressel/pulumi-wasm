/// Provides a core network resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = core_network::create(
///         "example",
///         CoreNetworkArgs::builder()
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With description
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = core_network::create(
///         "example",
///         CoreNetworkArgs::builder()
///             .description("example")
///             .global_network_id("${exampleAwsNetworkmanagerGlobalNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With tags
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkmanager:CoreNetwork
///     properties:
///       globalNetworkId: ${exampleAwsNetworkmanagerGlobalNetwork.id}
///       tags:
///         hello: world
/// ```
///
/// ### With VPC Attachment (Single Region)
///
/// The example below illustrates the scenario where your policy document has static routes pointing to VPC attachments and you want to attach your VPCs to the core network before applying the desired policy document. Set the `create_base_policy` argument to `true` if your core network does not currently have any `LIVE` policies (e.g. this is the first `pulumi up` with the core network resource), since a `LIVE` policy is required before VPCs can be attached to the core network. Otherwise, if your core network already has a `LIVE` policy, you may exclude the `create_base_policy` argument. There are 2 options to implement this:
///
/// - Option 1: Use the `base_policy_document` argument that allows the most customizations to a base policy. Use this to customize the `edge_locations` `asn`. In the example below, `us-west-2` and ASN `65500` are used in the base policy.
/// - Option 2: Use the `create_base_policy` argument only. This creates a base policy in the region specified in the `provider` block.
///
/// ### Option 1 - using base_policy_document
///
/// If you require a custom ASN for the edge location, please use the `base_policy_document` argument to pass a specific ASN. For example:
///
///
/// ### Option 2 - create_base_policy only
///
///
/// ### With VPC Attachment (Multi-Region)
///
/// The example below illustrates the scenario where your policy document has static routes pointing to VPC attachments and you want to attach your VPCs to the core network before applying the desired policy document. Set the `create_base_policy` argument of the `aws.networkmanager.CoreNetwork` resource to `true` if your core network does not currently have any `LIVE` policies (e.g. this is the first `pulumi up` with the core network resource), since a `LIVE` policy is required before VPCs can be attached to the core network. Otherwise, if your core network already has a `LIVE` policy, you may exclude the `create_base_policy` argument. For multi-region in a core network that does not yet have a `LIVE` policy, there are 2 options:
///
/// - Option 1: Use the `base_policy_document` argument that allows the most customizations to a base policy. Use this to customize the `edge_locations` `asn`. In the example below, `us-west-2`, `us-east-1` and specific ASNs are used in the base policy.
/// - Option 2: Pass a list of regions to the `aws.networkmanager.CoreNetwork` `base_policy_regions` argument. In the example below, `us-west-2` and `us-east-1` are specified in the base policy.
///
/// ### Option 1 - using base_policy_document
///
///
/// ### Option 2 - using base_policy_regions
///
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_core_network` using the core network ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/coreNetwork:CoreNetwork example core-network-0d47f6t230mz46dy4
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod core_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CoreNetworkArgs {
        /// Sets the base policy document for the core network. Refer to the [Core network policies documentation](https://docs.aws.amazon.com/network-manager/latest/cloudwan/cloudwan-policy-change-sets.html) for more information.
        #[builder(into, default)]
        pub base_policy_document: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The base policy created by setting the `create_base_policy` argument to `true` requires a region to be set in the `edge-locations`, `location` key. If `base_policy_region` is not specified, the region used in the base policy defaults to the region specified in the `provider` block.
        #[builder(into, default)]
        pub base_policy_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of regions to add to the base policy. The base policy created by setting the `create_base_policy` argument to `true` requires one or more regions to be set in the `edge-locations`, `location` key. If `base_policy_regions` is not specified, the region used in the base policy defaults to the region specified in the `provider` block.
        #[builder(into, default)]
        pub base_policy_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies whether to create a base policy when a core network is created or updated. A base policy is created and set to `LIVE` to allow attachments to the core network (e.g. VPC Attachments) before applying a policy document provided using the `aws.networkmanager.CoreNetworkPolicyAttachment` resource. This base policy is needed if your core network does not have any `LIVE` policies and your policy document has static routes pointing to VPC attachments and you want to attach your VPCs to the core network before applying the desired policy document. Valid values are `true` or `false`. An example of this Pulumi snippet can be found above for VPC Attachment in a single region and for VPC Attachment multi-region. An example base policy is shown below. This base policy is overridden with the policy that you specify in the `aws.networkmanager.CoreNetworkPolicyAttachment` resource.
        ///
        /// ```json
        /// {
        /// "version": "2021.12",
        /// "core-network-configuration": {
        /// "asn-ranges": [
        /// "64512-65534"
        /// ],
        /// "vpn-ecmp-support": false,
        /// "edge-locations": [
        /// {
        /// "location": "us-east-1"
        /// }
        /// ]
        /// },
        /// "segments": [
        /// {
        /// "name": "segment",
        /// "description": "base-policy",
        /// "isolate-attachments": false,
        /// "require-attachment-acceptance": false
        /// }
        /// ]
        /// }
        /// ```
        #[builder(into, default)]
        pub create_base_policy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description of the Core Network.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the global network that a core network will be a part of.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the Core Network. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CoreNetworkResult {
        /// Core Network Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Sets the base policy document for the core network. Refer to the [Core network policies documentation](https://docs.aws.amazon.com/network-manager/latest/cloudwan/cloudwan-policy-change-sets.html) for more information.
        pub base_policy_document: pulumi_gestalt_rust::Output<Option<String>>,
        /// The base policy created by setting the `create_base_policy` argument to `true` requires a region to be set in the `edge-locations`, `location` key. If `base_policy_region` is not specified, the region used in the base policy defaults to the region specified in the `provider` block.
        pub base_policy_region: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of regions to add to the base policy. The base policy created by setting the `create_base_policy` argument to `true` requires one or more regions to be set in the `edge-locations`, `location` key. If `base_policy_regions` is not specified, the region used in the base policy defaults to the region specified in the `provider` block.
        pub base_policy_regions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies whether to create a base policy when a core network is created or updated. A base policy is created and set to `LIVE` to allow attachments to the core network (e.g. VPC Attachments) before applying a policy document provided using the `aws.networkmanager.CoreNetworkPolicyAttachment` resource. This base policy is needed if your core network does not have any `LIVE` policies and your policy document has static routes pointing to VPC attachments and you want to attach your VPCs to the core network before applying the desired policy document. Valid values are `true` or `false`. An example of this Pulumi snippet can be found above for VPC Attachment in a single region and for VPC Attachment multi-region. An example base policy is shown below. This base policy is overridden with the policy that you specify in the `aws.networkmanager.CoreNetworkPolicyAttachment` resource.
        ///
        /// ```json
        /// {
        /// "version": "2021.12",
        /// "core-network-configuration": {
        /// "asn-ranges": [
        /// "64512-65534"
        /// ],
        /// "vpn-ecmp-support": false,
        /// "edge-locations": [
        /// {
        /// "location": "us-east-1"
        /// }
        /// ]
        /// },
        /// "segments": [
        /// {
        /// "name": "segment",
        /// "description": "base-policy",
        /// "isolate-attachments": false,
        /// "require-attachment-acceptance": false
        /// }
        /// ]
        /// }
        /// ```
        pub create_base_policy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Timestamp when a core network was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the Core Network.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more blocks detailing the edges within a core network. Detailed below.
        pub edges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkmanager::CoreNetworkEdge>,
        >,
        /// The ID of the global network that a core network will be a part of.
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// One or more blocks detailing the segments within a core network. Detailed below.
        pub segments: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkmanager::CoreNetworkSegment>,
        >,
        /// Current state of a core network.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the Core Network. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: CoreNetworkArgs,
    ) -> CoreNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_policy_document_binding = args.base_policy_document.get_output(context);
        let base_policy_region_binding = args.base_policy_region.get_output(context);
        let base_policy_regions_binding = args.base_policy_regions.get_output(context);
        let create_base_policy_binding = args.create_base_policy.get_output(context);
        let description_binding = args.description.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/coreNetwork:CoreNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basePolicyDocument".into(),
                    value: base_policy_document_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basePolicyRegion".into(),
                    value: base_policy_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "basePolicyRegions".into(),
                    value: base_policy_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createBasePolicy".into(),
                    value: create_base_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CoreNetworkResult {
            arn: o.get_field("arn"),
            base_policy_document: o.get_field("basePolicyDocument"),
            base_policy_region: o.get_field("basePolicyRegion"),
            base_policy_regions: o.get_field("basePolicyRegions"),
            create_base_policy: o.get_field("createBasePolicy"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            edges: o.get_field("edges"),
            global_network_id: o.get_field("globalNetworkId"),
            segments: o.get_field("segments"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
