#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_supported_database_flags {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSupportedDatabaseFlagsArgs {
        /// The canonical id of the location. For example: `us-east1`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSupportedDatabaseFlagsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// Contains a list of `flag`, which contains the details about a particular flag.
        pub supported_database_flags: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::alloydb::GetSupportedDatabaseFlagsSupportedDatabaseFlag,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSupportedDatabaseFlagsArgs,
    ) -> GetSupportedDatabaseFlagsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:alloydb/getSupportedDatabaseFlags:getSupportedDatabaseFlags"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSupportedDatabaseFlagsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            supported_database_flags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedDatabaseFlags"),
            ),
        }
    }
}
