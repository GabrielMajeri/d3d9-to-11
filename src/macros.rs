/// Macro which ensures a certain closure will only be run once.
///
/// Usually used to avoid logging the same warning multiple times.
macro_rules! run_once {
    ($closure: expr) => {{
        use std::sync::Once;
        static LOGGED: Once = Once::new();
        LOGGED.call_once($closure);
    }};
}
