//! 一些统一接口在不同系统上的实现，目前仅 windows

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

pub mod enums {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum HitResult {
        None,
        Client,
        Caption,
        MinButton,
        Close,
    }
}

/// 不同系统模块都需要实现的 trait
pub mod traits {
    use crate::widgets::WindowEvent;
    use crate::{traits::TopControl, widgets::UserEvent};

    use super::enums::HitResult;

    /// 系统窗口，提供了可绘制的画布以进行绘图
    pub trait SystemDrawableWindow<'a> {
        /// 创建一个窗口
        fn new(top_control: &'a mut dyn TopControl) -> Self;
        /// 获取窗口原始句柄，用于和画板进行对接
        fn raw_handle(&self) -> usize;
        /// 赋予一个顶级可绘制的控件
        fn set_top_control(&mut self, top_control: &'a mut dyn TopControl);
        /// 立即改变系统窗口 & 画布的大小
        fn resize(&mut self, width: u32, height: u32);
        /// 获取窗口左上角的横坐标
        fn pos_x(&self) -> i32;
        /// 设置窗口左上角的横坐标
        fn set_pos_x(&mut self, x: i32);
        /// 获取窗口左上角的纵坐标
        fn pos_y(&self) -> i32;
        /// 设置窗口左上角的纵坐标
        fn set_pos_y(&mut self, y: i32);
        /// 获取窗口的宽度
        fn size_x(&self) -> u32;
        /// 设置窗口的宽度
        fn set_size_x(&mut self, w: u32);
        /// 获取窗口的高度
        fn size_y(&self) -> u32;
        /// 设置窗口的高度
        fn set_size_y(&mut self, h: u32);
        /// 进行一次系统事件轮询，返回统一的窗口事件
        ///
        /// 第一个参数为是否不阻塞请求消息，否则除非事件队列内仍有事件，该函数将会阻塞直到有事件发生
        fn query_system_event(&mut self, peek: bool) -> WindowEvent;
        /// 进行一次用户控件事件轮询，返回简化的用户控件事件
        ///
        /// 第一个参数为是否不阻塞请求消息，否则除非事件队列内仍有事件，该函数将会阻塞直到有事件发生
        fn query_event(&mut self, peek: bool) -> UserEvent;
        /// 执行点击测试，确认该位置属于什么控件，如关闭按钮，客户区域等
        fn hit_test(&mut self, x: i32, y: i32) -> HitResult;
        /// 将画板的内容同步到系统窗口上
        fn sync(&mut self);
        /// 显示窗口
        fn show(&mut self);
        /// 隐藏窗口
        fn hide(&mut self);
    }
}
