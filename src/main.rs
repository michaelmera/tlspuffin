#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use crate::trace::{TraceContext, Step, ClientHelloSendAction, ServerHelloExpectAction};
use crate::variable::{
    CipherSuiteData, VersionData, CompressionData, ClientExtensionData, RandomData, SessionIDData,
    VariableData,
};
use std::thread;
use core::time;

mod agent;
mod debug;
mod io;
mod openssl_server;
mod trace;
mod variable;

fn main() {
    pretty_env_logger::init();
    //pretty_env_logger::formatted_builder().target(Target::Stdout).filter_level(LevelFilter::Trace).init();

    info!("{}", openssl_server::openssl_version());

    loop {
        let mut ctx = TraceContext::new();
        let openssl_server_agent = ctx.new_openssl_agent(true);
        let honest_agent = ctx.new_agent();
        let openssl_client_agent = ctx.new_openssl_agent(false);
        // TODO: let attacker_agent = ctx.new_agent();

        let client_hello = ClientHelloSendAction::new();
        let server_hello = ServerHelloExpectAction::new();
        let mut trace = trace::Trace {
            steps: vec![
                Step {
                    from: honest_agent,
                    to: openssl_server_agent,
                    action: &client_hello
                },
                Step {
                    from: openssl_server_agent,
                    to: honest_agent,
                    action: &server_hello
                },
            ],
        };

        info!("{}", trace);

        ctx.add_variable(Box::new(VersionData::random_value(honest_agent)));
        ctx.add_variable(Box::new(SessionIDData::random_value(honest_agent)));
        ctx.add_variable(Box::new(RandomData::random_value(honest_agent)));

        // A random extension
        //ctx.add_variable(Box::new(ExtensionData::random_value(fuzz_agent)));

        // Some static extensions
        ctx.add_variable(Box::new(ClientExtensionData::static_extension(honest_agent,
                                                                        ClientExtensionData::key_share(),
        )));
        ctx.add_variable(Box::new(ClientExtensionData::static_extension(honest_agent,
                                                                        ClientExtensionData::supported_versions(),
        )));
        ctx.add_variable(Box::new(ClientExtensionData::static_extension(honest_agent,
                                                                        ClientExtensionData::supported_groups(),
        )));
        ctx.add_variable(Box::new(ClientExtensionData::static_extension(honest_agent,
                                                                        ClientExtensionData::server_name("maxammann.org"),
        )));
        ctx.add_variable(Box::new(ClientExtensionData::static_extension(honest_agent,
                                                                        ClientExtensionData::signature_algorithms(),
        )));

        ctx.add_variable(Box::new(CipherSuiteData::random_value(honest_agent)));
        ctx.add_variable(Box::new(CipherSuiteData::random_value(honest_agent)));
        ctx.add_variable(Box::new(CipherSuiteData::random_value(honest_agent)));
        ctx.add_variable(Box::new(CompressionData::random_value(honest_agent)));

        trace.execute(&mut ctx);

        thread::sleep(time::Duration::from_millis(500));
    }
}
