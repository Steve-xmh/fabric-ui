//! An application struct



use crate::{system::traits::SystemDrawableWindow, traits::{Control}, utils::uid::gen_uid};

pub struct Application<D> {
    system_window: Box<dyn SystemDrawableWindow>,
    root: WidgetPod<D>,
    data: D
}

impl<D> Application<D> {
    pub fn new(window: Box<dyn Control<D>>, data: D) -> Self {
        Self {
            system_window: Box::new(crate::system::SystemWindow::new()),
            root: window.into(),
            data,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.system_window.query_event(false);
        }
    }
}

pub struct WidgetPod<D = ()> {
    pub widget_id: usize,
    pub widget: Box<dyn Control<D>>,
}

impl<D> From<Box<dyn Control<D>>> for WidgetPod<D> {
    fn from(c: Box<dyn Control<D>>) -> Self {
        Self {
            widget_id: gen_uid(),
            widget: c
        }
    }
}

impl<D: PartialEq> Application<D> {

}
