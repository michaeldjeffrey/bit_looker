/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    bits: Vec<bool>,
    new_bit: bool,

    styles: MyStyles,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MyStyles {
    button_spc_x: f32,
    button_spc_y: f32,
    button_round: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            bits: vec![false, false, false, false, false, false, false, false],
            new_bit: false,
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
            bits,
            new_bit,
            styles,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Left Bits");
                ui.horizontal(|ui| {
                    add_button(ui, "+ 1", || bits.push_left(true));
                    add_button(ui, "+ 0", || bits.push_left(false));
                    add_button(ui, "- x", || bits.pop_left());
                });
            });
            ui.separator();

            ui.vertical(|ui| {
                ui.label("Right Bits");
                ui.horizontal(|ui| {
                    add_button(ui, "+ 1", || bits.push_right(true));
                    add_button(ui, "+ 0", || bits.push_right(false));
                    add_button(ui, "- x", || bits.pop_right());
                });
            });
            ui.separator();

            ui.vertical(|ui| {
                ui.label("Num Bits");
                ui.horizontal(|ui| {
                    add_button(ui, "8", || bits.empty_and_set(8));
                    add_button(ui, "16", || bits.empty_and_set(16));
                    add_button(ui, "32", || bits.empty_and_set(32));
                });
            });
            ui.separator();

            ui.vertical(|ui| {
                ui.label("Reset");
                ui.horizontal(|ui| {
                    add_button(ui, "All 0", || bits.empty_and_set_with(bits.len(), false));
                    add_button(ui, "All 1", || bits.empty_and_set_with(bits.len(), true));
                });
            });
            ui.separator();

            egui::widgets::global_dark_light_mode_buttons(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.style_mut().spacing.button_padding.x = styles.button_spc_x;
            ui.style_mut().spacing.button_padding.y = styles.button_spc_y;
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

            let mut maybe_bits = bits.clone();
            let bit_count = bits.len();
            let chunk_count = bit_count / 8;

            for (chunk_idx, chunk) in bits.chunks_mut(8).enumerate() {
                ui.horizontal(|ui| {
                    let end = (chunk_count - chunk_idx) * 8;
                    let start = end - 7;
                    let range = (start..=end).rev();

                    for (bit, idx) in chunk.iter_mut().zip(range) {
                        add_bit(
                            ui,
                            idx,
                            *bit,
                            styles.button_round,
                            || bit.flip(),
                            || maybe_bits[bit_count - idx] = !maybe_bits[bit_count - idx],
                        );
                    }
                });

                ui.separator();
            }

            let num = bits_as_num(bits) as isize;
            let maybe_num = bits_as_num(&maybe_bits) as isize;

            ui.horizontal(|ui| {
                add_button(ui, "<<", || bits.shift_left(*new_bit));
                add_button(ui, ">>", || bits.shift_right(*new_bit));
                add_button(ui, new_bit.clone().bit_display(), || new_bit.flip());
            });

            ui.heading(format!("{num}"));
            if num != maybe_num {
                ui.heading(format!("maybe {maybe_num} :: diff {}", maybe_num - num));
            }

            egui::warn_if_debug_build(ui);
        });
    }
}

fn add_bit(
    ui: &mut egui::Ui,
    idx: usize,
    bit: bool,
    rounding: f32,
    mut on_click_block: impl FnMut(),
    mut on_hover_block: impl FnMut(),
) {
    ui.vertical(|ui| {
        // get the position power from 1-index
        let mut pow = idx.try_into().unwrap();
        pow -= 1;

        ui.label(format!("{idx}"));
        ui.small(format!("{}", usize::pow(2, pow)));

        let button = if bit {
            egui::Button::new(bit.bit_display())
                .rounding(rounding)
                .fill(if ui.visuals().dark_mode {
                    egui::Color32::DARK_GREEN
                } else {
                    egui::Color32::GREEN
                })
        } else {
            egui::Button::new(bit.bit_display()).rounding(rounding)
        };

        let b = ui.add(button);
        if b.clicked() {
            on_click_block();
        }
        if b.hovered() {
            on_hover_block();
        }
    });
}

fn add_button(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>, mut block: impl FnMut()) {
    if ui.button(text).clicked() {
        block();
    }
}

trait Shiftable {
    fn shift_left(&mut self, new_bit: bool);
    fn shift_right(&mut self, new_bit: bool);
    fn push_left(&mut self, new_bit: bool);
    fn push_right(&mut self, new_bit: bool);
    fn pop_left(&mut self);
    fn pop_right(&mut self);
    fn empty_and_set(&mut self, size: usize);
    fn empty_and_set_with(&mut self, size: usize, value: bool);
}

impl Shiftable for Vec<bool> {
    fn shift_left(&mut self, new_bit: bool) {
        self.reverse();
        self.pop();
        self.reverse();
        self.push(new_bit);
    }
    fn shift_right(&mut self, new_bit: bool) {
        self.pop();
        self.reverse();
        self.push(new_bit);
        self.reverse();
    }

    fn push_left(&mut self, new_bit: bool) {
        self.reverse();
        self.push(new_bit);
        self.reverse();
    }

    fn push_right(&mut self, new_bit: bool) {
        self.push(new_bit);
    }

    fn pop_left(&mut self) {
        self.reverse();
        self.pop();
        self.reverse();
    }

    fn pop_right(&mut self) {
        self.pop();
    }

    fn empty_and_set(&mut self, size: usize) {
        self.empty_and_set_with(size, false);
    }

    fn empty_and_set_with(&mut self, size: usize, value: bool) {
        self.clear();
        for _ in 0..size {
            self.push(value);
        }
    }
}

trait Bittable {
    fn bit_display(&self) -> &str;
    fn flip(&mut self);
}

impl Bittable for bool {
    fn bit_display(&self) -> &str {
        if *self {
            "1"
        } else {
            "0"
        }
    }

    fn flip(&mut self) {
        *self = !*self;
    }
}

fn bits_as_num(bits: &[bool]) -> usize {
    let mut result = 0;
    for bit in bits.iter() {
        let val = if *bit { 1 } else { 0 };
        result <<= 1;
        result ^= val;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_as_num_test() {
        assert_eq!(bits_as_num(&[false, false, false]), 0);
        assert_eq!(bits_as_num(&[false, false, true]), 1);
        assert_eq!(bits_as_num(&[false, true, false]), 2);
    }
}
