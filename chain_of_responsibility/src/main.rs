use chain_of_responsibility::{Auth, Handler, Request, Store};

fn main() {
    let req = Request {
        data: "this is a request".to_string(),
    };

    let mut auth = Auth::new();
    let mut store = Store::new();
    let sec = Auth::new();

    store.set_next(Box::new(&sec));
    auth.set_next(Box::new(&store));

    auth.handle(req);
}
