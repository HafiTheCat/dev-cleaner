#[macro_export]
macro_rules! display_field {
    ($f:expr, $name:expr, $value:expr) => {
        writeln!($f, "{}:", $name)?;
        writeln!($f, "{}\n", $value)?;
    };
}