//! This module provides a DLS for writing[`Term`]swithin Rust.
//!
//! # Example
//!
//! ```rust
//! use tlspuffin::tls::fn_impl::fn_client_hello;
//! use tlspuffin::term;
//! use tlspuffin::agent::AgentName;
//! use rustls::{ProtocolVersion, CipherSuite};
//! use rustls::msgs::handshake::{SessionID, Random, ClientExtension};
//! use rustls::msgs::enums::Compression;
//!
//! let client = AgentName::first();
//! let term = term! {
//!     fn_client_hello(
//!         ((client, 0)/ProtocolVersion),
//!         ((client, 0)/Random),
//!         ((client, 0)/SessionID),
//!         ((client, 0)/Vec<CipherSuite>),
//!         ((client, 0)/Vec<Compression>),
//!         ((client, 0)/Vec<ClientExtension>)
//!     )
//! };
//! ```

#[macro_export]
macro_rules! term {
    //
    // Handshake TlsMessageType with `None` as `HandshakeType`
    // `>$req_type:expr` must be the last part of the arm, even if it is not used.
    //
    (($agent:expr, $counter:expr) $([H])? / $typ:ty $(>$req_type:expr)?) => {{
        use crate::term::dynamic_function::TypeShape;
        term!(($agent, $counter) [H] > TypeShape::of::<$typ>())
    }};
    (($agent:expr, $counter:expr) $([H])? $(>$req_type:expr)?) => {{
        use $crate::trace::TlsMessageType;
        use $crate::term::signature::Signature;

        let var = Signature::new_var_by_type_id($($req_type)?, $agent, Some(TlsMessageType::Handshake(None)), $counter);
        $crate::term::Term::Variable(var)
    }};

    //
    // Handshake TlsMessageType with `Some(x)` as `HandshakeType`, where `x` is `TypeShape::of::<$typ>()`
    //
    (($agent:expr, $counter:expr) [H::$hs_type:expr] / $typ:ty $(>$req_type:expr)?) => {{
        use crate::term::dynamic_function::TypeShape;

        term!(($agent, $counter) [H::$hs_type] > TypeShape::of::<$typ>())
    }};
    // Extended with custom $hs_type
    (($agent:expr, $counter:expr) [H::$hs_type:expr] $(>$req_type:expr)?) => {{
        use $crate::trace::TlsMessageType;
        use $crate::term::signature::Signature;

        let var = Signature::new_var_by_type_id($($req_type)?, $agent, Some(TlsMessageType::Handshake(Some($hs_type))), $counter);
        $crate::term::Term::Variable(var)
    }};

    //
    // Application TlsMessageType
    //
    (($agent:expr, $counter:expr) [A] / $typ:ty $(>$req_type:expr)?) => {{
        use crate::term::dynamic_function::TypeShape;
        term!(($agent, $counter) [A] > TypeShape::of::<$typ>())
    }};
    (($agent:expr, $counter:expr) [A] $(>$req_type:expr)?) => {{
        use $crate::trace::TlsMessageType;
        use $crate::term::signature::Signature;

        let var = Signature::new_var_by_type_id($($req_type)?, $agent, Some(TlsMessageType::ApplicationData), $counter);
        $crate::term::Term::Variable(var)
    }};

    //
    // Alert TlsMessageType todo
    //

    //
    // Heartbleed TlsMessageType todo
    //

    //
    // ChangeCipherSpec TlsMessageType todo
    //

    //
    // Function Applications
    //
    ($func:ident ($($args:tt),*) $(>$req_type:expr)?) => {{
        use $crate::term::dynamic_function::TypeShape;
        use $crate::trace::TlsMessageType;
        use $crate::term::signature::Signature;

        let func = Signature::new_function(&$func);
        let mut i = 0;

        let arguments = vec![$({
            let argument: &TypeShape = &func.shape().argument_types[i];
            $crate::term_arg!($args > argument.clone())
        }),*];

        $crate::term::Term::Application(func, arguments)
    }};
    // Shorthand for constants
    ($func:ident $(>$req_type:expr)?) => {{
        use $crate::term::signature::Signature;

        let func = Signature::new_function(&$func);
        $crate::term::Term::Application(func, vec![])
    }};

    //
    // Allows to use variables which already contain a term by starting with a `@`
    //
    (@$e:ident $(>$req_type:expr)?) => {{
        let subterm: &$crate::term::Term = &$e;
        subterm.clone()
    }};
}

#[macro_export]
macro_rules! term_arg {
    // Somehow the following rules is very important
    ( ( $($e:tt)* ) $(>$req_type:expr)?) => (term!($($e)* $(>$req_type)?));
    // not sure why I should need this
    // ( ( $e:tt ) ) => (ast!($e));
    ($e:tt $(>$req_type:expr)?) => (term!($e $(>$req_type)?));
}
