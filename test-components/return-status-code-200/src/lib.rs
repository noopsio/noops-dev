wit_bindgen::generate!({
    world: "handler",
    path: "../../wit",
    exports: {
        world: TestHandler
    }
});

struct TestHandler;

impl Guest for TestHandler {
    fn handle(_: Request) -> Response {
        Response {
            status: 200,
            body: Default::default(),
        }
    }
}
