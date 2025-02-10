#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_organization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationArgs {
        /// The domain name of the Organization.
        ///
        /// > **NOTE:** One of `organization` or `domain` must be specified.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Organization's numeric ID, including an optional `organizations/` prefix.
        #[builder(into, default)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationResult {
        /// Timestamp when the Organization was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The Google for Work customer ID of the Organization.
        pub directory_customer_id: pulumi_gestalt_rust::Output<String>,
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Organization's current lifecycle state.
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Organization in the form `organizations/{organization_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Organization ID.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        pub organization: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrganizationArgs,
    ) -> GetOrganizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getOrganization:getOrganization".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: organization_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrganizationResult {
            create_time: o.get_field("createTime"),
            directory_customer_id: o.get_field("directoryCustomerId"),
            domain: o.get_field("domain"),
            id: o.get_field("id"),
            lifecycle_state: o.get_field("lifecycleState"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            organization: o.get_field("organization"),
        }
    }
}
