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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseInstanceArgs {
        /// The intended database state. Possible values: ACTIVE, DISABLED.
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// The globally unique identifier of the Firebase Realtime Database instance.
        /// Instance IDs cannot be reused after deletion.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to the region where the Firebase Realtime database resides.
        /// Check all [available regions](https://firebase.google.com/docs/projects/locations#rtdb-locations)
        #[builder(into)]
        pub region: pulumi_wasm_rust::Output<String>,
        /// The database type.
        /// Each project can create one default Firebase Realtime Database, which cannot be deleted once created.
        /// Creating user Databases is only available for projects on the Blaze plan.
        /// Projects can be upgraded using the Cloud Billing API https://cloud.google.com/billing/reference/rest/v1/projects/updateBillingInfo.
        /// Default value is `USER_DATABASE`.
        /// Possible values are: `DEFAULT_DATABASE`, `USER_DATABASE`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn create(name: &str, args: DatabaseInstanceArgs) -> DatabaseInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let desired_state_binding = args.desired_state.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firebase/databaseInstance:DatabaseInstance".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "databaseUrl".into(),
                },
                register_interface::ResultField {
                    name: "desiredState".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseInstanceResult {
            database_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseUrl").unwrap(),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredState").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
