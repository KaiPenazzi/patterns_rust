pub trait Handler<'a> {
    fn set_next(&mut self, handler: Box<&'a dyn Handler<'a>>);
    fn handle(&self, req: Request);
}

pub struct Request {
    pub data: String,
}

pub struct Auth<'a> {
    next: Option<Box<&'a dyn Handler<'a>>>,
}

impl<'a> Handler<'a> for Auth<'a> {
    fn set_next(&mut self, handler: Box<&'a dyn Handler<'a>>) {
        self.next = Some(handler);
    }

    fn handle(&self, req: Request) {
        println!("Auth: {}", req.data);

        match &self.next {
            Some(next) => next.handle(req),
            None => println!("end"),
        }
    }
}

impl<'b> Auth<'b> {
    pub fn new() -> Self {
        Auth { next: None }
    }
}

pub struct Store<'a> {
    next: Option<Box<&'a dyn Handler<'a>>>,
}

impl<'a> Handler<'a> for Store<'a> {
    fn set_next(&mut self, handler: Box<&'a dyn Handler<'a>>) {
        self.next = Some(handler);
    }

    fn handle(&self, req: Request) {
        println!("Store: {}", req.data);

        match &self.next {
            Some(next) => next.handle(req),
            None => println!("end"),
        }
    }
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Store { next: None }
    }
}
