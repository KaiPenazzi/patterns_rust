pub struct Singelton {
    pub data: Data,
}

impl Singelton {
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: Option<Singelton> = None;

        unsafe {
            INSTANCE.get_or_insert_with(|| Singelton {
                data: Data {
                    name: "Kai".to_string(),
                    age: 22,
                },
            })
        }
    }

    pub fn set_age(&mut self, new_age: i32) {
        self.data.age = new_age;
    }

    pub fn set_name(&mut self, new_name: String) {
        self.data.name = new_name;
    }
}

pub struct Data {
    pub name: String,
    pub age: i32,
}
