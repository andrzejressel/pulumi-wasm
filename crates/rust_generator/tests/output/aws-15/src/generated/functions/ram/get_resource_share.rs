#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceShareArgs {
        /// Filter used to scope the list e.g., by tags. See [related docs] (https://docs.aws.amazon.com/ram/latest/APIReference/API_TagFilter.html).
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ram::GetResourceShareFilter>>,
        >,
        /// Name of the resource share to retrieve.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Owner of the resource share. Valid values are `SELF` or `OTHER-ACCOUNTS`.
        #[builder(into)]
        pub resource_owner: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies that you want to retrieve details of only those resource shares that have this status. Valid values are `PENDING`, `ACTIVE`, `FAILED`, `DELETING`, and `DELETED`.
        #[builder(into, default)]
        pub resource_share_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags attached to the resource share.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResourceShareResult {
        /// ARN of the resource share.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ram::GetResourceShareFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the resource share.
        pub owning_account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of resource ARNs associated with the resource share.
        pub resource_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        pub resource_share_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Status of the resource share.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags attached to the resource share.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourceShareArgs,
    ) -> GetResourceShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_owner_binding = args.resource_owner.get_output(context);
        let resource_share_status_binding = args
            .resource_share_status
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ram/getResourceShare:getResourceShare".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceOwner".into(),
                    value: resource_owner_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceShareStatus".into(),
                    value: resource_share_status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourceShareResult {
            arn: o.get_field("arn"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owning_account_id: o.get_field("owningAccountId"),
            resource_arns: o.get_field("resourceArns"),
            resource_owner: o.get_field("resourceOwner"),
            resource_share_status: o.get_field("resourceShareStatus"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
