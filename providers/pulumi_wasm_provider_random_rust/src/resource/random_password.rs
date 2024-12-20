//! Identical to random_string.
//! 
//! This resource *does* use a cryptographic random number generator.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   password:
//!     type: random:RandomPassword
//!     properties:
//!       length: 16
//!       special: true
//!       overrideSpecial: '!#$%&*()-_=+[]{}<>:?'
//!   example:
//!     type: aws:rds:Instance
//!     properties:
//!       instanceClass: db.t3.micro
//!       allocatedStorage: 64
//!       engine: mysql
//!       username: someone
//!       password: ${password.result}
//! ```
//! 
//! ## Import
//! 
//! You can import external passwords into your Pulumi programs as follows:
//! 
//! ```sh
//!  $ import random:index/randomPassword:RandomPassword newPassword supersecret
//! ```
//! 
//! This command will encode the `supersecret` token in Pulumi state and generate a code suggestion to include a new RandomPassword resource in your Pulumi program. Include the suggested code and do a `pulumi up`. Your secret password is now securely stored in Pulumi, and you can reference it in your Pulumi program as `newPassword.result`. 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomPasswordArgs {
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
    #[builder(into)]
    pub length: pulumi_wasm_rust::Output<i32>,
    /// Include lowercase alphabet characters in the result. Default value is `true`.
    #[builder(into, default)]
    pub lower: pulumi_wasm_rust::Output<Option<bool>>,
    /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
    #[builder(into, default)]
    pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
    /// Minimum number of numeric characters in the result. Default value is `0`.
    #[builder(into, default)]
    pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
    /// Minimum number of special characters in the result. Default value is `0`.
    #[builder(into, default)]
    pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
    /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
    #[builder(into, default)]
    pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
    /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
    #[builder(into, default)]
    pub number: pulumi_wasm_rust::Output<Option<bool>>,
    /// Include numeric characters in the result. Default value is `true`.
    #[builder(into, default)]
    pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
    /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
    #[builder(into, default)]
    pub override_special: pulumi_wasm_rust::Output<Option<String>>,
    /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
    #[builder(into, default)]
    pub special: pulumi_wasm_rust::Output<Option<bool>>,
    /// Include uppercase alphabet characters in the result. Default value is `true`.
    #[builder(into, default)]
    pub upper: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct RandomPasswordResult {
    /// A bcrypt hash of the generated random string. **NOTE**: If the generated random string is greater than 72 bytes in length, `bcrypt_hash` will contain a hash of the first 72 bytes.
    pub bcrypt_hash: pulumi_wasm_rust::Output<String>,
    /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
    pub keepers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
    pub length: pulumi_wasm_rust::Output<i32>,
    /// Include lowercase alphabet characters in the result. Default value is `true`.
    pub lower: pulumi_wasm_rust::Output<bool>,
    /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
    pub min_lower: pulumi_wasm_rust::Output<i32>,
    /// Minimum number of numeric characters in the result. Default value is `0`.
    pub min_numeric: pulumi_wasm_rust::Output<i32>,
    /// Minimum number of special characters in the result. Default value is `0`.
    pub min_special: pulumi_wasm_rust::Output<i32>,
    /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
    pub min_upper: pulumi_wasm_rust::Output<i32>,
    /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
    pub number: pulumi_wasm_rust::Output<bool>,
    /// Include numeric characters in the result. Default value is `true`.
    pub numeric: pulumi_wasm_rust::Output<bool>,
    /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
    pub override_special: pulumi_wasm_rust::Output<Option<String>>,
    /// The generated random string.
    pub result: pulumi_wasm_rust::Output<String>,
    /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
    pub special: pulumi_wasm_rust::Output<bool>,
    /// Include uppercase alphabet characters in the result. Default value is `true`.
    pub upper: pulumi_wasm_rust::Output<bool>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomPasswordArgs) -> RandomPasswordResult {

    let result = crate::bindings::pulumi::random::random_password::invoke(name, &crate::bindings::pulumi::random::random_password::Args {
        keepers: &args.keepers.get_inner(),
        length: &args.length.get_inner(),
        lower: &args.lower.get_inner(),
        min_lower: &args.min_lower.get_inner(),
        min_numeric: &args.min_numeric.get_inner(),
        min_special: &args.min_special.get_inner(),
        min_upper: &args.min_upper.get_inner(),
        number: &args.number.get_inner(),
        numeric: &args.numeric.get_inner(),
        override_special: &args.override_special.get_inner(),
        special: &args.special.get_inner(),
        upper: &args.upper.get_inner(),
    });

    RandomPasswordResult {
        bcrypt_hash: crate::into_domain(result.bcrypt_hash),
        keepers: crate::into_domain(result.keepers),
        length: crate::into_domain(result.length),
        lower: crate::into_domain(result.lower),
        min_lower: crate::into_domain(result.min_lower),
        min_numeric: crate::into_domain(result.min_numeric),
        min_special: crate::into_domain(result.min_special),
        min_upper: crate::into_domain(result.min_upper),
        number: crate::into_domain(result.number),
        numeric: crate::into_domain(result.numeric),
        override_special: crate::into_domain(result.override_special),
        result: crate::into_domain(result.result),
        special: crate::into_domain(result.special),
        upper: crate::into_domain(result.upper),
    }
}
