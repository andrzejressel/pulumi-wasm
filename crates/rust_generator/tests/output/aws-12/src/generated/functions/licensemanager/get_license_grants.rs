#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_license_grants {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLicenseGrantsArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::licensemanager::GetLicenseGrantsFilter>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLicenseGrantsResult {
        /// List of all the license grant ARNs found.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::licensemanager::GetLicenseGrantsFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLicenseGrantsArgs,
    ) -> GetLicenseGrantsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:licensemanager/getLicenseGrants:getLicenseGrants".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLicenseGrantsResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
