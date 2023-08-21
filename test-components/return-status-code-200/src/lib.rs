wit_bindgen::generate!({
    world: "handler",
    path: "../../wit"
});

struct TestHandler;

impl Handler for TestHandler {
    fn handle(_: Request) -> Response {
        Response {
            status: 200,
            body: Default::default(),
        }
    }
}

export_handler!(TestHandler);
