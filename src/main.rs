use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, BTreeMap};
use std::sync::Arc;
use tokio::sync::Mutex;
use voting::{
    ElectionID, Nonce, Vote,
    pseudonym::{
        Credential, CredentialRequest, CredentialResponse, IssuerPrivateKey, IssuerPublicKey,
    },
};

/// Current status of a registration request.
enum RegistrationStatus {
    Request(CredentialRequest),
    Response(CredentialResponse),
    Reject(String),
}

/// The state kept by the server.
struct State {
    issuer_public_key: IssuerPublicKey,
    requests: BTreeMap<String, RegistrationStatus>,
    issued: BTreeSet<String>,
    votes: BTreeMap<ElectionID, BTreeMap<Nonce, Vote>>,
}

impl State {
    fn new() -> Self {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/votes", get(votes))
        .route("/vote", post(vote))
        .route("/register", post(register))
        .route("/registration-status", get(registration_status))
        .route("/registrations", get(registrations))
        .route("/review-registration", post(review_registration))
        .route("/start-election", post(start_election))
        .route("/stop-election", post(stop_election))
        .route("/issuer-public-key", post(issuer_public_key))
        .with_state(Arc::new(Mutex::new(State::new())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Returns all of the votes so far. Only works once the election is stopped, to prevent timing
/// attacks.
async fn votes() -> &'static str {
    "TODO"
}

/// Vote in the given election. Only works once the election is started and before it is stopped.
async fn vote() -> &'static str {
    "TODO"
}

/// Register for a credential. This sends a request for a credential, awaiting review by the
/// issuer.
async fn register() -> &'static str {
    "TODO"
}

/// Checks up on the status of our registration.
async fn registration_status() -> &'static str {
    "TODO"
}

/// Returns the list of attempted registrations to the issuer, so they can choose which ones to
/// approve or reject.
async fn registrations() -> &'static str {
    "TODO"
}

/// Reviews a registration, either accepting or denying it.
async fn review_registration() -> &'static str {
    "TODO"
}

/// Starts the election. Can only be run by the issuer.
async fn start_election() -> &'static str {
    "TODO"
}

/// Stops the election. Can only be run by the issuer.
async fn stop_election() -> &'static str {
    "TODO"
}

/// Returns the issuer's public key.
async fn issuer_public_key() -> &'static str {
    "TODO"
}
