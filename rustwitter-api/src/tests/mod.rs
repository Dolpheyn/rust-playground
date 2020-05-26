use super::server;
use super::json;

use http_types::{Method, Request, Url};
use http_service_mock::make_server;

#[async_std::test]
async fn index_test() {
    let app = server().await;
    let mut server = make_server(app).unwrap();

    let req = Request::new(
        Method::Get,
        Url::parse("http://127.0.0.1:8000/").unwrap(),
    );

    let res = server.simulate(req).unwrap();

    let expected_body = json!({
        "status": "Ok",
        "data": {
            "cutie": "nadia",
        },
    });
    assert_eq!(res.status(), 200);
    assert_eq!(res.body_string().await.unwrap(), expected_body.to_string());
}
