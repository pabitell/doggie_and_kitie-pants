#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

use common::webapp;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator = LockedAllocator::new(FreeListAllocator::new());

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    yew::start_app::<webapp::root::Root>();
}
