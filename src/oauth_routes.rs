use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use errors::Error;
use hydra;
use user_routes::User;

pub fn routes() -> Vec<rocket::Route> {
    routes![] //get_consent_info, accept_consent, reject_consent]
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

//#[get("/oauth/consent?<consent_id>")]
//pub fn get_consent_info(
//    consent_id: ConsentId,
//    hydra: State<hydra::client::Client>,
//) -> Json<Result<ConsentInfo, Error>> {
//    match hydra.consent_get(&consent_id.id) {
//        Ok(info) => Json(Ok(ConsentInfo {
//            client: info.client_id,
//            scopes: info.requested_scopes,
//            redirect: info.redirect_url,
//        })),
//        Err(e) => Json(Err(Error::client_error(format!(
//            "error getting info about consent request: {}",
//            e
//        )))),
//    }
//}
//
//#[derive(Deserialize)]
//pub struct AcceptConsent {
//    id: String,
//    scopes: Vec<String>,
//}
//
//#[post("/oauth/consent/accept", format = "application/json", data = "<req>")]
//pub fn accept_consent(
//    req: Json<AcceptConsent>,
//    user: User,
//    hydra: State<hydra::client::Client>,
//) -> Json<Result<(), Error>> {
//    match hydra.consent_accept(&req.id, &req.scopes, user.uuid) {
//        Ok(()) => Json(Ok(())),
//        Err(e) => Json(Err(Error::client_error(format!(
//            "error accepting consent: {}",
//            e
//        )))),
//    }
//}
//
//#[derive(Deserialize)]
//pub struct RejectConsent {
//    id: String,
//    reason: String,
//}
//
//#[post("/oauth/consent/reject", format = "application/json", data = "<req>")]
//pub fn reject_consent(
//    req: Json<RejectConsent>,
//    hydra: State<hydra::client::Client>,
//) -> Json<Result<(), Error>> {
//    match hydra.consent_reject(&req.id, &req.reason) {
//        Ok(()) => Json(Ok(())),
//        Err(e) => Json(Err(Error::client_error(format!(
//            "error accepting consent: {}",
//            e
//        )))),
//    }
//}
