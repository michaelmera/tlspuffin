//! This module provides a DLS for writing[`Term`]swithin Rust.
//!
//! # Example
//!
//! ```rust
//! use tlspuffin::tls::fn_impl::fn_client_hello;
//! use tlspuffin::term;
//! use rustls::{ProtocolVersion, CipherSuite};
//! use rustls::msgs::handshake::{SessionID, Random, ClientExtension};
//! use rustls::msgs::enums::Compression;
//!
//! let term = term! {
//!     fn_client_hello(
//!         ((0, 0)/ProtocolVersion),
//!         ((0, 0)/Random),
//!         ((0, 0)/SessionID),
//!         ((0, 0)/Vec<CipherSuite>),
//!         ((0, 0)/Vec<Compression>),
//!         ((0, 0)/Vec<ClientExtension>)
//!     )
//! };
//! ```

#[macro_export]
macro_rules! app_const {
    ($op:ident) => {
        Term::Application(Signature::new_function(&$op), vec![])
    };
}

#[macro_export]
macro_rules! app {
    ($op:ident, $($args:expr),*$(,)?) => {
        Term::Application(Signature::new_function(&$op),vec![$($args,)*])
    };
}

#[macro_export]
macro_rules! var {
    ($typ:ty, $id:expr) => {
        Term::Variable(Signature::new_var::<$typ>($id))
    };
}

// todo we could improve performance by not recreating these
#[macro_export]
macro_rules! term {
    // Variables
    (($step:expr, $msg:expr) / $typ:ty) => {{
        use std::rc::Rc;
        use std::cell::RefCell;
        use $crate::term::Term;
        use $crate::term::signature::Signature;

        let var = Signature::new_var::<$typ>( ($step, $msg));
        Rc::new(RefCell::new(Term::Variable(var)))
    }};

    // Constants
    ($func:ident) => {{
        use std::rc::Rc;
        use std::cell::RefCell;
        use $crate::term::Term;
        use $crate::term::signature::Signature;

        let func = Signature::new_function(&$func);
        Rc::new(RefCell::new(Term::Application(func, vec![])))
    }};

    // Function Applications
    ($func:ident ($($args:tt),*)) => {{
        use std::rc::Rc;
        use std::cell::RefCell;
        use $crate::term::Term;
        use $crate::term::signature::Signature;

        let func = Signature::new_function(&$func);
        Rc::new(RefCell::new(Term::Application(func, vec![$($crate::term_arg!($args)),*])))
    }};

    (@$e:expr) => {{
        $e
    }};
}

#[macro_export]
macro_rules! term_arg {
    // Somehow the following rules is very important
    ( ( $($e:tt)* ) ) => (term!($($e)*));
    // not sure why I should need this
    // ( ( $e:tt ) ) => (ast!($e));
    ($e:tt) => (term!($e));
}
