#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ami_ids {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAmiIdsArgs {
        /// Limit search to users with *explicit* launch
        /// permission on  the image. Valid items are the numeric account ID or `self`.
        #[builder(into, default)]
        pub executable_users: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more name/value pairs to filter off of. There
        /// are several valid keys, for a full reference, check out
        /// [describe-images in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetAmiIdsFilter>>,
        >,
        /// If true, all deprecated AMIs are included in the response.
        /// If false, no deprecated AMIs are included in the response. If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub include_deprecated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Regex string to apply to the AMI list returned
        /// by AWS. This allows more advanced filtering not supported from the AWS API.
        /// This filtering is done locally on what AWS returns, and could have a performance
        /// impact if the result is large. Combine this with other
        /// options to narrow down the list AWS returns.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of AMI owners to limit search. At least 1 value must be specified. Valid values: an AWS account ID, `self` (the current account), or an AWS owner alias (e.g., `amazon`, `aws-marketplace`, `microsoft`).
        #[builder(into)]
        pub owners: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Used to sort AMIs by creation time.
        /// If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub sort_ascending: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetAmiIdsResult {
        pub executable_users: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiIdsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub include_deprecated: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        pub owners: pulumi_gestalt_rust::Output<Vec<String>>,
        pub sort_ascending: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAmiIdsArgs,
    ) -> GetAmiIdsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let executable_users_binding = args.executable_users.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let include_deprecated_binding = args.include_deprecated.get_output(context);
        let name_regex_binding = args.name_regex.get_output(context);
        let owners_binding = args.owners.get_output(context);
        let sort_ascending_binding = args.sort_ascending.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getAmiIds:getAmiIds".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executableUsers".into(),
                    value: executable_users_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeDeprecated".into(),
                    value: include_deprecated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameRegex".into(),
                    value: name_regex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owners".into(),
                    value: owners_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sortAscending".into(),
                    value: sort_ascending_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAmiIdsResult {
            executable_users: o.get_field("executableUsers"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            include_deprecated: o.get_field("includeDeprecated"),
            name_regex: o.get_field("nameRegex"),
            owners: o.get_field("owners"),
            sort_ascending: o.get_field("sortAscending"),
        }
    }
}
