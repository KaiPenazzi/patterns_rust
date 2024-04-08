use strategy::{Navigator, Public, Road, Walking};
fn main() {
    let road = Road {};
    let nav = Navigator::new(Box::new(road));
    nav.build_route(20, 40);

    let public = Public {};
    let nav2 = Navigator::new(Box::new(public));
    nav2.build_route(20, 40);

    let walk = Walking {};
    let nav3 = Navigator::new(Box::new(walk));
    nav3.build_route(20, 40);
}
