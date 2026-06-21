use crate::collector_service::{self, CollectorEvent, ServiceConfig};
use crate::windows_config::AppConfig;
use anyhow::{Context, Result};
use native_windows_gui as nwg;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use telemetry_core::Sim;
use tokio::runtime::Runtime;
use tokio::sync::watch;

const APP_ICON: &[u8] = include_bytes!("../assets/icon.ico");

pub struct LaunchOverrides {
    pub server: Option<String>,
    pub token: Option<String>,
    pub show_window: bool,
}

struct App {
    window: nwg::Window,
    _shell: nwg::MessageWindow,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    tray_open_item: nwg::MenuItem,
    tray_start_item: nwg::MenuItem,
    tray_stop_item: nwg::MenuItem,
    tray_quit_item: nwg::MenuItem,
    notice: nwg::Notice,
    server_label: nwg::Label,
    server_input: nwg::TextInput,
    token_label: nwg::Label,
    token_input: nwg::TextInput,
    sim_label: nwg::Label,
    sim_value: nwg::Label,
    status_label: nwg::Label,
    save_button: nwg::Button,
    start_button: nwg::Button,
    stop_button: nwg::Button,
    icon: nwg::Icon,
    ui_font: nwg::Font,
    label_font: nwg::Font,
    status_font: nwg::Font,
    event_handler: RefCell<Option<nwg::EventHandler>>,
    worker_stop: RefCell<Option<watch::Sender<bool>>>,
    event_rx: RefCell<Option<Receiver<CollectorEvent>>>,
    running: RefCell<bool>,
}

pub fn run(overrides: LaunchOverrides) -> Result<()> {
    nwg::init().context("failed to initialize native-windows-gui")?;
    nwg::Font::set_global_family("Segoe UI").context("failed to set default font")?;

    let mut config = AppConfig::load().unwrap_or_default();
    if let Some(server) = overrides.server {
        config.server = server;
    }
    if let Some(token) = overrides.token {
        config.token = token;
    }

    let config_ready = config.is_ready();
    let app = App::build(config)?;
    let should_show = overrides.show_window || !config_ready;

    if should_show {
        app.show_window();
    }

    if config_ready {
        app.start_collector();
    } else {
        app.set_status("Enter the server endpoint and token, then click Save or Start.");
    }

    nwg::dispatch_thread_events();
    Ok(())
}

impl App {
    fn build(config: AppConfig) -> Result<Rc<Self>> {
        let mut window = nwg::Window::default();
        let mut shell = nwg::MessageWindow::default();
        let mut tray = nwg::TrayNotification::default();
        let mut tray_menu = nwg::Menu::default();
        let mut tray_open_item = nwg::MenuItem::default();
        let mut tray_start_item = nwg::MenuItem::default();
        let mut tray_stop_item = nwg::MenuItem::default();
        let mut tray_quit_item = nwg::MenuItem::default();
        let mut notice = nwg::Notice::default();
        let mut server_label = nwg::Label::default();
        let mut server_input = nwg::TextInput::default();
        let mut token_label = nwg::Label::default();
        let mut token_input = nwg::TextInput::default();
        let mut sim_label = nwg::Label::default();
        let mut sim_value = nwg::Label::default();
        let mut status_label = nwg::Label::default();
        let mut save_button = nwg::Button::default();
        let mut start_button = nwg::Button::default();
        let mut stop_button = nwg::Button::default();
        let mut ui_font = nwg::Font::default();
        let mut label_font = nwg::Font::default();
        let mut status_font = nwg::Font::default();

        let mut icon = nwg::Icon::default();
        nwg::Icon::builder()
            .source_bin(Some(APP_ICON))
            .build(&mut icon)
            .context("failed to load app icon")?;

        nwg::Font::builder()
            .family("Segoe UI")
            .size_absolute(16)
            .build(&mut ui_font)
            .context("failed to build UI font")?;

        nwg::Font::builder()
            .family("Segoe UI")
            .size_absolute(14)
            .weight(600)
            .build(&mut label_font)
            .context("failed to build label font")?;

        nwg::Font::builder()
            .family("Segoe UI")
            .size_absolute(14)
            .build(&mut status_font)
            .context("failed to build status font")?;

        nwg::Window::builder()
            .size((520, 300))
            .title("Race Agent Collector")
            .flags(nwg::WindowFlags::WINDOW)
            .icon(Some(&icon))
            .center(true)
            .build(&mut window)
            .context("failed to build window")?;

        nwg::MessageWindow::builder()
            .build(&mut shell)
            .context("failed to build shell window")?;

        nwg::Menu::builder()
            .popup(true)
            .parent(&window)
            .build(&mut tray_menu)
            .context("failed to build tray menu")?;

        nwg::MenuItem::builder()
            .text("Open")
            .parent(&tray_menu)
            .build(&mut tray_open_item)
            .context("failed to build tray open item")?;

        nwg::MenuItem::builder()
            .text("Start")
            .parent(&tray_menu)
            .build(&mut tray_start_item)
            .context("failed to build tray start item")?;

        nwg::MenuItem::builder()
            .text("Stop")
            .parent(&tray_menu)
            .build(&mut tray_stop_item)
            .context("failed to build tray stop item")?;
        tray_stop_item.set_enabled(false);

        nwg::MenuItem::builder()
            .text("Quit")
            .parent(&tray_menu)
            .build(&mut tray_quit_item)
            .context("failed to build tray quit item")?;

        nwg::TrayNotification::builder()
            .parent(&shell)
            .icon(Some(&icon))
            .tip(Some("Race Agent Collector"))
            .build(&mut tray)
            .context("failed to build tray icon")?;

        nwg::Notice::builder()
            .parent(&window)
            .build(&mut notice)
            .context("failed to build worker notice")?;

        nwg::Label::builder()
            .text("SERVER")
            .position((28, 34))
            .size((104, 24))
            .font(Some(&label_font))
            .h_align(nwg::HTextAlign::Right)
            .v_align(nwg::VTextAlign::Center)
            .parent(&window)
            .build(&mut server_label)
            .context("failed to build server label")?;

        nwg::TextInput::builder()
            .text(&config.server)
            .placeholder_text(Some("http://localhost:3000"))
            .position((148, 28))
            .size((330, 34))
            .font(Some(&ui_font))
            .parent(&window)
            .build(&mut server_input)
            .context("failed to build server input")?;

        nwg::Label::builder()
            .text("TOKEN")
            .position((28, 84))
            .size((104, 24))
            .font(Some(&label_font))
            .h_align(nwg::HTextAlign::Right)
            .v_align(nwg::VTextAlign::Center)
            .parent(&window)
            .build(&mut token_label)
            .context("failed to build token label")?;

        nwg::TextInput::builder()
            .text(&config.token)
            .placeholder_text(Some("Paste ingest token"))
            .position((148, 78))
            .size((330, 34))
            .password(Some('*'))
            .font(Some(&ui_font))
            .parent(&window)
            .build(&mut token_input)
            .context("failed to build token input")?;

        nwg::Label::builder()
            .text("SIMULATOR")
            .position((28, 134))
            .size((104, 24))
            .font(Some(&label_font))
            .h_align(nwg::HTextAlign::Right)
            .v_align(nwg::VTextAlign::Center)
            .parent(&window)
            .build(&mut sim_label)
            .context("failed to build sim label")?;

        nwg::Label::builder()
            .text("ACC")
            .position((148, 130))
            .size((330, 32))
            .font(Some(&ui_font))
            .v_align(nwg::VTextAlign::Center)
            .parent(&window)
            .build(&mut sim_value)
            .context("failed to build sim value")?;

        nwg::Label::builder()
            .text("Idle")
            .position((28, 184))
            .size((450, 42))
            .font(Some(&status_font))
            .v_align(nwg::VTextAlign::Center)
            .parent(&window)
            .build(&mut status_label)
            .context("failed to build status label")?;

        nwg::Button::builder()
            .text("Save")
            .position((148, 242))
            .size((96, 34))
            .font(Some(&ui_font))
            .parent(&window)
            .build(&mut save_button)
            .context("failed to build save button")?;

        nwg::Button::builder()
            .text("Start")
            .position((262, 242))
            .size((104, 34))
            .font(Some(&ui_font))
            .focus(true)
            .parent(&window)
            .build(&mut start_button)
            .context("failed to build start button")?;

        nwg::Button::builder()
            .text("Stop")
            .position((384, 242))
            .size((94, 34))
            .enabled(false)
            .font(Some(&ui_font))
            .parent(&window)
            .build(&mut stop_button)
            .context("failed to build stop button")?;

        window.set_visible(false);

        let app = Rc::new(Self {
            window,
            _shell: shell,
            tray,
            tray_menu,
            tray_open_item,
            tray_start_item,
            tray_stop_item,
            tray_quit_item,
            notice,
            server_label,
            server_input,
            token_label,
            token_input,
            sim_label,
            sim_value,
            status_label,
            save_button,
            start_button,
            stop_button,
            icon,
            ui_font,
            label_font,
            status_font,
            event_handler: RefCell::new(None),
            worker_stop: RefCell::new(None),
            event_rx: RefCell::new(None),
            running: RefCell::new(false),
        });

        app.update_controls();
        app.bind_events();
        Ok(app)
    }

    fn bind_events(self: &Rc<Self>) {
        let app = Rc::clone(self);

        let handler = nwg::full_bind_event_handler(&self.window.handle, move |event, _, handle| {
            if handle == app.window.handle {
                match event {
                    nwg::Event::OnWindowClose => {
                        app.window.set_visible(false);
                    }
                    nwg::Event::OnNotice => {
                        app.drain_worker_events();
                    }
                    _ => {}
                }
            }

            if handle == app.save_button.handle && event == nwg::Event::OnButtonClick {
                app.save_config();
            }

            if handle == app.start_button.handle && event == nwg::Event::OnButtonClick {
                app.start_collector();
            }

            if handle == app.stop_button.handle && event == nwg::Event::OnButtonClick {
                app.stop_collector();
            }

            if handle == app.tray.handle && event == nwg::Event::OnContextMenu {
                let (_, y) = nwg::GlobalCursor::position();
                let (x, _) = nwg::GlobalCursor::position();
                app.tray_menu.popup(x, y);
            }

            if handle == app.tray_open_item.handle && event == nwg::Event::OnMenuItemSelected {
                app.show_window();
            }

            if handle == app.tray_start_item.handle && event == nwg::Event::OnMenuItemSelected {
                app.start_collector();
            }

            if handle == app.tray_stop_item.handle && event == nwg::Event::OnMenuItemSelected {
                app.stop_collector();
            }

            if handle == app.tray_quit_item.handle && event == nwg::Event::OnMenuItemSelected {
                app.stop_collector();
                nwg::stop_thread_dispatch();
            }
        });

        *self.event_handler.borrow_mut() = Some(handler);
    }

    fn show_window(&self) {
        self.window.set_visible(true);
        self.window.restore();
        self.window.set_focus();
    }

    fn save_config(&self) {
        let config = self.current_config();
        if !config.is_ready() {
            self.show_window();
            self.set_status("Server endpoint and token are required.");
            return;
        }

        match config.save() {
            Ok(()) => self.set_status("Configuration saved."),
            Err(error) => self.set_status(&format!("Failed to save configuration: {error}")),
        }
    }

    fn start_collector(&self) {
        if *self.running.borrow() {
            return;
        }

        let config = self.current_config();
        if !config.is_ready() {
            self.show_window();
            self.set_status("Server endpoint and token are required before starting.");
            return;
        }

        if let Err(error) = config.save() {
            self.set_status(&format!("Failed to save configuration: {error}"));
            return;
        }

        let (event_tx, event_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = watch::channel(false);
        let notice = self.notice.sender();

        *self.worker_stop.borrow_mut() = Some(stop_tx);
        *self.event_rx.borrow_mut() = Some(event_rx);
        *self.running.borrow_mut() = true;
        self.update_controls();
        self.set_status("Starting collector...");

        thread::spawn(move || {
            let runtime = match Runtime::new() {
                Ok(runtime) => runtime,
                Err(error) => {
                    let _ = event_tx.send(CollectorEvent::Status(format!(
                        "Failed to start runtime: {error}"
                    )));
                    let _ = event_tx.send(CollectorEvent::Stopped);
                    notice.notice();
                    return;
                }
            };

            let service_config = ServiceConfig {
                server: config.server,
                token: config.token,
                sim: config.sim,
            };

            let result = runtime.block_on(collector_service::run(
                service_config,
                event_tx.clone(),
                stop_rx,
            ));

            if let Err(error) = result {
                let _ = event_tx.send(CollectorEvent::Status(format!(
                    "Collector stopped: {error}"
                )));
            }

            let _ = event_tx.send(CollectorEvent::Stopped);
            notice.notice();
        });
    }

    fn stop_collector(&self) {
        if let Some(stop_tx) = self.worker_stop.borrow_mut().take() {
            let _ = stop_tx.send(true);
            self.set_status("Stopping collector...");
        }
        self.update_controls();
    }

    fn drain_worker_events(&self) {
        let mut stop_seen = false;

        if let Some(rx) = self.event_rx.borrow_mut().as_mut() {
            while let Ok(event) = rx.try_recv() {
                match event {
                    CollectorEvent::Status(message) => self.set_status(&message),
                    CollectorEvent::Running => {
                        *self.running.borrow_mut() = true;
                        self.update_controls();
                    }
                    CollectorEvent::Stopped => {
                        stop_seen = true;
                    }
                }
            }
        }

        if stop_seen {
            *self.running.borrow_mut() = false;
            self.worker_stop.borrow_mut().take();
            self.event_rx.borrow_mut().take();
            self.update_controls();
        }
    }

    fn set_status(&self, message: &str) {
        self.status_label.set_text(message);
        self.tray
            .set_tip(&format!("Race Agent Collector - {message}"));
    }

    fn update_controls(&self) {
        let running = *self.running.borrow();

        self.start_button.set_enabled(!running);
        self.stop_button.set_enabled(running);
        self.tray_start_item.set_enabled(!running);
        self.tray_stop_item.set_enabled(running);
        self.server_input.set_enabled(!running);
        self.token_input.set_enabled(!running);
        self.save_button.set_enabled(!running);
    }

    fn current_config(&self) -> AppConfig {
        AppConfig {
            server: self.server_input.text(),
            token: self.token_input.text(),
            sim: Sim::Acc,
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(handler) = self.event_handler.borrow_mut().take() {
            nwg::unbind_event_handler(&handler);
        }

        let _ = self
            .worker_stop
            .borrow_mut()
            .take()
            .map(|stop_tx| stop_tx.send(true));
        let _ = &self.server_label;
        let _ = &self.token_label;
        let _ = &self.sim_label;
        let _ = &self.sim_value;
        let _ = &self.icon;
        let _ = &self.ui_font;
        let _ = &self.label_font;
        let _ = &self.status_font;
    }
}
