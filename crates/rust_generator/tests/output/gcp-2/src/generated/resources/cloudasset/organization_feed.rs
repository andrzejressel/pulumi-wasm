/// Describes a Cloud Asset Inventory feed used to to listen to asset updates.
///
///
/// To get more information about OrganizationFeed, see:
///
/// * [API documentation](https://cloud.google.com/asset-inventory/docs/reference/rest/)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/asset-inventory/docs)
///
/// ## Example Usage
///
/// ### Cloud Asset Organization Feed
///
///
/// ```yaml
/// resources:
///   # Create a feed that sends notifications about network resource updates under a
///   # particular organization.
///   organizationFeed:
///     type: gcp:cloudasset:OrganizationFeed
///     name: organization_feed
///     properties:
///       billingProject: my-project-name
///       orgId: '123456789'
///       feedId: network-updates
///       contentType: RESOURCE
///       assetTypes:
///         - compute.googleapis.com/Subnetwork
///         - compute.googleapis.com/Network
///       feedOutputConfig:
///         pubsubDestination:
///           topic: ${feedOutput.id}
///       condition:
///         expression: |
///           !temporal_asset.deleted &&
///           temporal_asset.prior_asset_state == google.cloud.asset.v1.TemporalAsset.PriorAssetState.DOES_NOT_EXIST
///         title: created
///         description: Send notifications on creation events
///   # The topic where the resource change notifications will be sent.
///   feedOutput:
///     type: gcp:pubsub:Topic
///     name: feed_output
///     properties:
///       project: my-project-name
///       name: network-updates
/// variables:
///   # Find the project number of the project whose identity will be used for sending
///   # the asset change notifications.
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments:
///         projectId: my-project-name
/// ```
///
/// ## Import
///
/// OrganizationFeed can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/feeds/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, OrganizationFeed can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudasset/organizationFeed:OrganizationFeed default organizations/{{org_id}}/feeds/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudasset/organizationFeed:OrganizationFeed default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_feed {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationFeedArgs {
        /// A list of the full names of the assets to receive updates. You must specify either or both of assetNames and assetTypes.
        /// Only asset updates matching specified assetNames and assetTypes are exported to the feed. For example:
        /// //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1. See
        /// https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info.
        #[builder(into, default)]
        pub asset_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of types of the assets to receive updates. You must specify either or both of assetNames and assetTypes. Only
        /// asset updates matching specified assetNames and assetTypes are exported to the feed. For example:
        /// "compute.googleapis.com/Disk" See https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all
        /// supported asset types.
        #[builder(into, default)]
        pub asset_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The project whose identity will be used when sending messages to the
        /// destination pubsub topic. It also specifies the project for API
        /// enablement check, quota, and billing.
        #[builder(into)]
        pub billing_project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A condition which determines whether an asset update should be published. If specified, an asset will be returned only
        /// when the expression evaluates to true. When set, expression field must be a valid CEL expression on a TemporalAsset with
        /// name temporal_asset. Example: a Feed with expression "temporal_asset.deleted == true" will only publish Asset deletions.
        /// Other fields of condition are optional.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudasset::OrganizationFeedCondition>,
        >,
        /// Asset content type. If not specified, no content but the asset name and type will be returned. Possible values:
        /// ["CONTENT_TYPE_UNSPECIFIED", "RESOURCE", "IAM_POLICY", "ORG_POLICY", "OS_INVENTORY", "ACCESS_POLICY"]
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This is the client-assigned asset feed identifier and it needs to be unique under a specific parent.
        #[builder(into)]
        pub feed_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Output configuration for asset feed destination.
        /// Structure is documented below.
        #[builder(into)]
        pub feed_output_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudasset::OrganizationFeedFeedOutputConfig,
        >,
        /// The organization this feed should be created in.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationFeedResult {
        /// A list of the full names of the assets to receive updates. You must specify either or both of assetNames and assetTypes.
        /// Only asset updates matching specified assetNames and assetTypes are exported to the feed. For example:
        /// //compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1. See
        /// https://cloud.google.com/apis/design/resourceNames#fullResourceName for more info.
        pub asset_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of types of the assets to receive updates. You must specify either or both of assetNames and assetTypes. Only
        /// asset updates matching specified assetNames and assetTypes are exported to the feed. For example:
        /// "compute.googleapis.com/Disk" See https://cloud.google.com/asset-inventory/docs/supported-asset-types for a list of all
        /// supported asset types.
        pub asset_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The project whose identity will be used when sending messages to the
        /// destination pubsub topic. It also specifies the project for API
        /// enablement check, quota, and billing.
        pub billing_project: pulumi_gestalt_rust::Output<String>,
        /// A condition which determines whether an asset update should be published. If specified, an asset will be returned only
        /// when the expression evaluates to true. When set, expression field must be a valid CEL expression on a TemporalAsset with
        /// name temporal_asset. Example: a Feed with expression "temporal_asset.deleted == true" will only publish Asset deletions.
        /// Other fields of condition are optional.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudasset::OrganizationFeedCondition>,
        >,
        /// Asset content type. If not specified, no content but the asset name and type will be returned. Possible values:
        /// ["CONTENT_TYPE_UNSPECIFIED", "RESOURCE", "IAM_POLICY", "ORG_POLICY", "OS_INVENTORY", "ACCESS_POLICY"]
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// This is the client-assigned asset feed identifier and it needs to be unique under a specific parent.
        pub feed_id: pulumi_gestalt_rust::Output<String>,
        /// Output configuration for asset feed destination.
        /// Structure is documented below.
        pub feed_output_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudasset::OrganizationFeedFeedOutputConfig,
        >,
        /// The format will be organizations/{organization_number}/feeds/{client-assigned_feed_identifier}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization this feed should be created in.
        pub org_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationFeedArgs,
    ) -> OrganizationFeedResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asset_names_binding = args.asset_names.get_output(context);
        let asset_types_binding = args.asset_types.get_output(context);
        let billing_project_binding = args.billing_project.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let feed_id_binding = args.feed_id.get_output(context);
        let feed_output_config_binding = args.feed_output_config.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudasset/organizationFeed:OrganizationFeed".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetNames".into(),
                    value: &asset_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetTypes".into(),
                    value: &asset_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingProject".into(),
                    value: &billing_project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "feedId".into(),
                    value: &feed_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "feedOutputConfig".into(),
                    value: &feed_output_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationFeedResult {
            asset_names: o.get_field("assetNames"),
            asset_types: o.get_field("assetTypes"),
            billing_project: o.get_field("billingProject"),
            condition: o.get_field("condition"),
            content_type: o.get_field("contentType"),
            feed_id: o.get_field("feedId"),
            feed_output_config: o.get_field("feedOutputConfig"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
        }
    }
}
