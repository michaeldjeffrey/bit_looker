use crate::{app::MyStyles, num_format::ToFormattedString};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct State {
    bits: Vec<bool>,
    new_bit: bool,
    hovering: BitHover,
}

impl Default for State {
    fn default() -> Self {
        Self {
            bits: vec![false, false, false, false, false, false, false, false],
            new_bit: false,
            hovering: BitHover(0),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BitHover(usize);

impl BitHover {
    fn start_hover(&mut self, idx: usize) {
        self.0 = idx;
    }

    fn hover_has_entered(&self, idx: usize) -> bool {
        self.0 != idx
    }
}

enum BitEvent {
    Clicked,
    Hovered,
}

impl State {
    pub fn side_panel(&mut self, ui: &mut egui::Ui) {
        let Self { bits, .. } = self;

        side_panel_area(ui, "Left Bits", |ui| {
            add_button(ui, "+ 1", || bits.push_left(true));
            add_button(ui, "+ 0", || bits.push_left(false));
            add_button(ui, "- x", || bits.pop_left());
        });

        side_panel_area(ui, "Right Bits", |ui| {
            add_button(ui, "+ 1", || bits.push_right(true));
            add_button(ui, "+ 0", || bits.push_right(false));
            add_button(ui, "- x", || bits.pop_right());
        });

        side_panel_area(ui, "Num Bits", |ui| {
            add_button(ui, "8", || bits.empty_and_set(8));
            add_button(ui, "16", || bits.empty_and_set(16));
            add_button(ui, "32", || bits.empty_and_set(32));
            add_button(ui, "64", || bits.empty_and_set(64));
        });

        side_panel_area(ui, "Reset", |ui| {
            add_button(ui, "All 0", || bits.empty_and_set_with(bits.len(), false));
            add_button(ui, "All 1", || bits.empty_and_set_with(bits.len(), true));
            add_button(ui, "Invert", || bits.invert());
        });
    }

    pub fn main_view(&mut self, ui: &mut egui::Ui, _styles: &mut MyStyles) {
        let Self {
            bits,
            new_bit,
            hovering: clicking,
        } = self;

        let mut maybe_bit: Option<i128> = None;
        let bit_count = bits.len();
        let chunk_count = bit_count / 8;

        for (chunk_idx, chunk) in bits.chunks_mut(8).enumerate() {
            ui.horizontal(|ui| {
                // Fix OBO offset with -1.
                let end = (chunk_count - chunk_idx) * 8 - 1;
                let start = end - 7;
                let range = (start..=end).rev();

                for (bit, idx) in chunk.iter_mut().zip(range) {
                    add_bit(ui, clicking, idx, *bit, |event| match event {
                        BitEvent::Clicked => bit.flip(),
                        BitEvent::Hovered => {
                            if *bit {
                                maybe_bit = Some(0 - index_pow(idx) as i128);
                            } else {
                                maybe_bit = Some(index_pow(idx) as i128);
                            }
                        }
                    });
                }
            });

            ui.separator();
        }

        ui.horizontal(|ui| {
            add_button(ui, "<<", || bits.shift_left(*new_bit));
            add_button(ui, ">>", || bits.shift_right(*new_bit));
            add_button(ui, new_bit.clone().bit_display(), || new_bit.flip());
        });

        let num = bits_as_num(bits);
        let diff: i128 = maybe_bit.unwrap_or_default();
        let maybe_num = num as i128 + diff;

        ui.heading(num.to_formatted_string());
        let maybe_text = egui::RichText::new(format!(
            "maybe {} :: diff {}",
            maybe_num.to_formatted_string(),
            diff.to_formatted_string()
        ));
        ui.heading(maybe_text.weak());
    }
}

fn side_panel_area(
    ui: &mut egui::Ui,
    text: impl Into<egui::WidgetText>,
    mut block: impl FnMut(&mut egui::Ui),
) {
    ui.vertical(|ui| {
        ui.label(text);
        ui.horizontal(|ui| block(ui))
    });
    ui.separator();
}

fn add_button(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>, mut block: impl FnMut()) {
    if ui.button(text).clicked() {
        block();
    }
}

fn add_bit(
    ui: &mut egui::Ui,
    clicking: &mut BitHover,
    idx: usize,
    bit: bool,
    mut on_event_block: impl FnMut(BitEvent),
) {
    let rounding = 8.0;
    ui.vertical(|ui| {
        ui.label(format!("{idx}"));
        ui.small(index_pow(idx).to_formatted_string());

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
            on_event_block(BitEvent::Clicked);
        } else if b.hovered() {
            on_event_block(BitEvent::Hovered);
        } else {
            // Does the button contain the cursor,
            //   AND is it currently down
            //   AND have we not been in that state before.
            ui.ctx().input(|i| {
                if let Some(pos) = i.pointer.hover_pos() {
                    if b.rect.contains(pos)
                        && i.pointer.any_down()
                        && clicking.hover_has_entered(idx)
                    {
                        on_event_block(BitEvent::Clicked);
                        clicking.start_hover(idx);
                    }
                }
            })
        }
    });
}

// 2^0 = 1, 2^1 = 2, 2^2 = 4
fn index_pow(idx: usize) -> u64 {
    u64::pow(2, idx as u32)
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
    fn invert(&mut self);
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

    fn invert(&mut self) {
        *self = self.iter().map(|x| !x).collect();
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

fn bits_as_num(bits: &[bool]) -> u128 {
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
