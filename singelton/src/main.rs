use singelton::Singelton;

fn main() {
    let mut _sing = Singelton::instance();
    let _sign2 = Singelton::instance();

    println!("age: {}", _sing.data.age);
    println!("name: {}", _sing.data.name);

    println!("age: {}", _sign2.data.age);
    println!("name: {}", _sign2.data.name);

    _sing.set_age(50);
    _sing.set_name("stuff".to_string());

    println!("age: {}", _sing.data.age);
    println!("name: {}", _sing.data.name);

    println!("age: {}", _sign2.data.age);
    println!("name: {}", _sign2.data.name);
}
