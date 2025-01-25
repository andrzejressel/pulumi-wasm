pub mod get_resource_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceShareArgs {
        /// Filter used to scope the list e.g., by tags. See [related docs] (https://docs.aws.amazon.com/ram/latest/APIReference/API_TagFilter.html).
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ram::GetResourceShareFilter>>,
        >,
        /// Name of the resource share to retrieve.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Owner of the resource share. Valid values are `SELF` or `OTHER-ACCOUNTS`.
        #[builder(into)]
        pub resource_owner: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies that you want to retrieve details of only those resource shares that have this status. Valid values are `PENDING`, `ACTIVE`, `FAILED`, `DELETING`, and `DELETED`.
        #[builder(into, default)]
        pub resource_share_status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags attached to the resource share.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResourceShareResult {
        /// ARN of the resource share.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ram::GetResourceShareFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns the resource share.
        pub owning_account_id: pulumi_wasm_rust::Output<String>,
        /// A list of resource ARNs associated with the resource share.
        pub resource_arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        pub resource_share_status: pulumi_wasm_rust::Output<Option<String>>,
        /// Status of the resource share.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Tags attached to the resource share.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResourceShareArgs,
    ) -> GetResourceShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owningAccountId".into(),
                },
                register_interface::ResultField {
                    name: "resourceArns".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "resourceShareStatus".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResourceShareResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owning_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owningAccountId").unwrap(),
            ),
            resource_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArns").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            resource_share_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceShareStatus").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
