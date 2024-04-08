use decorator::{DDNoticer, DecoratedNoticer, Noticer, TNoticer};

fn main() {
    let ntc = Noticer {};
    println!("ntc: {}", ntc.notice());

    let dntc = DecoratedNoticer::new(Box::new(ntc));
    println!("dntc: {}", dntc.notice());

    let ddntc = DDNoticer::new(Box::new(dntc));
    println!("ddntc: {}", ddntc.notice());
}
