/// Manages a Data Share.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:datashare:Account
///     name: example
///     properties:
///       name: example-dsa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///       tags:
///         foo: bar
///   exampleShare:
///     type: azure:datashare:Share
///     name: example
///     properties:
///       name: example_dss
///       accountId: ${exampleAccount.id}
///       kind: CopyBased
///       description: example desc
///       terms: example terms
///       snapshotSchedule:
///         name: example-ss
///         recurrence: Day
///         startTime: 2020-04-17T04:47:52.9614956Z
/// ```
///
/// ## Import
///
/// Data Shares can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/share:Share example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1
/// ```
///
pub mod share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareArgs {
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Data Share's description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `snapshot_schedule` block as defined below.
        #[builder(into, default)]
        pub snapshot_schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        #[builder(into, default)]
        pub terms: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ShareResult {
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The Data Share's description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `snapshot_schedule` block as defined below.
        pub snapshot_schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        pub terms: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ShareArgs,
    ) -> ShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kind_binding = args.kind.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let snapshot_schedule_binding = args
            .snapshot_schedule
            .get_output(context)
            .get_inner();
        let terms_binding = args.terms.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datashare/share:Share".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotSchedule".into(),
                    value: &snapshot_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "terms".into(),
                    value: &terms_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ShareResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            snapshot_schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotSchedule"),
            ),
            terms: pulumi_gestalt_rust::__private::into_domain(o.extract_field("terms")),
        }
    }
}
