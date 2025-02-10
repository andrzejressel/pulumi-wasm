/// An authorized organizations description describes a list of organizations
/// (1) that have been authorized to use certain asset (for example, device) data
/// owned by different organizations at the enforcement points, or (2) with certain
/// asset (for example, device) have been authorized to access the resources in
/// another organization at the enforcement points.
///
///
/// To get more information about AuthorizedOrgsDesc, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.authorizedOrgsDescs)
/// * How-to Guides
///     * [gcloud docs](https://cloud.google.com/beyondcorp-enterprise/docs/cross-org-authorization)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Authorized Orgs Desc Basic
///
///
/// ```yaml
/// resources:
///   authorized-orgs-desc:
///     type: gcp:accesscontextmanager:AuthorizedOrgsDesc
///     properties:
///       parent: accessPolicies/${["test-access"].name}
///       name: accessPolicies/${["test-access"].name}/authorizedOrgsDescs/fakeDescName
///       authorizationType: AUTHORIZATION_TYPE_TRUST
///       assetType: ASSET_TYPE_CREDENTIAL_STRENGTH
///       authorizationDirection: AUTHORIZATION_DIRECTION_TO
///       orgs:
///         - organizations/12345
///         - organizations/98765
///   test-access:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/
///       title: my policy
/// ```
///
/// ## Import
///
/// AuthorizedOrgsDesc can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AuthorizedOrgsDesc can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/authorizedOrgsDesc:AuthorizedOrgsDesc default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorized_orgs_desc {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizedOrgsDescArgs {
        /// The type of entities that need to use the authorization relationship during
        /// evaluation, such as a device. Valid values are "ASSET_TYPE_DEVICE" and
        /// "ASSET_TYPE_CREDENTIAL_STRENGTH".
        /// Possible values are: `ASSET_TYPE_DEVICE`, `ASSET_TYPE_CREDENTIAL_STRENGTH`.
        #[builder(into, default)]
        pub asset_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The direction of the authorization relationship between this organization
        /// and the organizations listed in the "orgs" field. The valid values for this
        /// field include the following:
        /// AUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic
        /// in the organizations listed in the `orgs` field.
        /// AUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the `orgs`
        /// field to evaluate the traffic in this organization.
        /// For the authorization relationship to take effect, all of the organizations
        /// must authorize and specify the appropriate relationship direction. For
        /// example, if organization A authorized organization B and C to evaluate its
        /// traffic, by specifying "AUTHORIZATION_DIRECTION_TO" as the authorization
        /// direction, organizations B and C must specify
        /// "AUTHORIZATION_DIRECTION_FROM" as the authorization direction in their
        /// "AuthorizedOrgsDesc" resource.
        /// Possible values are: `AUTHORIZATION_DIRECTION_TO`, `AUTHORIZATION_DIRECTION_FROM`.
        #[builder(into, default)]
        pub authorization_direction: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A granular control type for authorization levels. Valid value is "AUTHORIZATION_TYPE_TRUST".
        /// Possible values are: `AUTHORIZATION_TYPE_TRUST`.
        #[builder(into, default)]
        pub authorization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource name for the `AuthorizedOrgsDesc`. Format:
        /// `accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}`.
        /// The `authorized_orgs_desc` component must begin with a letter, followed by
        /// alphanumeric characters or `_`.
        /// After you create an `AuthorizedOrgsDesc`, you cannot change its `name`.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of organization ids in this AuthorizedOrgsDesc.
        /// Format: `organizations/<org_number>`
        /// Example: `organizations/123456`
        #[builder(into, default)]
        pub orgs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Required. Resource name for the access policy which owns this `AuthorizedOrgsDesc`.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizedOrgsDescResult {
        /// The type of entities that need to use the authorization relationship during
        /// evaluation, such as a device. Valid values are "ASSET_TYPE_DEVICE" and
        /// "ASSET_TYPE_CREDENTIAL_STRENGTH".
        /// Possible values are: `ASSET_TYPE_DEVICE`, `ASSET_TYPE_CREDENTIAL_STRENGTH`.
        pub asset_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The direction of the authorization relationship between this organization
        /// and the organizations listed in the "orgs" field. The valid values for this
        /// field include the following:
        /// AUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic
        /// in the organizations listed in the `orgs` field.
        /// AUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the `orgs`
        /// field to evaluate the traffic in this organization.
        /// For the authorization relationship to take effect, all of the organizations
        /// must authorize and specify the appropriate relationship direction. For
        /// example, if organization A authorized organization B and C to evaluate its
        /// traffic, by specifying "AUTHORIZATION_DIRECTION_TO" as the authorization
        /// direction, organizations B and C must specify
        /// "AUTHORIZATION_DIRECTION_FROM" as the authorization direction in their
        /// "AuthorizedOrgsDesc" resource.
        /// Possible values are: `AUTHORIZATION_DIRECTION_TO`, `AUTHORIZATION_DIRECTION_FROM`.
        pub authorization_direction: pulumi_gestalt_rust::Output<Option<String>>,
        /// A granular control type for authorization levels. Valid value is "AUTHORIZATION_TYPE_TRUST".
        /// Possible values are: `AUTHORIZATION_TYPE_TRUST`.
        pub authorization_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the AuthorizedOrgsDesc was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Resource name for the `AuthorizedOrgsDesc`. Format:
        /// `accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}`.
        /// The `authorized_orgs_desc` component must begin with a letter, followed by
        /// alphanumeric characters or `_`.
        /// After you create an `AuthorizedOrgsDesc`, you cannot change its `name`.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The list of organization ids in this AuthorizedOrgsDesc.
        /// Format: `organizations/<org_number>`
        /// Example: `organizations/123456`
        pub orgs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Required. Resource name for the access policy which owns this `AuthorizedOrgsDesc`.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Time the AuthorizedOrgsDesc was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizedOrgsDescArgs,
    ) -> AuthorizedOrgsDescResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let asset_type_binding = args.asset_type.get_output(context);
        let authorization_direction_binding = args
            .authorization_direction
            .get_output(context);
        let authorization_type_binding = args.authorization_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let orgs_binding = args.orgs.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/authorizedOrgsDesc:AuthorizedOrgsDesc"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetType".into(),
                    value: asset_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationDirection".into(),
                    value: authorization_direction_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationType".into(),
                    value: authorization_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgs".into(),
                    value: orgs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizedOrgsDescResult {
            asset_type: o.get_field("assetType"),
            authorization_direction: o.get_field("authorizationDirection"),
            authorization_type: o.get_field("authorizationType"),
            create_time: o.get_field("createTime"),
            name: o.get_field("name"),
            orgs: o.get_field("orgs"),
            parent: o.get_field("parent"),
            update_time: o.get_field("updateTime"),
        }
    }
}
