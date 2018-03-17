use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use errors::Error;
use hydra;
use hydra_client;
use user_routes::User;
use multi_reactor_drifting::{Handle, Future};
use futures::Future as _______________;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_consent_info, accept_consent, reject_consent]
}

#[derive(FromForm)]
pub struct ConsentId {
    id: String,
}

#[derive(Serialize)]
pub struct ConsentInfo {
    client: String,
    scopes: Vec<String>,
    redirect: String,
}

#[get("/oauth/consent?<consent_id>")]
pub fn get_consent_info(
    consent_id: ConsentId,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<Json<Result<ConsentInfo, Error>>, ()> {
    let client = hydra.build(&handle.into()).client();
    let json_future = client.o_auth2_api().get_o_auth2_consent_request(&consent_id.id)
        .then(|res| {
            match res {
                Ok(info) => Ok(Json(Ok(ConsentInfo {
                    client: info.client_id().unwrap().to_string(),
                    scopes: info.requested_scopes().unwrap().clone(),
                    redirect: info.redirect_url().unwrap().to_string(),
                }))),
                Err(e) => Ok(Json(Err(Error::client_error(format!(
                    "error getting info about consent request: {:?}",
                    e
                ))))),
            }
        });

    Future(Box::new(json_future))
}

#[derive(Deserialize)]
pub struct AcceptConsent {
    id: String,
    scopes: Vec<String>,
}

#[post("/oauth/consent/accept", format = "application/json", data = "<req>")]
pub fn accept_consent(
    req: Json<AcceptConsent>,
    user: User,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<Json<Result<(), Error>>, ()> {
    let client = hydra.build(&handle.into()).client();
    let accept = hydra_client::models::ConsentRequestAcceptance::new()
        .with_subject(user.uuid.simple().to_string())
        .with_grant_scopes(req.scopes.clone());
    let resp = client.o_auth2_api()
        .accept_o_auth2_consent_request(&req.id, accept)
        .then(|res| {
            match res {
                Ok(()) => Ok(Json(Ok(()))),
                Err(e) => Err(Json(Err(Error::client_error(format!(
                    "error accepting consent: {:?}",
                    e
                ))))),
            }
        }).map_err(|err: Json<Result<(), Error>>| {
            ()
        });

    Future(Box::new(resp))
}

#[derive(Deserialize)]
pub struct RejectConsent {
    id: String,
    reason: String,
}

#[post("/oauth/consent/reject", format = "application/json", data = "<req>")]
pub fn reject_consent(
    req: Json<RejectConsent>,
    user: User,
    hydra: State<hydra::client::ClientBuilder>,
    handle: Handle,
) -> Future<Json<Result<(), Error>>, ()> {
    let client = hydra.build(&handle.into()).client();
    let accept = hydra_client::models::ConsentRequestRejection::new()
        .with_reason("user rejected".to_string());
    let reject = client.o_auth2_api().reject_o_auth2_consent_request(&req.id, accept).then(|res| {
        match res {
            Ok(()) => Ok(Json(Ok(()))),
            Err(e) => Ok(Json(Err(Error::client_error(format!(
                "error accepting consent: {:?}",
                e
            ))))),
        }
    }).map_err(|_: ()| ());

    Future(Box::new(reject.map_err(|_| ())))
}
