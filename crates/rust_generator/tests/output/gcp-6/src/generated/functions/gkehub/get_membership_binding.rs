#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_membership_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMembershipBindingArgs {
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub membership_binding_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub membership_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetMembershipBindingResult {
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub membership_binding_id: pulumi_gestalt_rust::Output<String>,
        pub membership_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub scope: pulumi_gestalt_rust::Output<String>,
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::gkehub::GetMembershipBindingState>,
        >,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMembershipBindingArgs,
    ) -> GetMembershipBindingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let membership_binding_id_binding_1 = args
            .membership_binding_id
            .get_output(context);
        let membership_binding_id_binding = membership_binding_id_binding_1.get_inner();
        let membership_id_binding_1 = args.membership_id.get_output(context);
        let membership_id_binding = membership_id_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:gkehub/getMembershipBinding:getMembershipBinding".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "membershipBindingId".into(),
                    value: &membership_binding_id_binding,
                },
                register_interface::ObjectField {
                    name: "membershipId".into(),
                    value: &membership_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetMembershipBindingResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            membership_binding_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("membershipBindingId"),
            ),
            membership_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("membershipId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            states: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("states"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
