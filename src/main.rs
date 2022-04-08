use cli_clipboard;
use std::net::TcpStream;
use std::time::Duration;
slint::include_modules!();

fn get_ip() -> Option<String> {
    if let Ok(stream) =
        TcpStream::connect_timeout(&"114.114.114.114:80".parse().unwrap(), Duration::new(2, 0))
    {
        if let Ok(local_addr) = stream.local_addr() {
            return Some(format!("{:?}", local_addr.ip()));
        }
    }
    return None;
}

fn main() {
    let ui = AppWindow::new();
    ui.set_net_ip(get_ip().unwrap_or("failed".to_string()).into());
    let ui_handle = ui.as_weak();
    ui.on_copy_me(move |name, x| {
        let ui = ui_handle.unwrap();
        cli_clipboard::set_contents(x.to_string()).unwrap();
        ui.set_tips(format!("{} copied!", name).into());
    });

    ui.run();
}
