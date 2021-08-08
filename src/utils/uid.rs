//! 独一标识符：确保每次新建控件时都会得到不一样的标识符

use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_uid() -> usize {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Acquire)
}
