wit_bindgen::generate!("handler");

struct MyHandler;

impl Handler for MyHandler {
    fn handle(req: Request) -> Response {
        println!("{:?}", req.params);
        Response { status: 200 }
    }
}

export_handler!(MyHandler);
