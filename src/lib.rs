#[macro_export]
macro_rules! make_option {
    (Option<$ty:ty>) => {
        Option<$ty>
    };
    ($ty:ty) => {
        Option<$ty>
    }
}

#[macro_export]
macro_rules! value_as_option {
    (Option<$ty:ty>, $expr:expr) => {
        $expr
    };
    ($ty:ty, $expr:expr) => {
        Some($expr)
    };
}

#[macro_export]
macro_rules! value_maybe_as_option {
    (Option<$ty:ty>, $_:expr, $expr:expr) => {
        $expr
    };
    ($ty:ty, $expr:expr, $_:expr) => {
        $expr
    };
}
