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
pub mod app_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppGroupArgs {
        /// A list of attributes
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigee::AppGroupAttribute>>,
        >,
        /// Channel identifier identifies the owner maintaining this grouping.
        #[builder(into, default)]
        pub channel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the associated storefront/marketplace.
        #[builder(into, default)]
        pub channel_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// App group name displayed in the UI
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._-$ %.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee Organization associated with the Apigee app group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Valid values are active or inactive. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as active or inactive.
        /// Possible values are: `active`, `inactive`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppGroupResult {
        /// Internal identifier that cannot be edited
        pub app_group_id: pulumi_wasm_rust::Output<String>,
        /// A list of attributes
        /// Structure is documented below.
        pub attributes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigee::AppGroupAttribute>>,
        >,
        /// Channel identifier identifies the owner maintaining this grouping.
        pub channel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the associated storefront/marketplace.
        pub channel_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Created time as milliseconds since epoch.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// App group name displayed in the UI
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Modified time as milliseconds since epoch.
        pub last_modified_at: pulumi_wasm_rust::Output<String>,
        /// Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._-$ %.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee app group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// App group name displayed in the UI
        pub organization: pulumi_wasm_rust::Output<String>,
        /// Valid values are active or inactive. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as active or inactive.
        /// Possible values are: `active`, `inactive`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AppGroupArgs) -> AppGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_inner();
        let channel_id_binding = args.channel_id.get_inner();
        let channel_uri_binding = args.channel_uri.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let name_binding = args.name.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let status_binding = args.status.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appGroupId".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "channelId".into(),
                },
                register_interface::ResultField {
                    name: "channelUri".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedAt".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "organization".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppGroupResult {
            app_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appGroupId").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            channel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelId").unwrap(),
            ),
            channel_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("channelUri").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            last_modified_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedAt").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            organization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organization").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
