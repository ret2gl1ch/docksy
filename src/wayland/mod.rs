use wayland_client::{
    protocol::{wl_compositor::WlCompositor, wl_surface::WlSurface},
    Connection, Dispatch, QueueHandle, WEnum,
};

use wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase;

struct AppState {
    compositor: Option<WlCompositor>,
    surface: Option<WlSurface>,
}

impl Dispatch<WlCompositor, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlCompositor,
        _event: <WlCompositor as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        
    }
}

impl Dispatch<WlSurface, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSurface,
        _event: <WlSurface as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    
    }
}

impl Dispatch<XdgWmBase, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &XdgWmBase,
        _event: <XdgWmBase as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        
    }
}

pub fn start() {
    // Connect to the Wayland display
    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland");
    let display = conn.display();

    // Create an event queue
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    // Create initial state
    let mut state = AppState {
        compositor: None,
        surface: None,
    };

    println!("✅ Wayland connected!");




    event_queue.blocking_dispatch(&mut state).expect("Failed to dispatch");
}