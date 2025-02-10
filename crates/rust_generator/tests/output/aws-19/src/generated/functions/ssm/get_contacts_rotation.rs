#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_contacts_rotation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactsRotationArgs {
        /// The Amazon Resource Name (ARN) of the rotation.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContactsRotationResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
        pub contact_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name for the rotation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Information about when an on-call rotation is in effect and how long the rotation period lasts.
        pub recurrences: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssm::GetContactsRotationRecurrence>,
        >,
        /// The date and time, in RFC 3339 format, that the rotation goes into effect.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
        pub time_zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContactsRotationArgs,
    ) -> GetContactsRotationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssm/getContactsRotation:getContactsRotation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContactsRotationResult {
            arn: o.get_field("arn"),
            contact_ids: o.get_field("contactIds"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            recurrences: o.get_field("recurrences"),
            start_time: o.get_field("startTime"),
            tags: o.get_field("tags"),
            time_zone_id: o.get_field("timeZoneId"),
        }
    }
}
