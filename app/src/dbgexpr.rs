#[allow(unused_macros)]
macro_rules! debug_expr {
    ( $x:expr ) => {{
        let v = $x;
        log::debug!("{} = {:?}", stringify!($x), &v);
        v
    }};
}

#[allow(unused_imports)]
pub(crate) use debug_expr;
