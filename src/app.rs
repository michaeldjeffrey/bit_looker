use crate::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    bit_looker: bit_looker::State,
    net_id: net_id::State,
    santa: santa::State,

    show_bit_looker: bool,
    show_net_id: bool,
    show_santa: bool,

    styles: MyStyles,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MyStyles {
    pub button_spc_x: f32,
    pub button_spc_y: f32,
    pub button_round: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            bit_looker: Default::default(),
            net_id: Default::default(),
            santa: Default::default(),
            show_bit_looker: Default::default(),
            show_net_id: Default::default(),
            show_santa: Default::default(),
            styles: MyStyles {
                button_spc_x: 15.0,
                button_spc_y: 10.0,
                button_round: 8.0,
            },
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            bit_looker,
            show_bit_looker,
            net_id,
            show_net_id,
            santa,
            show_santa,
            styles,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("apps").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(show_bit_looker, "Bit Looker");
                ui.toggle_value(show_net_id, "Net ID");
                ui.toggle_value(show_santa, "Santa");
            });
        });
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("SHowing");
            ui.label(format!("bit_looker: {show_bit_looker}"));
            ui.label(format!("net_id: {show_net_id}"));
            ui.label(format!("santa: {show_santa}"));

            bit_looker.side_panel(ui);
            net_id.side_panel(ui);
            santa.side_panel(ui);
            ui.separator();
            egui::widgets::global_dark_light_mode_buttons(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if *show_bit_looker {
                ui.style_mut().spacing.button_padding.x = styles.button_spc_x;
                ui.style_mut().spacing.button_padding.y = styles.button_spc_y;
                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                bit_looker.main_view(ui, styles);
                ui.reset_style();
                ui.separator();
            }

            if *show_net_id {
                net_id.main_view(ui, styles);
                ui.separator();
            }

            if *show_santa {
                santa.main_view(ui, styles);
                ui.separator();
            }

            if !(*show_bit_looker || *show_net_id || *show_santa) {
                ui.style_mut().spacing.button_padding.x = styles.button_spc_x;
                ui.style_mut().spacing.button_padding.y = styles.button_spc_y;
                ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

                ui.horizontal(|ui| {
                    if ui.button("Bit Looker").clicked() {
                        *show_bit_looker = true;
                    }
                    ui.label("Visualize Binary Numbers");
                });

                ui.horizontal(|ui| {
                    if ui.button("Net ID").clicked() {
                        *show_net_id = true;
                    }
                    ui.label("Visualize LoRaWAN Net IDs");
                });

                ui.horizontal(|ui| {
                    if ui.button("Santa").clicked() {
                        *show_santa = true;
                    }
                    ui.label("Secret Santa matcher");
                });
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
