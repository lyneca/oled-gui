use piscreen::{View, ButtonSet, Display, ReturnState, ReturnStateEnum::*};
use piscreen::views::{MenuView, MenuEntry, TextView, FileView, TextInputView};
use piscreen::{menu_view, text_view, file_view};
use network_manager::{NetworkManager, Device, AccessPoint};

pub struct WifiView {
    menu: MenuView,
    device: Device,
    aps: Vec<AccessPoint>
}

impl WifiView {
    pub fn new() -> WifiView {
        WifiView {
            menu: MenuView::new(),
            device: NetworkManager::new()
                .get_device_by_interface("wlan0").unwrap(),
            aps: Vec::new()
        }
    }

    pub fn scan(&mut self) {
        if let Some(wifi) = self.device.as_wifi_device() {
            if let Ok(aps) = wifi.get_access_points() {
                self.aps = aps;
            }
        }
    }
}

impl View for WifiView {
    fn activate(&mut self) {
        self.scan();
        self.menu.set_entries(self.aps.iter().map(|ap: &AccessPoint| -> MenuEntry {
            (ap.ssid().as_str().unwrap().to_owned(), Box::new(text_view!(ap.ssid().as_str().unwrap())))
        }).collect())
    }

    fn render(&mut self, disp: &mut Display) {
        self.menu.render(disp)
    }

    fn handle_buttons(&mut self, buttons: &mut ButtonSet) -> ReturnState {
        self.menu.handle_buttons(buttons)
    }
}
