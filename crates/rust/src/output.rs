use crate::{Context, GestaltContext, Output};
use serde::Serialize;

/// Generates Output<String> with formatted string. Supports up to 16 arguments.
///
/// Format string passed as first arguments is the same as in `format!` macro.
/// ```no_run
/// use anyhow::Result;
/// use pulumi_gestalt_rust::*;
///
/// fn pulumi_main(context: &Context) -> Result<()> {
///   let a = context.new_output(&1);
///   let b = context.new_output(&"test".to_string());
///   let formatted: Output<String> = pulumi_format!(context, "{} {}", a, b); // "1 test"
///   Ok(())
/// }
#[macro_export]
macro_rules! pulumi_format {
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021) => {{
        $o1.create_output(&$context).map(|a| format!($format, a))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine2(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
        )
        .map(|(a, b)| format!($format, a, b))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine3(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
        )
        .map(|(a, b, c)| format!($format, a, b, c))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine4(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
        )
        .map(|(a, b, c, d)| format!($format, a, b, c, d))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine5(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
        )
        .map(|(a, b, c, d, e)| format!($format, a, b, c, d, e))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine6(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f)| format!($format, a, b, c, d, e, f))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine7(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g)| format!($format, a, b, c, d, e, f, g))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine8(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h)| format!($format, a, b, c, d, e, f, g, h))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine9(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i)| format!($format, a, b, c, d, e, f, g, h, i))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine10(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j)| format!($format, a, b, c, d, e, f, g, h, i, j))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine11(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k)| format!($format, a, b, c, d, e, f, g, h, i, j, k))
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021, $o12:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine12(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
            $o12.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l)
        })
    }};
    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021, $o12:expr_2021, $o13:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine13(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
            $o12.create_output(&$context),
            $o13.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m)
        })
    }};

    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021, $o12:expr_2021, $o13:expr_2021, $o14:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine14(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
            $o12.create_output(&$context),
            $o13.create_output(&$context),
            $o14.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        })
    }};

    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021, $o12:expr_2021, $o13:expr_2021, $o14:expr_2021, $o15:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine15(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
            $o12.create_output(&$context),
            $o13.create_output(&$context),
            $o14.create_output(&$context),
            $o15.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        })
    }};

    ($context:expr_2021, $format:expr_2021, $o1:expr_2021, $o2:expr_2021, $o3:expr_2021, $o4:expr_2021, $o5:expr_2021, $o6:expr_2021, $o7:expr_2021, $o8:expr_2021, $o9:expr_2021, $o10:expr_2021, $o11:expr_2021, $o12:expr_2021, $o13:expr_2021, $o14:expr_2021, $o15:expr_2021, $o16:expr_2021) => {{
        pulumi_gestalt_rust::__private::output::combine16(
            $o1.create_output(&$context),
            $o2.create_output(&$context),
            $o3.create_output(&$context),
            $o4.create_output(&$context),
            $o5.create_output(&$context),
            $o6.create_output(&$context),
            $o7.create_output(&$context),
            $o8.create_output(&$context),
            $o9.create_output(&$context),
            $o10.create_output(&$context),
            $o11.create_output(&$context),
            $o12.create_output(&$context),
            $o13.create_output(&$context),
            $o14.create_output(&$context),
            $o15.create_output(&$context),
            $o16.create_output(&$context),
        )
        .map(|(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)| {
            format!($format, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        })
    }};
    ($($arg:expr_2021),+) => {
        compile_error!("pulumi_format! macro supports up to 16 arguments")
    };
}

/// Combine multiple Outputs into a single Output of [tuple] type. Supports up to 16 arguments.
/// ```no_run
/// use anyhow::Result;
/// use pulumi_gestalt_rust::*;
///
/// fn pulumi_main(context: &Context) -> Result<()> {
///   let a = context.new_output(&1);
///   let b = context.new_output(&"test".to_string());
///   let combined: Output<(i32, String)> = pulumi_combine!(a, b);
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pulumi_combine {
    ($arg1:expr_2021, $arg2:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine2($arg1, $arg2)
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine3($arg1, $arg2, $arg3)
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine4($arg1, $arg2, $arg3, $arg4)
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine5($arg1, $arg2, $arg3, $arg4, $arg5)
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine6($arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine7(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine8(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine9(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine10(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine11(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021, $arg12:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine12(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021, $arg12:expr_2021, $arg13:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine13(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021, $arg12:expr_2021, $arg13:expr_2021, $arg14:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine14(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021, $arg12:expr_2021, $arg13:expr_2021, $arg14:expr_2021, $arg15:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine15(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14, $arg15,
        )
    };
    ($arg1:expr_2021, $arg2:expr_2021, $arg3:expr_2021, $arg4:expr_2021, $arg5:expr_2021, $arg6:expr_2021, $arg7:expr_2021, $arg8:expr_2021, $arg9:expr_2021, $arg10:expr_2021, $arg11:expr_2021, $arg12:expr_2021, $arg13:expr_2021, $arg14:expr_2021, $arg15:expr_2021, $arg16:expr_2021) => {
        pulumi_gestalt_rust::__private::output::combine16(
            $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12,
            $arg13, $arg14, $arg15, $arg16,
        )
    };
    ($($arg:expr_2021),+) => {
        compile_error!("pulumi_combine! macro supports up to 16 arguments")
    };
}

/// Helper trait utilized in [pulumi_format!](`crate::pulumi_format!`) macro
pub trait ToOutput<T> {
    fn create_output(&self, engine: &Context) -> Output<T>;
}

impl<T: Serialize> ToOutput<T> for T {
    fn create_output(&self, engine: &Context) -> Output<T> {
        engine.new_output(self)
    }
}

impl<T> ToOutput<T> for Output<T> {
    fn create_output(&self, _: &Context) -> Output<T> {
        self.clone()
    }
}
