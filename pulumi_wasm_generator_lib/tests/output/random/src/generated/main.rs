/// The resource `random.RandomId` generates random numbers that are intended to be
/// used as unique identifiers for other resources.
///
/// This resource *does* use a cryptographic random number generator in order
/// to minimize the chance of collisions, making the results of this resource
/// when a 16-byte identifier is requested of equivalent uniqueness to a
/// type-4 UUID.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a unique name for an AWS EC2
/// instance that changes each time a new AMI id is selected.
///
///
/// ## Import
///
/// Random Ids can be imported using the `b64_url` with an optional `prefix`. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example with no prefix
///
/// ```sh
///  $ pulumi import random:index/randomId:RandomId server p-9hUg
/// ```
///
///  Example with prefix (prefix is separated by a `,`)
///
/// ```sh
///  $ pulumi import random:index/randomId:RandomId server my-prefix-,p-9hUg
/// ```
///
///
pub mod random_id {
    include!("resources/random_id.rs");
}
/// The resource `random.RandomInteger` generates random values from a given range, described by the `min` and `max` attributes of a given resource.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set, to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a random priority between 1 and 50000 for
/// a `aws_alb_listener_rule` resource:
///
///
/// The result of the above will set a random priority.
///
/// ## Import
///
/// Random integers can be imported using the `result`, `min`, and `max`, with an optional `seed`. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example (values are separated by a `,`)
///
/// ```sh
///  $ pulumi import random:index/randomInteger:RandomInteger priority 15390,1,50000
/// ```
///
///
pub mod random_integer {
    include!("resources/random_integer.rs");
}
/// Identical to the `random.RandomString` resource with the exception that the
/// result is treated as sensitive and, thus, _not_ displayed in console output.
///
/// > **Note:** All attributes including the generated password will be stored in
/// the raw state as plain-text.
///
/// This resource *does* use a cryptographic random number generator.
///
///
/// ## Example Usage
///
///
/// ## Import
///
/// Random Password can be imported by specifying the value of the string
///
/// ```sh
///  $ pulumi import random:index/randomPassword:RandomPassword password securepassword
/// ```
///
///
pub mod random_password {
    include!("resources/random_password.rs");
}
/// The resource `random.RandomPet` generates random pet names that are intended to be
/// used as unique identifiers for other resources.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set, to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a unique pet name for an AWS EC2
/// instance that changes each time a new AMI id is selected.
///
///
/// The result of the above will set the Name of the AWS Instance to
/// `web-server-simple-snake`.
pub mod random_pet {
    include!("resources/random_pet.rs");
}
/// The resource `random.RandomShuffle` generates a random permutation of a list
/// of strings given as an argument.
///
///
/// ## Example Usage
///
pub mod random_shuffle {
    include!("resources/random_shuffle.rs");
}
/// The resource `random.RandomString` generates a random permutation of alphanumeric
/// characters and optionally special characters.
///
/// This resource *does* use a cryptographic random number generator.
///
/// Historically this resource's intended usage has been ambiguous as the original example
/// used it in a password. For backwards compatibility it will
/// continue to exist. For unique ids please use the `random.RandomId` resource, for sensitive
/// random values please use the `random.RandomPassword` resource.
///
///
/// ## Example Usage
///
///
/// ## Import
///
/// Strings can be imported by just specifying the value of the string
///
/// ```sh
///  $ pulumi import random:index/randomString:RandomString test test
/// ```
///
///
pub mod random_string {
    include!("resources/random_string.rs");
}
/// The resource `random.RandomUuid` generates random uuid string that is intended to be
/// used as unique identifiers for other resources.
///
/// This resource uses the `hashicorp/go-uuid` to generate a UUID-formatted string
/// for use with services needed a unique string identifier.
///
///
///
/// ## Example Usage
///
/// The following example shows how to generate a unique name for an Azure Resource Group.
///
///
/// ## Import
///
/// Random UUID's can be imported. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example
///
/// ```sh
///  $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
/// ```
///
///
pub mod random_uuid {
    include!("resources/random_uuid.rs");
}
pub mod functions {}
pub mod types {}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-random {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface } }
    );
}
