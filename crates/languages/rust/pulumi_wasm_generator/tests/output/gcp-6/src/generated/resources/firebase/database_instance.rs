/// ## Example Usage
///
/// ### Firebase Database Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = database_instance::create(
///         "basic",
///         DatabaseInstanceArgs::builder()
///             .instance_id("active-db")
///             .project("my-project-name")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase Database Instance Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let full = database_instance::create(
///         "full",
///         DatabaseInstanceArgs::builder()
///             .desired_state("DISABLED")
///             .instance_id("disabled-db")
///             .project("my-project-name")
///             .region("europe-west1")
///             .type_("USER_DATABASE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firebase Database Instance Default Database
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:organizations:Project
///     properties:
///       projectId: rtdb-project
///       name: rtdb-project
///       orgId: '123456789'
///       deletionPolicy: DELETE
///       labels:
///         firebase: enabled
///   firebase:
///     type: gcp:projects:Service
///     properties:
///       project: ${default.projectId}
///       service: firebase.googleapis.com
///       disableOnDestroy: false
///   defaultProject:
///     type: gcp:firebase:Project
///     name: default
///     properties:
///       project: ${default.projectId}
///     options:
///       dependsOn:
///         - ${firebase}
///   firebaseDatabase:
///     type: gcp:projects:Service
///     name: firebase_database
///     properties:
///       project: ${defaultProject.project}
///       service: firebasedatabase.googleapis.com
///       disableOnDestroy: false
///   wait60Seconds:
///     type: time:sleep
///     name: wait_60_seconds
///     properties:
///       createDuration: 60s
///     options:
///       dependsOn:
///         - ${firebaseDatabase}
///   defaultDatabaseInstance:
///     type: gcp:firebase:DatabaseInstance
///     name: default
///     properties:
///       project: ${defaultProject.project}
///       region: us-central1
///       instanceId: rtdb-project-default-rtdb
///       type: DEFAULT_DATABASE
///     options:
///       dependsOn:
///         - ${wait60Seconds}
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{region}}/{{instance_id}}`
///
/// * `{{region}}/{{instance_id}}`
///
/// * `{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebase/databaseInstance:DatabaseInstance default projects/{{project}}/locations/{{region}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/databaseInstance:DatabaseInstance default {{project}}/{{region}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/databaseInstance:DatabaseInstance default {{region}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firebase/databaseInstance:DatabaseInstance default {{instance_id}}
/// ```
///
pub mod database_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseInstanceArgs {
        /// The intended database state. Possible values: ACTIVE, DISABLED.
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The globally unique identifier of the Firebase Realtime Database instance.
        /// Instance IDs cannot be reused after deletion.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the region where the Firebase Realtime database resides.
        /// Check all [available regions](https://firebase.google.com/docs/projects/locations#rtdb-locations)
        #[builder(into)]
        pub region: pulumi_wasm_rust::InputOrOutput<String>,
        /// The database type.
        /// Each project can create one default Firebase Realtime Database, which cannot be deleted once created.
        /// Creating user Databases is only available for projects on the Blaze plan.
        /// Projects can be upgraded using the Cloud Billing API https://cloud.google.com/billing/reference/rest/v1/projects/updateBillingInfo.
        /// Default value is `USER_DATABASE`.
        /// Possible values are: `DEFAULT_DATABASE`, `USER_DATABASE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseInstanceResult {
        /// The database URL in the form of https://{instance-id}.firebaseio.com for us-central1 instances
        /// or https://{instance-id}.{region}.firebasedatabase.app in other regions.
        pub database_url: pulumi_wasm_rust::Output<String>,
        /// The intended database state. Possible values: ACTIVE, DISABLED.
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// The globally unique identifier of the Firebase Realtime Database instance.
        /// Instance IDs cannot be reused after deletion.
        ///
        ///
        /// - - -
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The fully-qualified resource name of the Firebase Realtime Database, in
        /// the format: projects/PROJECT_NUMBER/locations/REGION_IDENTIFIER/instances/INSTANCE_ID
        /// PROJECT_NUMBER: The Firebase project's [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number)
        /// Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510).
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// A reference to the region where the Firebase Realtime database resides.
        /// Check all [available regions](https://firebase.google.com/docs/projects/locations#rtdb-locations)
        pub region: pulumi_wasm_rust::Output<String>,
        /// The current database state. Set desired_state to :DISABLED to disable the database and :ACTIVE to reenable the database
        pub state: pulumi_wasm_rust::Output<String>,
        /// The database type.
        /// Each project can create one default Firebase Realtime Database, which cannot be deleted once created.
        /// Creating user Databases is only available for projects on the Blaze plan.
        /// Projects can be upgraded using the Cloud Billing API https://cloud.google.com/billing/reference/rest/v1/projects/updateBillingInfo.
        /// Default value is `USER_DATABASE`.
        /// Possible values are: `DEFAULT_DATABASE`, `USER_DATABASE`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatabaseInstanceArgs,
    ) -> DatabaseInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let desired_state_binding = args.desired_state.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/databaseInstance:DatabaseInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseInstanceResult {
            database_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("databaseUrl"),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredState"),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
