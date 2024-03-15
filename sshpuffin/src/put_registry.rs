use puffin::{
    put::PutName,
    put_registry::{Factory, PutRegistry},
};

use crate::protocol::SshProtocolBehavior;

pub const LIBSSH_PUT: PutName = PutName(['L', 'I', 'B', 'S', 'S', 'H', '_', '_', '_', '_']);

pub fn ssh_default_registry() -> PutRegistry<SshProtocolBehavior> {
    PutRegistry::new(&[crate::libssh::new_libssh_factory], DEFAULT_PUT_FACTORY)
}

pub const DEFAULT_PUT_FACTORY: fn() -> Box<dyn Factory<SshProtocolBehavior>> =
    crate::libssh::new_libssh_factory;
