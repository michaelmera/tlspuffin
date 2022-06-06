use ring::digest;
use ring::hkdf::Prk;
use rustls::hash_hs::HandshakeHash;
use rustls::tls13::key_schedule::{KeyScheduleEarly, KeyScheduleHandshake, KeyScheduleHandshakeStart, KeySchedulePreHandshake, KeyScheduleTrafficWithClientFinishedPending};

use rustls::msgs::enums::NamedGroup;
use rustls::NoKeyLog;
use rustls::SupportedCipherSuite;

use crate::tls::error::FnError;
use crate::tls::key_exchange::tls13_key_exchange;

pub fn tls13_handshake_traffic_secret(
    server_hello: &HandshakeHash,
    server_key_share: &Option<Vec<u8>>,
    psk: &Option<Vec<u8>>,
    server: bool,
) -> Result<(&'static SupportedCipherSuite, Prk, KeyScheduleHandshake), FnError> {
    let client_random = &[1u8; 32]; // todo see op_random() https://gitlab.inria.fr/mammann/tlspuffin/-/issues/45
    let suite = &rustls::tls13::TLS13_AES_128_GCM_SHA256; // todo see op_cipher_suites() https://gitlab.inria.fr/mammann/tlspuffin/-/issues/45
    let group = NamedGroup::secp384r1; // todo https://gitlab.inria.fr/mammann/tlspuffin/-/issues/45
    let mut key_schedule = dhe_key_schedule(suite, group, server_key_share, psk)?;

    let (hs, client_secret, server_secret) = key_schedule.derive_handshake_secrets(
        &server_hello.get_current_hash_raw(),
        &NoKeyLog {},
        client_random,
    );

    Ok((
        suite,
        if server { client_secret } else { server_secret },
        hs,
    ))
}

pub fn tls13_application_traffic_secret(
    server_hello: &HandshakeHash,
    server_finished: &HandshakeHash,
    server_key_share: &Option<Vec<u8>>,
    psk: &Option<Vec<u8>>,
    server: bool,
) -> Result<
    (
        &'static SupportedCipherSuite,
        Prk,
        KeyScheduleTrafficWithClientFinishedPending,
    ),
    FnError,
> {
    let client_random = &[1u8; 32]; // todo see op_random() https://gitlab.inria.fr/mammann/tlspuffin/-/issues/45
    let (suite, _key, key_schedule) =
        tls13_handshake_traffic_secret(server_hello, server_key_share, psk, server)?;

    let (mut pending, client_secret, server_secret) = key_schedule.into_traffic_with_client_finished_pending_raw(&server_finished.get_current_hash_raw(), &NoKeyLog {},
                                                                                                                 client_random,);
    Ok((
        suite,
        if server { client_secret } else { server_secret },
        pending,
    ))
}

pub fn tls13_derive_psk(
    server_hello: &HandshakeHash,
    server_finished: &HandshakeHash,
    client_finished: &HandshakeHash,
    server_key_share: &Option<Vec<u8>>,
    new_ticket_nonce: &Vec<u8>,
) -> Result<Vec<u8>, FnError> {
    let client_random = &[1u8; 32]; // todo see op_random() https://gitlab.inria.fr/mammann/tlspuffin/-/issues/45

    let (_, _, mut pending) = tls13_application_traffic_secret(
        server_hello,
        server_finished,
        server_key_share,
        &None,
        true,
    )?;

/*    application_key_schedule.exporter_master_secret_raw(
        &server_finished.get_current_hash_raw(),
        &NoKeyLog {},
        client_random,
    );*/

    let (traffic, tag, client_secret) = pending.sign_client_finish_raw(    &server_finished.get_current_hash_raw());
    let psk = traffic
        .resumption_master_secret_and_derive_ticket_psk_raw(
            &client_finished.get_current_hash_raw(),
            new_ticket_nonce,
        );

    Ok(psk)
}

pub fn dhe_key_schedule(
    suite: &SupportedCipherSuite,
    group: NamedGroup,
    server_key_share: &Option<Vec<u8>>,
    psk: &Option<Vec<u8>>,
) -> Result<KeyScheduleHandshakeStart, FnError> {
    let hkdf_algorithm = suite.tls13().ok_or_else(|| FnError::Rustls("No tls 1.3 suite".to_owned()))?.hkdf_algorithm;

    // Key Schedule with or without PSK
    let key_schedule = match (server_key_share, psk) {
        (Some(server_key_share), Some(psk)) => {
            let shared_secret = tls13_key_exchange(server_key_share, group)?;
            let early = KeyScheduleEarly::new(hkdf_algorithm, psk.as_slice());
            let pre: KeySchedulePreHandshake = early.into();
            Ok(pre.into_handshake(&shared_secret))
        }
        (Some(server_key_share), None) => {
            let shared_secret = tls13_key_exchange(server_key_share, group)?;
            Ok(KeySchedulePreHandshake::new(hkdf_algorithm).into_handshake(&shared_secret))
        }
        (None, Some(psk)) => {
            // todo this empty secret is not specified in the RFC 8446
            let zeroes = [0u8; digest::MAX_OUTPUT_LEN];
            let early = KeyScheduleEarly::new(hkdf_algorithm, psk.as_slice());
            let pre: KeySchedulePreHandshake = early.into();
            Ok(
                pre.into_handshake(
                    &zeroes[..hkdf_algorithm
                        .hmac_algorithm()
                        .digest_algorithm()
                        .output_len],
                ),
            )
        }
        (None, None) => Err(FnError::Unknown(
            "Need at least a key share or a psk".to_owned(),
        )),
    };

    key_schedule
}
