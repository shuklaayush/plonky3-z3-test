use std::sync::OnceLock;
use z3::{Config, Context};

struct SafeContext(Context);
unsafe impl Send for SafeContext {}
unsafe impl Sync for SafeContext {}

pub(crate) fn context() -> &'static Context {
    static CONTEXT: OnceLock<SafeContext> = OnceLock::new();
    &CONTEXT
        .get_or_init(|| SafeContext(Context::new(&Config::new())))
        .0
}
