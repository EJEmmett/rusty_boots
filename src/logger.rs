
#[macro_export]
macro_rules! log {
    ( $($arg:tt)* ) => {
        $crate::vga::writer::_cprint($crate::vga::color_code::ColorCode::GREEN, format_args!("[{}ns]", $crate::time::rdtsc::delta_ns()));
        $crate::println!("{}\n", format_args!($ ($arg) *));
    };
}