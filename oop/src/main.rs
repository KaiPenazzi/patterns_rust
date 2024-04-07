struct Company<'a> {
    emps: Vec<Box<&'a dyn Employe>>,
}

impl<'a> Company<'a> {
    fn new() -> Self {
        Company { emps: Vec::new() }
    }

    fn add_emp(&mut self, emp: &'a impl Employe) {
        self.emps.push(Box::new(emp));
    }

    fn working(&self) {
        for emp in &self.emps {
            emp.work();
        }
    }
}

trait Employe {
    fn work(&self);
}

struct Coder {}

impl Coder {
    fn new() -> Self {
        Coder {}
    }
}

impl Employe for Coder {
    fn work(&self) {
        println!("coding");
    }
}

struct Bwler {}

impl Bwler {
    fn new() -> Self {
        Bwler {}
    }
}

impl Employe for Bwler {
    fn work(&self) {
        println!("nothing");
    }
}

fn main() {
    let mut cmp = Company::new();

    let cod = Coder::new();
    let bw = Bwler::new();

    cmp.add_emp(&cod);
    cmp.add_emp(&bw);

    cmp.working();
}

