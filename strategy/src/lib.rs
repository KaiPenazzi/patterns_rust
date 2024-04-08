pub struct Navigator {
    route_strategy: Box<dyn RouteStrategy>,
}

impl Navigator {
    pub fn new(navigator: Box<dyn RouteStrategy>) -> Self {
        Navigator {
            route_strategy: navigator,
        }
    }

    pub fn build_route(&self, a: i32, b: i32) {
        self.route_strategy.build_route(a, b);
    }
}

pub trait RouteStrategy {
    fn build_route(&self, a: i32, b: i32);
}

pub struct Road {}
impl RouteStrategy for Road {
    fn build_route(&self, a: i32, b: i32) {
        println!("Road: {} to {}", a, b);
    }
}

pub struct Public {}
impl RouteStrategy for Public {
    fn build_route(&self, a: i32, b: i32) {
        println!("Public: {} to {}", a, b);
    }
}

pub struct Walking {}
impl RouteStrategy for Walking {
    fn build_route(&self, a: i32, b: i32) {
        println!("Walking: {} to {}", a, b);
    }
}
