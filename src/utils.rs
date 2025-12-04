#[macro_export]
macro_rules! pause {
    () => {
        use std::io::{self, Read};
        let _ = io::stdin().read(&mut [0u8]);
    };
    ($($args:tt)*) => {
        use std::io::{self, Read};
        println!($($args)*);
        let _ = io::stdin().read(&mut [0u8]);
    };
}
