#include <stdlib.h>
#include <stdio.h>
#include <openssl/ssl.h>
#include <openssl/decoder.h>

#include "put.h"

typedef struct
{
    AGENT_DESCRIPTOR *descriptor;

    SSL *ssl;

    BIO *in;
    BIO *out;
} AGENT;

const char *openssl_version();
void *openssl_create(AGENT_DESCRIPTOR *descriptor);
void *openssl_create_client(AGENT_DESCRIPTOR *descriptor);
void *openssl_create_server(AGENT_DESCRIPTOR *descriptor);
void openssl_progress(void *put, uint8_t agent_name);
void openssl_reset(void *put, uint8_t agent_name);
void openssl_rename(void *put, uint8_t agent_name);
const char *openssl_describe_state(void *put);
bool openssl_is_successful(void *put);
void openssl_set_deterministic(void *put);
const char *openssl_shutdown(void *put);
int openssl_add_inbound(void *put, const uint8_t *bytes, size_t length, size_t *written);
int openssl_take_outbound(void *put, uint8_t *bytes, size_t max_length, size_t *readbytes);

const C_PUT_TYPE CPUT = {
    .create = openssl_create,
    .version = openssl_version,

    .progress = openssl_progress,
    .reset = openssl_reset,
    .rename_agent = openssl_rename,
    .describe_state = openssl_describe_state,
    .is_state_successful = openssl_is_successful,
    .set_deterministic = openssl_set_deterministic,
    .shutdown = openssl_shutdown,

    .add_inbound = openssl_add_inbound,
    .take_outbound = openssl_take_outbound,
};

// TODO: `log` should be moved to the public cput interface
//
//     The `log` function is a convenient utility function and should be useful
//     for every implementer of the cput interface.
//
//     The declaration should be part of the standard header for cput `put.h`.
//     The implementation should be statically linked inside tlspuffin.

void log(void (*logger)(const char *), const char *format, ...);

const int tls_version[] = {TLS1_3_VERSION, TLS1_2_VERSION};
const char *version_str[] = {"V1_3", "V1_2"};
const char *type_str[] = {"client", "server"};

const char *openssl_version()
{
    return OPENSSL_FULL_VERSION_STR;
}

void *openssl_create(AGENT_DESCRIPTOR *descriptor)
{
    log(TLSPUFFIN.info, "descriptor %u version: %s type: %s", descriptor->name, version_str[descriptor->tls_version], type_str[descriptor->type]);

    SSL_library_init();

    if (descriptor->type == CLIENT)
    {
        return openssl_create_client(descriptor);
    }

    if (descriptor->type == SERVER)
    {
        return openssl_create_server(descriptor);
    }

    log(TLSPUFFIN.error, "unknown agent type for descriptor %u: %u", descriptor->name, descriptor->type);
    return NULL;
}

void openssl_progress(void *put, uint8_t agent_name)
{
    if (!openssl_is_successful(put))
    {
        // not connected yet -> do handshake
        SSL_do_handshake(((AGENT *)put)->ssl);
        return;
    }

    // trigger another read
    void *buf = malloc(128);
    SSL_read(((AGENT *)put)->ssl, buf, 128);
    return;
}

void openssl_reset(void *put, uint8_t agent_name)
{
    SSL_clear(((AGENT *)put)->ssl);
}

void openssl_rename(void *put, uint8_t agent_name)
{
    return;
}

const char *openssl_describe_state(void *put)
{
    // NOTE: Very useful for nonblocking according to docs:
    //     https://www.openssl.org/docs/manmaster/man3/SSL_state_string.html
    //
    //     When using nonblocking sockets, the function call performing the
    //     handshake may return with SSL_ERROR_WANT_READ or SSL_ERROR_WANT_WRITE
    //     condition, so that SSL_state_string[_long]() may be called.
    return SSL_state_string_long(((AGENT *)put)->ssl);
}

bool openssl_is_successful(void *put)
{
    return (strstr(openssl_describe_state(put), "SSL negotiation finished successfully") != NULL);
}

void openssl_set_deterministic(void *put)
{
}

const char *openssl_shutdown(void *put)
{
    return "";
}

int openssl_add_inbound(void *put, const uint8_t *bytes, size_t length, size_t *written)
{
    int result = BIO_write_ex(((AGENT *)put)->in, bytes, length, written);

    return (result == 1) ? 0 : -1;
}

int openssl_take_outbound(void *put, uint8_t *bytes, size_t max_length, size_t *readbytes)
{
    int result = BIO_read_ex(((AGENT *)put)->out, bytes, max_length, readbytes);

    return (result == 1) ? 0 : -1;
}

void *openssl_create_client(AGENT_DESCRIPTOR *descriptor)
{
    SSL_CTX *ssl_ctx = SSL_CTX_new(TLS_method());
    SSL_CTX_set_max_proto_version(ssl_ctx, tls_version[descriptor->tls_version]);

    // Disallow EXPORT in client
    SSL_CTX_set_cipher_list(ssl_ctx, "ALL:!EXPORT:!LOW:!aNULL:!eNULL:!SSLv2");
    SSL_CTX_set_verify(ssl_ctx, SSL_VERIFY_NONE, NULL);

    if (descriptor->client_authentication)
    {
        // load cert
        X509 *cert = X509_new();
        BIO *cert_bio = BIO_new_mem_buf(descriptor->cert.bytes, descriptor->cert.length);
        cert = PEM_read_bio_X509(cert_bio, &cert, NULL, NULL);
        SSL_CTX_use_certificate(ssl_ctx, cert);

        // load pkey
        EVP_PKEY *pkey = NULL;
        BIO *pkey_bio = BIO_new_mem_buf(descriptor->pkey.bytes, descriptor->pkey.length);
        OSSL_DECODER_CTX *dctx = OSSL_DECODER_CTX_new_for_pkey(&pkey, "PEM", NULL, NULL, OSSL_KEYMGMT_SELECT_KEYPAIR, NULL, NULL);
        OSSL_DECODER_from_bio(dctx, pkey_bio);
        OSSL_DECODER_CTX_free(dctx);
        SSL_CTX_use_PrivateKey(ssl_ctx, pkey);
    }

    if (descriptor->server_authentication)
    {
        SSL_CTX_set_verify(ssl_ctx, SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT, NULL);

        // load certs in store
        X509_STORE *store = X509_STORE_new();
        for (size_t i = 0; descriptor->store[i] != NULL; ++i)
        {
            const PEM *const pem = descriptor->store[i];

            BIO *cert_bio = BIO_new_mem_buf(pem->bytes, pem->length);
            X509 *cert = PEM_read_bio_X509(cert_bio, NULL, NULL, NULL);
            X509_STORE_add_cert(store, cert);
        }

        SSL_CTX_set_cert_store(ssl_ctx, store);
    }

    SSL *ssl = SSL_new(ssl_ctx);
    SSL_set_connect_state(ssl);

    AGENT *agent = malloc(sizeof(AGENT));
    agent->descriptor = descriptor;
    agent->ssl = ssl;

    agent->in = BIO_new(BIO_s_mem());
    agent->out = BIO_new(BIO_s_mem());
    SSL_set_bio(agent->ssl, agent->in, agent->out);

    return agent;
}

void *openssl_create_server(AGENT_DESCRIPTOR *descriptor)
{
    SSL_CTX *ssl_ctx = SSL_CTX_new(TLS_method());
    SSL_CTX_set_max_proto_version(ssl_ctx, tls_version[descriptor->tls_version]);

    // Allow EXPORT in server
    SSL_CTX_set_cipher_list(ssl_ctx, "ALL:EXPORT:!LOW:!aNULL:!eNULL:!SSLv2");
    SSL_CTX_set_verify(ssl_ctx, SSL_VERIFY_NONE, NULL);

    // load cert
    X509 *cert = X509_new();
    BIO *cert_bio = BIO_new_mem_buf(descriptor->cert.bytes, descriptor->cert.length);
    cert = PEM_read_bio_X509(cert_bio, &cert, NULL, NULL);
    SSL_CTX_use_certificate(ssl_ctx, cert);

    // load pkey
    EVP_PKEY *pkey = NULL;
    BIO *pkey_bio = BIO_new_mem_buf(descriptor->pkey.bytes, descriptor->pkey.length);
    OSSL_DECODER_CTX *dctx = OSSL_DECODER_CTX_new_for_pkey(&pkey, "PEM", NULL, NULL, OSSL_KEYMGMT_SELECT_KEYPAIR, NULL, NULL);
    OSSL_DECODER_from_bio(dctx, pkey_bio);
    OSSL_DECODER_CTX_free(dctx);
    SSL_CTX_use_PrivateKey(ssl_ctx, pkey);

    if (descriptor->client_authentication)
    {
        SSL_CTX_set_verify(ssl_ctx, SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT, NULL);

        // load certs in store
        X509_STORE *store = X509_STORE_new();
        for (size_t i = 0; descriptor->store[i] != NULL; ++i)
        {
            const PEM *const pem = descriptor->store[i];

            BIO *cert_bio = BIO_new_mem_buf(pem->bytes, pem->length);
            X509 *cert = PEM_read_bio_X509(cert_bio, NULL, NULL, NULL);
            X509_STORE_add_cert(store, cert);
        }

        SSL_CTX_set_cert_store(ssl_ctx, store);
    }

    SSL *ssl = SSL_new(ssl_ctx);
    SSL_set_accept_state(ssl);

    AGENT *agent = malloc(sizeof(AGENT));
    agent->descriptor = descriptor;
    agent->ssl = ssl;

    agent->in = BIO_new(BIO_s_mem());
    agent->out = BIO_new(BIO_s_mem());
    SSL_set_bio(agent->ssl, agent->in, agent->out);

    return agent;
}

void log(void (*logger)(const char *), const char *format, ...)
{
    char *message = NULL;
    va_list args;

    va_start(args, format);
    vasprintf(&message, format, args);
    va_end(args);
    logger(message);

    free(message);
}
