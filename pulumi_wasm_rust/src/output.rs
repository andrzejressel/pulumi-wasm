use anyhow::Error;
use once_cell::sync::Lazy;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface;
use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::stack_interface::add_export;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::sync::Mutex;
use uuid::Uuid;

/// Not yet known value
pub struct Output<T> {
    phantom: PhantomData<T>,
    underlying_id: u32,
}

impl<T> Copy for Output<T> {}

impl<T> Clone for Output<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Default + Serialize> Default for Output<T> {
    fn default() -> Self {
        Output::new(&T::default())
    }
}

impl<T: Serialize> From<T> for Output<T> {
    fn from(value: T) -> Output<T> {
        Output::new(&value)
    }
}

impl<T: Serialize> From<T> for Output<Option<T>> {
    fn from(value: T) -> Self {
        Output::new(&Some(value))
    }
}

impl<T: Serialize + DeserializeOwned> From<Output<T>> for Output<Option<T>> {
    fn from(output: Output<T>) -> Self {
        output.map(|v| Some(v))
    }
}

impl From<&str> for Output<String> {
    fn from(value: &str) -> Self {
        Output::new(&value.to_string())
    }
}

impl From<&str> for Output<Option<String>> {
    fn from(value: &str) -> Self {
        Output::new(&Some(value.to_string()))
    }
}

impl From<Vec<&str>> for Output<Vec<String>> {
    fn from(value: Vec<&str>) -> Self {
        Output::new(&value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl From<Vec<&str>> for Output<Option<Vec<String>>> {
    fn from(value: Vec<&str>) -> Self {
        Output::new(&Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for Output<Vec<T>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        Output::new(&value.into_iter().collect())
    }
}

impl<T: Serialize, const N: usize> From<[T; N]> for Output<Option<Vec<T>>>
where
    T: Serialize,
{
    fn from(value: [T; N]) -> Self {
        Output::new(&Some(value.into_iter().collect()))
    }
}

impl<const N: usize> From<[&str; N]> for Output<Vec<String>> {
    fn from(value: [&str; N]) -> Self {
        Output::new(&value.into_iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for Output<Option<Vec<String>>> {
    fn from(value: [&str; N]) -> Self {
        Output::new(&Some(value.into_iter().map(|s| s.to_string()).collect()))
    }
}

type Function = Box<dyn Fn(&String) -> Result<String, Error> + Send>;

pub(crate) static HASHMAP: Lazy<Mutex<HashMap<String, Function>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

static NONE_OUTPUT: Lazy<Output<Option<String>>> = Lazy::new(|| {
    let op = None::<String>;
    Output::new(&op)
});

impl<T> Output<T> {
    pub fn map<B, F>(&self, f: F) -> Output<B>
    where
        F: Fn(T) -> B + Send + 'static,
        T: DeserializeOwned,
        B: Serialize,
    {
        let f = move |arg: &String| {
            let argument = serde_json::from_str(arg)?;
            let result = f(argument);
            let result = serde_json::to_string(&result)?;
            Ok(result)
        };

        let uuid = Uuid::now_v7().to_string();
        let mut map = HASHMAP.lock().unwrap();
        map.insert(uuid.clone(), Box::new(f));

        let wit = self.get_inner();
        let new_output = wit.map(uuid.as_str());

        Output {
            phantom: PhantomData,
            underlying_id: output_interface::Output::take_handle(&new_output),
        }
    }

    pub(crate) fn add_to_export(&self, name: &str) {
        add_export(name, &self.get_inner());
    }

    /// Forcefully changes apparent type of underlying Output
    ///
    /// Can be used to workaround Pulumi provider incorrect types
    ///
    /// # Safety
    ///
    /// Underlying output must be of type `F`.
    pub unsafe fn transmute<F: Serialize>(&self) -> Output<F> {
        Output {
            phantom: PhantomData::<F>,
            underlying_id: self.underlying_id,
        }
    }

    #[doc(hidden)]
    ///
    /// # Safety
    ///
    /// Underlying output must be of type `F`.
    pub unsafe fn new_from_handle<F: Serialize>(handle: output_interface::Output) -> Output<F> {
        Output {
            phantom: PhantomData::<F>,
            underlying_id: output_interface::Output::take_handle(&handle),
        }
    }

    #[doc(hidden)]
    pub fn get_inner(&self) -> ManuallyDrop<output_interface::Output> {
        unsafe { ManuallyDrop::new(output_interface::Output::from_handle(self.underlying_id)) }
    }
}

impl<T: Serialize> Output<T> {
    pub fn new(value: &T) -> Self {
        let binding = serde_json::to_string(&value).unwrap();
        let resource = output_interface::Output::new(binding.as_str());
        Output {
            phantom: PhantomData,
            underlying_id: output_interface::Output::take_handle(&resource),
        }
    }

    /// Returns singleton Output containing serialized null
    pub fn empty() -> Output<Option<T>> {
        unsafe { NONE_OUTPUT.transmute::<Option<T>>() }
    }
}

/// Generates Output<String> with formatted string. Supports up to 16 arguments.
///
/// Format string passed as first arguments is the same as in `format!` macro.
/// ```no_run
/// use pulumi_wasm_rust::{pulumi_format, Output, ToOutput};
///
/// let a = Output::new(&1);
/// let b = Output::new(&"test".to_string());
/// let formatted: Output<String> = pulumi_format!("{} {}", a, b); // "1 test"
#[macro_export]
macro_rules! pulumi_format {
    ($format:expr, $o1:expr) => {{
        $o1.create_output().map(|a| format!($format, a))
    }};
    ($format:expr, $o1:expr, $o2:expr) => {{
        pulumi_wasm_rust::__private::output::combine2($o1.create_output(), $o2.create_output())
            .map(|(a, b)| format!($format, a, b))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr) => {{
        pulumi_wasm_rust::__private::output::combine3(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
        )
        .map(|(a, b, c)| format!($format, a, b, c))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr) => {{
        pulumi_wasm_rust::__private::output::combine4(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
        )
        .map(|(a, b, c, d)| format!($format, a, b, c, d))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr) => {{
        pulumi_wasm_rust::__private::output::combine5(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
        )
        .map(|(a, b, c, d, e)| format!($format, a, b, c, d, e))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr) => {{
        pulumi_wasm_rust::__private::output::combine6(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
        )
        .map(|(a, b, c, d, e, f)| format!($format, a, b, c, d, e, f))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr) => {{
        pulumi_wasm_rust::__private::output::combine7(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
        )
        .map(|(a, b, c, d, e, f, g)| format!($format, a, b, c, d, e, f, g))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr) => {{
        pulumi_wasm_rust::__private::output::combine8(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h)| format!($format, a, b, c, d, e, f, g, h))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr) => {{
        pulumi_wasm_rust::__private::output::combine9(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i)| format!($format, a, b, c, d, e, f, g, h, i))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr) => {{
        pulumi_wasm_rust::__private::output::combine10(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j)| format!($format, a, b, c, d, e, f, g, h, i, j))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr) => {{
        pulumi_wasm_rust::__private::output::combine11(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k)| format!($format, a, b, c, d, e, f, g, h, i, j, k))
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr, $o12:expr) => {{
        pulumi_wasm_rust::__private::output::combine12(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
            $o12.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l)
        })
    }};
    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr, $o12:expr, $o13:expr) => {{
        pulumi_wasm_rust::__private::output::combine13(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
            $o12.create_output(),
            $o13.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m)
        })
    }};

    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr, $o12:expr, $o13:expr, $o14:expr) => {{
        pulumi_wasm_rust::__private::output::combine14(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
            $o12.create_output(),
            $o13.create_output(),
            $o14.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        })
    }};

    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr, $o12:expr, $o13:expr, $o14:expr, $o15:expr) => {{
        pulumi_wasm_rust::__private::output::combine15(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
            $o12.create_output(),
            $o13.create_output(),
            $o14.create_output(),
            $o15.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        })
    }};

    ($format:expr, $o1:expr, $o2:expr, $o3:expr, $o4:expr, $o5:expr, $o6:expr, $o7:expr, $o8:expr, $o9:expr, $o10:expr, $o11:expr, $o12:expr, $o13:expr, $o14:expr, $o15:expr, $o16:expr) => {{
        pulumi_wasm_rust::__private::output::combine16(
            $o1.create_output(),
            $o2.create_output(),
            $o3.create_output(),
            $o4.create_output(),
            $o5.create_output(),
            $o6.create_output(),
            $o7.create_output(),
            $o8.create_output(),
            $o9.create_output(),
            $o10.create_output(),
            $o11.create_output(),
            $o12.create_output(),
            $o13.create_output(),
            $o14.create_output(),
            $o15.create_output(),
            $o16.create_output(),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        })
    }};
    ($($arg:expr),+) => {
        compile_error!("pulumi_format! macro supports up to 16 arguments")
    };
}

/// Combine multiple Outputs into a single Output of [tuple] type. Supports up to 16 arguments.
/// ```no_run
/// use pulumi_wasm_rust::{pulumi_combine, Output};
///
/// let a = Output::new(&1);
/// let b = Output::new(&"test");
/// let combined: Output<(i32, &str)> = pulumi_combine!(a, b);
/// ```
#[macro_export]
macro_rules! pulumi_combine {
    ($arg1:expr, $arg2:expr) => {
        pulumi_wasm_rust::__private::output::combine2($arg1, $arg2)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr) => {
        pulumi_wasm_rust::__private::output::combine3($arg1, $arg2, $arg3)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        pulumi_wasm_rust::__private::output::combine4($arg1, $arg2, $arg3, $arg4)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        pulumi_wasm_rust::__private::output::combine5($arg1, $arg2, $arg3, $arg4, $arg5)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        pulumi_wasm_rust::__private::output::combine6($arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr) => {
        pulumi_wasm_rust::__private::output::combine7(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr) => {
        pulumi_wasm_rust::__private::output::combine8(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr) => {
        pulumi_wasm_rust::__private::output::combine9(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr) => {
        pulumi_wasm_rust::__private::output::combine10(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr) => {
        pulumi_wasm_rust::__private::output::combine11(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr) => {
        pulumi_wasm_rust::__private::output::combine12(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr, $arg13:expr) => {
        pulumi_wasm_rust::__private::output::combine13(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr, $arg13:expr, $arg14:expr) => {
        pulumi_wasm_rust::__private::output::combine14(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr, $arg13:expr, $arg14:expr, $arg15:expr) => {
        pulumi_wasm_rust::__private::output::combine15(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14, $arg15,
        )
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr, $arg13:expr, $arg14:expr, $arg15:expr, $arg16:expr) => {
        pulumi_wasm_rust::__private::output::combine16(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14, $arg15, $arg16,
        )
    };
    ($($arg:expr),+) => {
        compile_error!("pulumi_combine! macro supports up to 16 arguments")
    };
}

/// Helper trait utilized in [pulumi_format!](`crate::pulumi_format!`) macro
pub trait ToOutput<T> {
    fn create_output(&self) -> Output<T>;
}

impl<T: Serialize> ToOutput<T> for T {
    fn create_output(&self) -> Output<T> {
        Output::new(self)
    }
}

impl<T> ToOutput<T> for Output<T> {
    fn create_output(&self) -> Output<T> {
        *self
    }
}
