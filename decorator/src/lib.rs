pub trait TNoticer {
    fn notice(&self) -> String;
}

pub struct Noticer;
impl TNoticer for Noticer {
    fn notice(&self) -> String {
        "noticer default".to_string()
    }
}

pub struct DecoratedNoticer {
    noticer: Box<dyn TNoticer>,
}
impl DecoratedNoticer {
    pub fn new(noticer: Box<dyn TNoticer>) -> Self {
        DecoratedNoticer { noticer: noticer }
    }
}
impl TNoticer for DecoratedNoticer {
    fn notice(&self) -> String {
        let ret = self.noticer.notice();
        format!("dec {} orated", ret)
    }
}

pub struct DDNoticer {
    noticer: Box<dyn TNoticer>,
}
impl DDNoticer {
    pub fn new(noticer: Box<dyn TNoticer>) -> Self {
        DDNoticer { noticer: noticer }
    }
}
impl TNoticer for DDNoticer {
    fn notice(&self) -> String {
        let ret = self.noticer.notice();
        format!("double {} decorated", ret)
    }
}
