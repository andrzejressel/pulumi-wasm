/// An `AppGroup` in Apigee.
///
///
/// To get more information about AppGroup, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.appgroups)
/// * How-to Guides
///     * [Organizing client app ownership](https://cloud.google.com/apigee/docs/api-platform/publish/organizing-client-app-ownership)
///
/// ## Example Usage
///
/// ### Apigee App Group Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: instance
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       peeringCidrRange: SLASH_22
///   apigeeAppGroup:
///     type: gcp:apigee:AppGroup
///     name: apigee_app_group
///     properties:
///       name: my-app-group
///       displayName: Test app group
///       channelId: storefront
///       channelUri: https://my-dev-portal.org/groups/my-group
///       status: active
///       orgId: ${apigeeOrg.id}
///     options:
///       dependsOn:
///         - ${apigeeInstance}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee App Group With Attributes
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: instance
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       peeringCidrRange: SLASH_22
///   apigeeAppGroup:
///     type: gcp:apigee:AppGroup
///     name: apigee_app_group
///     properties:
///       name: my-app-group
///       displayName: Test app group
///       channelId: storefront
///       channelUri: https://my-dev-portal.org/groups/my-group
///       status: active
///       orgId: ${apigeeOrg.id}
///       attributes:
///         - name: business_unit
///           value: HR
///         - name: department
///           value: payroll
///     options:
///       dependsOn:
///         - ${apigeeInstance}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// AppGroup can be imported using any of these accepted formats:
///
/// * `{{org_id}}/appgroups/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, AppGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/appGroup:AppGroup default {{org_id}}/appgroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/appGroup:AppGroup default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppGroupArgs {
        /// A list of attributes
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apigee::AppGroupAttribute>>,
        >,
        /// Channel identifier identifies the owner maintaining this grouping.
        #[builder(into, default)]
        pub channel_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the associated storefront/marketplace.
        #[builder(into, default)]
        pub channel_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// App group name displayed in the UI
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._-$ %.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee Organization associated with the Apigee app group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Valid values are active or inactive. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as active or inactive.
        /// Possible values are: `active`, `inactive`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppGroupResult {
        /// Internal identifier that cannot be edited
        pub app_group_id: pulumi_gestalt_rust::Output<String>,
        /// A list of attributes
        /// Structure is documented below.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apigee::AppGroupAttribute>>,
        >,
        /// Channel identifier identifies the owner maintaining this grouping.
        pub channel_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A reference to the associated storefront/marketplace.
        pub channel_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Created time as milliseconds since epoch.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// App group name displayed in the UI
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Modified time as milliseconds since epoch.
        pub last_modified_at: pulumi_gestalt_rust::Output<String>,
        /// Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._-$ %.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee app group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// App group name displayed in the UI
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// Valid values are active or inactive. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as active or inactive.
        /// Possible values are: `active`, `inactive`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AppGroupArgs,
    ) -> AppGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attributes_binding_1 = args.attributes.get_output(context);
        let attributes_binding = attributes_binding_1.get_inner();
        let channel_id_binding_1 = args.channel_id.get_output(context);
        let channel_id_binding = channel_id_binding_1.get_inner();
        let channel_uri_binding_1 = args.channel_uri.get_output(context);
        let channel_uri_binding = channel_uri_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let org_id_binding_1 = args.org_id.get_output(context);
        let org_id_binding = org_id_binding_1.get_inner();
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/appGroup:AppGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "channelUri".into(),
                    value: &channel_uri_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AppGroupResult {
            app_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appGroupId"),
            ),
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            channel_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("channelId"),
            ),
            channel_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("channelUri"),
            ),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            last_modified_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedAt"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            org_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orgId"),
            ),
            organization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organization"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
