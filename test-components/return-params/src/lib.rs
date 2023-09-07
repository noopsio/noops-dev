wit_bindgen::generate!({
    world: "handler",
    path: "../../wit",
    exports: {
        world: TestHandler
    }
});

struct TestHandler;

impl Guest for TestHandler {
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
