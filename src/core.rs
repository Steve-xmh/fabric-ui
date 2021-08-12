//! An application struct



use crate::{DrawCtx, EventCtx, r#box::AreaBox, system::traits::SystemDrawableWindow, traits::{Widget}, utils::{WidgetUid, uid::gen_uid}};

pub struct Application<D> {
    system_window: Box<dyn SystemDrawableWindow>,
    root: WidgetPod<D>,
    data: D
}

impl<D> Application<D> {
    pub fn new(window: Box<dyn Widget<D>>, data: D) -> Self {
        Self {
            system_window: Box::new(crate::system::SystemWindow::new()),
            root: window.into(),
            data,
        }
    }

    pub fn run(&mut self) {
        let mut f = self.system_window.fabric().pixmap_mut();
        let width = f.width() as _;
        let height = f.height() as _;
        let mut draw_ctx = DrawCtx::new(&mut f);
        draw_ctx.widget_size = self.root.layout(AreaBox {
            left: 0,
            right: width,
            top: 0,
            down: height,
        }).to_size_f32();
        self.root.draw(&mut draw_ctx, &self.data);
        // println!("Pixmap {:?}", self.system_window.fabric().pixmap_mut().data_mut());
        self.system_window.sync();
        loop {
            self.system_window.query_event(false);
        }
    }
}

pub struct WidgetPod<D = ()> {
    pub widget_id: WidgetUid,
    pub widget: Box<dyn Widget<D>>,
    widget_size: (f32, f32),
}

impl<D> WidgetPod<D> {
    pub fn uid(&self) -> WidgetUid {
        self.widget_id
    }
    pub fn event(&mut self, ctx: &mut EventCtx, data: &mut D) {
        self.widget.event(ctx, data);
    }
    pub fn update(&mut self, data: &D) {
        self.widget.update(data);
    }
    pub fn draw(&mut self, ctx: &mut DrawCtx, data: &D) {
        ctx.widget_size = self.widget_size.to_owned();
        self.widget.draw(ctx, data);
    }
    pub fn layout(&mut self, max_box: AreaBox) -> AreaBox {
        let b = self.widget.layout(max_box);
        self.widget_size = b.to_size_f32();
        b
    }
}

impl<D> From<Box<dyn Widget<D>>> for WidgetPod<D> {
    fn from(c: Box<dyn Widget<D>>) -> Self {
        Self {
            widget_id: gen_uid(),
            widget: c,
            widget_size: (f32::MAX, f32::MAX)
        }
    }
}
