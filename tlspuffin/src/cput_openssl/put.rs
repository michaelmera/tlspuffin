use log::info;
use puffin::codec::Reader;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::rc::Rc;

use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

use crate::{
    protocol::TLSProtocolBehavior,
    put::TlsPutConfig,
    put_registry::C_PUT,
    tls::rustls::msgs::{
        deframer::MessageDeframer,
        message::{Message, OpaqueMessage},
    },
};

use puffin::{
    agent::{AgentDescriptor, AgentName, AgentType},
    error::Error,
    protocol::MessageResult,
    put::{Put, PutName},
    put_registry::Factory,
    stream::{MemoryStream, Stream},
    trace::TraceContext,
};

use crate::cput_openssl::bindings::CPUT;

pub fn new_cput_openssl_factory() -> Box<dyn Factory<TLSProtocolBehavior>> {
    struct CPUTOpenSSLFactory;
    impl Factory<TLSProtocolBehavior> for CPUTOpenSSLFactory {
        fn create(
            &self,
            context: &TraceContext<TLSProtocolBehavior>,
            agent_descriptor: &AgentDescriptor,
        ) -> Result<Box<dyn Put<TLSProtocolBehavior>>, Error> {
            let put_descriptor = context.put_descriptor(agent_descriptor);

            let options = &put_descriptor.options;

            let use_clear = options
                .get_option("use_clear")
                .map(|value| value.parse().unwrap_or(false))
                .unwrap_or(false);

            // FIXME: Add non-clear method like in wolfssl
            if !use_clear {
                info!("OpenSSL put does not support clearing mode")
            }

            let config = TlsPutConfig {
                descriptor: agent_descriptor.clone(),
                claims: context.claims().clone(),
                authenticate_peer: agent_descriptor.typ == AgentType::Client
                    && agent_descriptor.server_authentication
                    || agent_descriptor.typ == AgentType::Server
                        && agent_descriptor.client_authentication,
                extract_deferred: Rc::new(RefCell::new(None)),
                use_clear,
            };

            Ok(Box::new(CPUTOpenSSL::new(config).map_err(|err| {
                Error::Put(format!("Failed to create client/server: {}", err))
            })?))
        }

        fn name(&self) -> PutName {
            C_PUT
        }

        fn version(&self) -> String {
            CPUTOpenSSL::version()
        }
    }

    Box::new(CPUTOpenSSLFactory)
}

pub struct CPUTOpenSSL {
    pub config: TlsPutConfig,
    pub c_data: *mut ::std::os::raw::c_void,
}

impl Stream<Message, OpaqueMessage> for CPUTOpenSSL {
    fn add_to_inbound(&mut self, message: &OpaqueMessage) {}

    fn take_message_from_outbound(
        &mut self,
    ) -> Result<Option<MessageResult<Message, OpaqueMessage>>, Error> {
        // make a vec<u8> and convert it to channel to use puffin::stream::Stream implementation
        let mut stream = MemoryStream::new(MessageDeframer::new());
        let content = vec![1u8, 2u8];
        stream.write(&content);
        stream.take_message_from_outbound()
    }
}

impl Put<TLSProtocolBehavior> for CPUTOpenSSL {
    fn progress(&mut self, agent_name: &AgentName) -> Result<(), Error> {
        unsafe { (CPUT.progress.unwrap())(self.c_data, (*agent_name).into()) };
        Ok(())
    }

    fn reset(&mut self, agent_name: AgentName) -> Result<(), Error> {
        unsafe { (CPUT.reset.unwrap())(self.c_data, agent_name.into()) };
        Ok(())
    }

    fn descriptor(&self) -> &AgentDescriptor {
        &self.config.descriptor
    }

    fn rename_agent(&mut self, agent_name: AgentName) -> Result<(), Error> {
        unsafe { (CPUT.rename_agent.unwrap())(self.c_data, agent_name.into()) };
        Ok(())
    }

    fn describe_state(&self) -> &str {
        unsafe {
            CStr::from_ptr((CPUT.describe_state.unwrap())(self.c_data))
                .to_str()
                .unwrap()
        }
    }

    fn is_state_successful(&self) -> bool {
        unsafe { (CPUT.is_state_successful.unwrap())(self.c_data) }
    }

    fn set_deterministic(&mut self) -> Result<(), Error> {
        unsafe { (CPUT.set_deterministic.unwrap())(self.c_data) };
        Ok(())
    }

    fn shutdown(&mut self) -> String {
        unsafe {
            CStr::from_ptr((CPUT.shutdown.unwrap())(self.c_data))
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    fn version() -> String {
        unsafe {
            CStr::from_ptr((CPUT.version.unwrap())())
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}

impl CPUTOpenSSL {
    fn new(config: TlsPutConfig) -> Result<CPUTOpenSSL, Error> {
        Ok(CPUTOpenSSL {
            config,
            c_data: unsafe { (CPUT.new.unwrap())() },
        })
    }
}

// pub unsafe extern "C" _log(mut ap: ...) {
// }

#[cfg(test)]
mod tests {
    use super::new_cput_openssl_factory;
    use super::CPUTOpenSSL;
    use puffin::put::Put;

    #[test]
    fn create_cput_openssl_factory() {
        new_cput_openssl_factory();
        return;
    }

    #[test]
    fn valid_cput_version() {
        assert_eq!(CPUTOpenSSL::version(), "0.0.1-dummy-cputopenssl");
        return;
    }
}