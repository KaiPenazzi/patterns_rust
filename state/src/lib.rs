pub struct Humam {
    state: Box<dyn State>,
}
impl Humam {
    pub fn new() -> Self {
        Humam {
            state: Box::new(Sleep {}),
        }
    }

    pub fn go_to_work(&mut self) {
        self.state = Box::new(Work {});
    }

    pub fn finished_work(&mut self) {
        self.state = Box::new(FreeTime {});
    }

    pub fn go_sleep(&mut self) {
        self.state = Box::new(Sleep {});
    }

    pub fn do_stuff(&self) {
        self.state.do_stuff();
    }
}

trait State {
    fn do_stuff(&self);
}

struct Work {}
impl State for Work {
    fn do_stuff(&self) {
        println!("working");
    }
}

struct Sleep {}
impl State for Sleep {
    fn do_stuff(&self) {
        println!("sleeping");
    }
}

struct FreeTime {}
impl State for FreeTime {
    fn do_stuff(&self) {
        println!("having fun ");
    }
}
