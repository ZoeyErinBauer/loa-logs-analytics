#[macro_export]
macro_rules! debug_print {
    ($msg:expr, $e:expr) => {
        #[cfg(debug_assertions)]
        {
            use log::info;
            use std::fmt::Debug;
            fn print_if_debug<T: Debug>(msg: &str, x: T) {
                info!("{}: {:?}", msg, x);
            }
            print_if_debug($msg, $e);
        }
    };
}
