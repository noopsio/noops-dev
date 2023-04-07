wit_bindgen::generate!({
    world: "handler",
    path: "../../wit"
});

struct MyHandler;

impl Handler for MyHandler {
    fn handle(req: Request) -> Response {
        Response { status: 200 }
    }
}

export_handler!(MyHandler);
