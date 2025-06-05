mod wayland;

fn main() {
    env_logger::init();
    wayland::start();
}
