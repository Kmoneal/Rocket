// extern crate rocket;
extern crate rustls;
extern crate hyper_sync_rustls;

pub use self::hyper_sync_rustls::{util, WrappedStream, ServerSession, TlsServer};
pub use self::rustls::{Certificate, PrivateKey, RootCertStore};

// use super::rocket::outcome::self;
// use rocket::outcome::Outcome::*;

// use rocket::request::Request;

/*
#[derive(Debug)]
pub struct MutualTlsUser {
    peer_certs: Vec<Certificate>,
}

impl MutualTlsUser {
    pub fn new(peer_certs: Vec<Certificate>) -> MutualTlsUser {
        MutualTlsUser {
            peer_certs
        }
    }

    /// Get the common name
    pub fn name(&self) -> String {
        unimplemented!();
    }
}

        // Fail if there are no client certificates
        // If there are client certs, the chain is guaranteed to be rooted in our trust roots,
        // but we still need to check the common name
impl <'a, 'r> FromRequest<'a, 'r> for MutualTlsUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.get_peer_certificates() {
            Some(certs) => Success(MutualTlsUser::new(certs)),
            None => Forward(())
        }
    }
}
*/
