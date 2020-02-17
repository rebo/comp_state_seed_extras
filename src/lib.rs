#![feature(track_caller)]

mod ev_handlers;
mod seed_bind;
mod utils;

pub use ev_handlers::StateAccessEventHandlers;
pub use seed_bind::{bind, UpdateElLocal};
pub use utils::{after_render, after_render_once, get_html_element_by_id, request_animation_frame};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
