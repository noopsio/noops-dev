wit_bindgen::generate!({
    world: "handler",
    path: "../../../wit"
});

struct TestHandler;

impl Handler for TestHandler {
    fn handle(req: Request) -> Response {
        let mut response_body = String::default();
        for (key, value) in req.query_params {
            response_body.push_str(&format!("{}={}\n", key, value));
        }
        let response_body = response_body;

        Response {
            status: 200,
            body: response_body,
        }
    }
}

export_handler!(TestHandler);
