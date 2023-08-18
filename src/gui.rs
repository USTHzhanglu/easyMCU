use iced::subscription;
use iced::widget::{button, combo_box, container, pick_list, text, Column, Row};
use iced::window;
use iced::Event;
use iced::{executor, Padding};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};

use std::fmt::Debug;
use std::ops::Not;

use rust_embed::RustEmbed;
#[cfg(target_os = "windows")]
use winapi::um::winnt;
#[cfg(target_os = "windows")]
use winapi::um::winuser;

#[derive(RustEmbed)]
#[folder = "./ico/"]
struct Asset;

#[derive(Debug)]
pub struct Gui;

#[derive(Debug, Clone)]
pub enum Message {
    Focused,
}

fn reset_sys_menu() -> bool {
    if cfg!(target_os = "windows") {
        let hwnd = unsafe {
            winuser::FindWindowA(
                "Window Class\0".as_ptr() as winnt::LPCSTR,
                "easyMCU\0".as_ptr() as winnt::LPCSTR,
            )
        };
        // unsafe {
        //     let mut lp_class_name: Vec<u8> = vec![0; 100];
        //     winuser::GetWindowTextA(
        //         hwnd,
        //         lp_class_name.as_mut_ptr() as winnt::LPSTR,
        //         100,
        //     );
        //     let str = String::from_utf8(lp_class_name).unwrap();
        //     println!("{}", str);
        // }
        if hwnd == std::ptr::null_mut() {
            return false;
        };
        let mut hwsytle: u32 =
            unsafe { winuser::GetWindowLongA(hwnd, winuser::GWL_STYLE.try_into().unwrap()) } as u32;
        unsafe {
            hwsytle &= (winuser::WS_MAXIMIZEBOX).not();
            winuser::SetWindowLongA(hwnd, winuser::GWL_STYLE, hwsytle as i32);
        };
    };
    return true;
}

impl Gui {
    pub fn start() -> iced::Result {
        let icon = window::icon::from_file_data(
            &Asset::get("icon.ico").unwrap().data,
            Some(image::ImageFormat::Ico),
        )
        .unwrap();

        let window_settings = window::Settings {
            size: (800, 480), //logic pixel
            position: iced::window::Position::Centered,
            resizable: false,
            //icon: Some(window::icon::from_file("./ico/icon.ico").unwrap()),
            icon: Some(icon),
            ..window::Settings::default()
        };

        let settings = Settings {
            window: window_settings,
            id: Some("easyMCU".try_into().unwrap()),
            ..Default::default()
        };

        Gui::run(settings)
    }
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Message>) {
        let ret = (Self {}, { Command::none() });

        return ret;
    }

    fn title(&self) -> String {
        String::from("easyMCU")
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Focused => {
                //del system maximize button when open whindow
                static mut HAVE_DEL_MENU: bool = false;
                unsafe {
                    if HAVE_DEL_MENU == false {
                        HAVE_DEL_MENU = reset_sys_menu();
                    }
                };
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        // let probe_list = pick_list(
        //     &self.probe_list,
        //     Some({
        //         match self.probe_selected {
        //             Some(selected) => self.probe_list.get(selected).unwrap().to_string(),
        //             None => "No probe found".to_string(),
        //         }
        //     }),
        //     Message::ProbeSelected,
        // )
        // .width(150);

        // let probe_list_btn = button(text("Ref"))
        //     .width(50)
        //     .on_press(Message::ProbeRefresh);

        // let probe_box = Row::new()
        //     .width(200)
        //     .align_items(Alignment::Center)
        //     .spacing(0)
        //     .push(probe_list)
        //     .push(probe_list_btn);

        // let target_list = combo_box(
        //     &self.target_list,
        //     "Select a MCU...",
        //     self.target_selected.as_ref(),
        //     Message::TargetSelected,
        // )
        // .width(200);

        // let target_box = Column::new()
        //     .width(240)
        //     .align_items(Alignment::Center)
        //     .spacing(20)
        //     .push(probe_box)
        //     .push(target_list);

        // let log = text(self.log_text.clone()).width(200);

        // let file_path = text(match self.file_path.clone() {
        //     None => "No file selected".to_string(),
        //     Some(path) => "Now Loaded: ".to_string() + path.as_str(),
        // })
        // .width(200);

        // let path_box = Column::new()
        //     .width(240)
        //     .align_items(Alignment::Center)
        //     .spacing(20)
        //     .push(log)
        //     .push(file_path);

        // let erase_btn = button(text("Erase"))
        //     .width(200)
        //     .height(50)
        //     .padding(Padding::from([13, 80]))
        //     .on_press(Message::Erase);

        // let flash_btn = button(text("Flash"))
        //     .width(200)
        //     .height(50)
        //     .padding(Padding::from([13, 80]))
        //     .on_press(Message::Flash);

        // let btn_box = Column::new()
        //     .width(240)
        //     .align_items(Alignment::Center)
        //     .spacing(20)
        //     .push(erase_btn)
        //     .push(flash_btn);

        let content = Row::new().align_items(Alignment::Center).spacing(0);
        // .push(target_box)
        // .push(path_box)
        // .push(btn_box);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _| match event {
            // Event::Window(window::Event::FileDropped(path)) => Some(Message::FileSelected(path)),
            Event::Window(window::Event::Focused) => Some(Message::Focused),
            _ => None,
        })
    }
}
