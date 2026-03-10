use cosmic::{
    app::{self, Core},
    iced::{Alignment,  Length, Subscription},
    iced_widget::{row, text},
    Element,
};
use std::time::Duration;
use sysinfo::System;
use std::fs;

fn get_ram_usage() -> String {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();

    let mut total = 0;
    let mut available = 0;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal") {
            total = line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
        }

        if line.starts_with("MemAvailable") {
            available = line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
        }
    }

    let used = total - available;

    let used_gb = used as f64 / 1024.0 / 1024.0;
    let total_gb = total as f64 / 1024.0 / 1024.0;

    format!("{:.1}/{:.1}", used_gb, total_gb)
}
fn read_temperature() -> Option<i32> {
    if let Ok(temp) = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(val) = temp.trim().parse::<i32>() {
            return Some(val / 1000);
        }
    }

    None
}
pub struct Window {
    core: Core,
    ram: String,
    sys: System,
    cpu_usage: f32,
    temperature: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl cosmic::Application for Window {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();

    const APP_ID: &str = "com.system76.CosmicAppletRamMonitor";

    fn init(core: Core, _flags: Self::Flags) -> (Self, app::Task<Self::Message>) {
        let mut sys = System::new_all();
        sys.refresh_all();
        (
            Self {
                core,
                ram: get_ram_usage(),
                sys,
                cpu_usage: 0.0,
                temperature: 0,
            },
            app::Task::none(),
        )
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
    cosmic::iced::time::every(Duration::from_secs(2))
        .map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> app::Task<Message> {
        match message {
            Message::Tick => {
                self.sys.refresh_cpu();
                self.cpu_usage = self.sys.global_cpu_info().cpu_usage();
                if let Some(t) = read_temperature() {
                    self.temperature = t;
                }
                self.ram = get_ram_usage();
            }
        }

        app::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let info = format!(
        "CPU {:.0}% {} {}°C",
        self.cpu_usage,
        self.ram,
        self.temperature
    );

    row![text(info)]
        .align_y(Alignment::Center)
        .width(Length::Shrink)
        .into()
    }
}