pub mod get_contacts_rotation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactsRotationArgs {
        /// The Amazon Resource Name (ARN) of the rotation.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContactsRotationResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the contacts to add to the rotation. The order in which you list the contacts is their shift order in the rotation schedule.
        pub contact_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name for the rotation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Information about when an on-call rotation is in effect and how long the rotation period lasts.
        pub recurrences: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssm::GetContactsRotationRecurrence>,
        >,
        /// The date and time, in RFC 3339 format, that the rotation goes into effect.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The time zone to base the rotation’s activity on in Internet Assigned Numbers Authority (IANA) format.
        pub time_zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetContactsRotationArgs,
    ) -> GetContactsRotationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getContactsRotation:getContactsRotation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "contactIds".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recurrences".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeZoneId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetContactsRotationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            contact_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactIds").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recurrences: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurrences").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZoneId").unwrap(),
            ),
        }
    }
}
