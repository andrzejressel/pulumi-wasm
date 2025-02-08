#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ledger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLedgerArgs {
        /// Friendly name of the ledger to match.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLedgerResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub permissions_mode: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLedgerArgs,
    ) -> GetLedgerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:qldb/getLedger:getLedger".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLedgerResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKey"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            permissions_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissionsMode"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
