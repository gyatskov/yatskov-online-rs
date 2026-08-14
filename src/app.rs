use egui::{Color32, Visuals};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct YatskovOnlineApp {
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for YatskovOnlineApp {
    fn default() -> Self {
        Self {
            label: "Yatskov Online".to_owned(),
            value: 2.7,
        }
    }
}

impl YatskovOnlineApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for YatskovOnlineApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Theme customization
        // ui.visuals_mut().widgets.noninteractive = WidgetVisuals {
        //     weak_bg_fill: Color32::from_rgb(0x30, 0x30, 0x30),
        //     bg_fill: Color32::from_rgb(0x40, 0x40, 0x40),
        //     expansion: 0.0,
        //     ..ui.visuals().widgets.noninteractive
        // };
        // ui.visuals_mut().widgets.inactive = WidgetVisuals {
        //     weak_bg_fill: Color32::from_rgb(0x30, 0x30, 0x30),
        //     bg_fill: Color32::from_rgb(0x40, 0x40, 0x40),
        //     expansion: 0.0,
        //     ..ui.visuals().widgets.noninteractive
        // };
        // ui.visuals_mut().widgets.hovered = WidgetVisuals {
        //     weak_bg_fill: Color32::from_rgb(0x30, 0x30, 0x30),
        //     bg_fill: Color32::from_rgb(0x40, 0x40, 0x40),
        //     expansion: 0.0,
        //     ..ui.visuals().widgets.noninteractive
        // };
        // ui.visuals_mut().widgets.active = WidgetVisuals {
        //     weak_bg_fill: Color32::from_rgb(0x30, 0x30, 0x30),
        //     bg_fill: Color32::from_rgb(0x40, 0x40, 0x40),
        //     expansion: 0.0,
        //     ..ui.visuals().widgets.noninteractive
        // };
        // ui.visuals_mut().widgets.open = WidgetVisuals {
        //     weak_bg_fill: Color32::from_rgb(0x30, 0x30, 0x30),
        //     bg_fill: Color32::from_rgb(0x40, 0x40, 0x40),
        //     expansion: 0.0,
        //     ..ui.visuals().widgets.noninteractive
        // };
        // TODO: Extract to struct
        ui.set_visuals_of(egui::Theme::Dark, Visuals {
            panel_fill: Color32::from_rgb(0x40, 0x40, 0x40),
            window_fill: Color32::from_rgb(0x30, 0x30, 0x30),
            ..Default::default()
        });

        egui::Panel::top("top_panel")
            .resizable(false)
            .show(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Yatskov Online");
            ui.add_space(4.0);

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
