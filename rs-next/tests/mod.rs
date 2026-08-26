use std::sync::atomic::AtomicUsize;

static ON_SUCCESS_COUNTER: AtomicUsize = AtomicUsize::new(0);
static ON_FAILURE_COUNTER: AtomicUsize = AtomicUsize::new(0);

mod field_configs;
mod smoke;
