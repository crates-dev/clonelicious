/// The `clone!` macro automatically clones the variables passed to it and immediately executes the closure.
///
/// This macro simplifies the process of cloning variables and passing them into a closure, while also
/// ensuring the closure is executed right away with the cloned values.
///
/// # Parameters
/// - `$( $var:ident ),*`: A list of variables to be cloned. These variables will be cloned before
///   being passed to the closure. Each variable is cloned individually.
/// - `move |$($arg:ident : $typ:ty),*| $body:block`: A closure that takes the cloned variables as arguments.
///   The closure's parameters should be annotated with types. The closure body will be executed with the
///   cloned values.
///
/// # Behavior
/// - The macro first clones the provided variables (`s1`, `s2`, etc.).
/// - It then constructs and immediately calls a `move` closure, passing the cloned values as arguments.
/// - The closure is executed right away with the cloned data, ensuring no ownership or borrowing issues.
///
/// # Notes
/// This macro is useful for cases where you need to clone values and pass them to a closure,
/// but don't want to manually clone each value before passing it into the closure.
#[macro_export]
macro_rules! clone {
    ( $( $var:ident ),* => { $($body:tt)* } ) => {{
        $( let $var = $var.clone(); )*
        { $($body)* }
    }};

    ( $( $var:ident ),* => async move { $($body:tt)* } ) => {{
        $( let $var = $var.clone(); )*
        async move { $($body)* }
    }};
}
