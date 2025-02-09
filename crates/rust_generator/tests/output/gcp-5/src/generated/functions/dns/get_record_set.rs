#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_record_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRecordSetArgs {
        /// The Name of the zone.
        #[builder(into)]
        pub managed_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DNS name for the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project for the Google Cloud.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The RRSet type. [See this table for supported types](https://cloud.google.com/dns/docs/records#record_type).
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRecordSetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_zone: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The string data for the records in this record set.
        pub rrdatas: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The time-to-live of this record set (seconds).
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRecordSetArgs,
    ) -> GetRecordSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let managed_zone_binding_1 = args.managed_zone.get_output(context);
        let managed_zone_binding = managed_zone_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getRecordSet:getRecordSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedZone".into(),
                    value: &managed_zone_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRecordSetResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedZone"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            rrdatas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rrdatas"),
            ),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
