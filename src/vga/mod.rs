pub mod color_code;
pub mod writer;
mod display_char;

#[macro_export]
macro_rules! print {
    ( $($arg:tt)* ) => ($crate::vga::writer::_print(format_args!($ ($arg)* )));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ( $($arg:tt)* ) => ($crate::print!("{}\n", format_args!($ ($arg)* )));
}

#[macro_export]
macro_rules! cprint {
    ( $color: expr, $($arg: tt),* ) => (
        ($crate::vga::writer::_cprint($color, format_args!($ ($arg),* )));
    );
}

#[macro_export]
macro_rules! cprintln {
    () => ($crate::println!());
    ( $color: expr, $($arg: tt),* ) => (
        ($crate::vga::writer::_cprint($color, format_args!($ ($arg),* )));
        $crate::println!();
    );
}

#[macro_export]
macro_rules! color {
    ( $fg: path, $bg: path ) => {
        ($crate::vga::color_code::ColorCode::new($fg, $bg));
    };
    ( $fg: literal, $bg: literal ) => {
        ($crate::vga::color_code::ColorCode::new(
            $crate::vga::color_code::Color::from_name($fg),
            $crate::vga::color_code::Color::from_name($bg),
        ))
    };
}
