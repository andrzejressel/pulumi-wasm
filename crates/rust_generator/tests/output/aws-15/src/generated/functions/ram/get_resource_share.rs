#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResourceShareArgs,
    ) -> GetResourceShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_owner_binding = args.resource_owner.get_output(context).get_inner();
        let resource_share_status_binding = args
            .resource_share_status
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ram/getResourceShare:getResourceShare".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceOwner".into(),
                    value: &resource_owner_binding,
                },
                register_interface::ObjectField {
                    name: "resourceShareStatus".into(),
                    value: &resource_share_status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourceShareResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owning_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("owningAccountId"),
            ),
            resource_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArns"),
            ),
            resource_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceOwner"),
            ),
            resource_share_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceShareStatus"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
