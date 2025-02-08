#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        pub application_id: pulumi_gestalt_rust::Output<String>,
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apphub::GetApplicationAttribute>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub scopes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::apphub::GetApplicationScope>,
        >,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetApplicationArgs,
    ) -> GetApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:apphub/getApplication:getApplication".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
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
        GetApplicationResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopes"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
