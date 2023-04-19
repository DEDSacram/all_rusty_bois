#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;


// api
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use poll_promise::Promise;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CipherRes{
    skip : u32,
    offset : u32,
    text: String,
    token: String
}

fn main() -> Result<(), eframe::Error> {



    
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}


struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    tx: Sender<u32>,
    rx: Receiver<u32>,
}

// async fn getfromapi(url : &str)-> CipherRes{
//     let res = reqwest::get(url).await.unwrap();
//     let json = res.json::<CipherRes>().await.unwrap();
//     println!("{:?}",json);
//     json

// }

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            dropped_files: Vec::new(),
            picked_path: None,
        }
    }
}

fn sendreq( tx: Sender<u32>){
    std::thread::spawn(move || {
        tx.send(25)
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        println!("{:?}",self.rx.try_recv());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");

            if ui.button("Get From url").clicked() {
                // if let Some(path) = rfd::FileDialog::new().pick_file() {
                //     self.picked_path = Some(path.display().to_string());
                // }

                // let promise = self.promise.get_or_insert_with(|| {
                //     // Begin download.
                //     // We download the image using `ehttp`, a library that works both in WASM and on native.
                //     // We use the `poll-promise` library to communicate with the UI thread.
                //     let ctx = ctx.clone();
                    // let (sender, promise) = Promise::new();
                    // let request = ehttp::Request::get("https://picsum.photos/seed/1.759706314/1024");
                    // ehttp::fetch(request, move |response| {
                    //     let image = response.and_then(parse_response);
                    //     sender.send(image); // send the results back to the UI thread.
                    //     ctx.request_repaint(); // wake up UI thread
                    // });
                //     promise
                // });

                
     

   
                
           
        
                sendreq(self.tx.clone())
      
  
    
    

           
                
            }
            

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            use std::fmt::Write as _;
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files = i.raw.dropped_files.clone();
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn parse_response(response: ehttp::Response) -> Result<CipherRes, String> {
    let content_type = response.content_type().unwrap_or_default();
    // if content_type.starts_with("image/") {
    //     CipherRes::from_image_bytes(&response.url, &response.bytes)
    // } else {
    //     Err(format!(
    //         "Expected image, found content-type {:?}",
    //         content_type
    //     ))
    // }
    let text = response.text().unwrap();
    if(!text.is_empty()){
        return Err("Yo Fucked up".to_owned())
    }
    let obj : CipherRes =serde_json::from_str(text).unwrap();
  
    return Ok(obj);

}