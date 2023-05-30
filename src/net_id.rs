use crate::{app::MyStyles, num_format::ToFormattedString};
use egui::Color32;
use std::str::FromStr;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct State {
    net_id: String,
    devaddr: String,
    #[serde(skip)]
    styles: Styles,
    common: Vec<CommonNetId>,
    new_name: String,
    new_net_id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Styles {
    devaddr_mem_type: Color32,
    devaddr_nwk_addr: Color32,
    devaddr_addr: Color32,
    net_id_mem_type: Color32,
    net_id_id: Color32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CommonNetId {
    name: String,
    net_id: String,
}

impl CommonNetId {
    fn new(name: &str, net_id: &str) -> Self {
        Self {
            name: name.to_string(),
            net_id: net_id.to_string(),
        }
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            devaddr_mem_type: Color32::BLUE,
            devaddr_nwk_addr: Color32::GREEN,
            devaddr_addr: Color32::RED,
            net_id_mem_type: Color32::BLUE,
            net_id_id: Color32::GREEN,
        }
    }
}

impl Styles {
    fn reset_devaddr(&self) -> Self {
        Self {
            net_id_mem_type: self.net_id_mem_type,
            net_id_id: self.net_id_id,
            ..Default::default()
        }
    }
    fn reset_net_id(&self) -> Self {
        Self {
            devaddr_addr: self.devaddr_addr,
            devaddr_mem_type: self.devaddr_mem_type,
            devaddr_nwk_addr: self.devaddr_nwk_addr,
            ..Default::default()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            net_id: Default::default(),
            devaddr: Default::default(),
            styles: Default::default(),
            common: vec![
                CommonNetId::new("Helium", "00003C"),
                CommonNetId::new("Helium", "000024"),
                CommonNetId::new("Helium", "C00053"),
                CommonNetId::new("Helium", "600053"),
            ],
            new_name: Default::default(),
            new_net_id: Default::default(),
        }
    }
}

impl State {
    fn new_common_net_id(&mut self) -> CommonNetId {
        let new = CommonNetId::new(&self.new_name, &self.new_net_id);
        self.new_name = Default::default();
        self.new_net_id = Default::default();
        new
    }

    pub fn side_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Common Net ID");

        for common in self.common.iter() {
            ui.horizontal(|ui| {
                ui.label(&common.name);
                if ui.button(&common.net_id).clicked() {
                    self.net_id = common.net_id.clone();
                }
            });
        }
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.new_name);
        });
        ui.horizontal(|ui| {
            ui.label("NetID:");
            ui.text_edit_singleline(&mut self.new_net_id);
        });
        if ui.button("Add Common").clicked() {
            let new = self.new_common_net_id();
            self.common.push(new);
        }
        ui.separator();

        ui.heading("Net ID Colors");
        egui::Grid::new("net_id_colors").show(ui, |ui| {
            ui.label("MemType:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.styles.net_id_mem_type,
                egui::color_picker::Alpha::Opaque,
            );
            ui.end_row();

            ui.label("ID:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.styles.net_id_id,
                egui::color_picker::Alpha::Opaque,
            );
            ui.end_row();

            ui.end_row();
            if ui.button("Reset").clicked() {
                self.styles = self.styles.reset_net_id();
            }
        });

        ui.heading("Devaddr Colors");
        egui::Grid::new("devaddr_colors").show(ui, |ui| {
            ui.label("MemType:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.styles.devaddr_mem_type,
                egui::color_picker::Alpha::Opaque,
            );
            ui.end_row();

            ui.label("NwkAddr:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.styles.devaddr_nwk_addr,
                egui::color_picker::Alpha::Opaque,
            );
            ui.end_row();

            ui.label("Addr:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.styles.devaddr_addr,
                egui::color_picker::Alpha::Opaque,
            );
            ui.end_row();
            if ui.button("Reset").clicked() {
                self.styles = self.styles.reset_devaddr();
            }
        });
    }

    pub fn main_view(&mut self, ui: &mut egui::Ui, _styles: &mut MyStyles) {
        let Self {
            net_id,
            devaddr,
            styles,
            ..
        } = self;

        // ====================================================================
        let net_id_field = egui::TextEdit::singleline(net_id).hint_text("Net ID");
        ui.horizontal(|ui| {
            ui.label("Net ID:");
            ui.add(net_id_field);
        });

        if let Ok(n) = NetID::new(net_id) {
            // Outside of grid to not ruin the spacing
            ui.horizontal(|ui| {
                ui.label("Bin:");
                n.as_bin(ui, styles);
            });
            egui::Grid::new("net_id_grid").show(ui, |ui| {
                ui.label("Type:");
                ui.label(n.mem_type.to_string());

                if ui
                    .add_enabled(n.mem_type > 0, egui::Button::new("-"))
                    .clicked()
                {
                    let nn = NetID::with_fields(n.mem_type - 1, n.id);
                    *net_id = nn.as_hex();
                }
                if ui
                    .add_enabled(n.mem_type < 7, egui::Button::new("+"))
                    .clicked()
                {
                    let nn = NetID::with_fields(n.mem_type + 1, n.id);
                    *net_id = nn.as_hex();
                }
                ui.end_row();

                // ======================
                ui.label("NwkID:");
                ui.label(n.id.to_string());

                if ui.add_enabled(n.id > 0, egui::Button::new("-")).clicked() {
                    let nn = NetID::with_fields(n.mem_type, n.id - 1);
                    *net_id = nn.as_hex();
                }
                if ui.add_enabled(true, egui::Button::new("+")).clicked() {
                    let nn = NetID::with_fields(n.mem_type, n.id + 1);
                    *net_id = nn.as_hex();
                }
                ui.end_row();
                // ======================
                ui.label("Hex:");
                ui.label(n.as_hex());
                ui.end_row();
                // ======================
                ui.label("Dec:");
                ui.label(n.as_dec());
                ui.end_row();
                // ======================
                ui.label("Arr:");
                ui.label(n.as_arr());
                ui.end_row();
                // ======================
                let start = n.start_addr();
                let end = n.end_addr();
                let size = end.addr - start.addr + 1;

                ui.label("Size:");
                ui.label(size.to_formatted_string());
                ui.label(format!("{} bits", addr_offset_for_mem_type(n.mem_type)));
                ui.end_row();
                // ======================
                ui.label("Range");
                if ui.button(start.as_hex()).clicked() {
                    *devaddr = start.as_hex();
                }
                ui.label("->");
                if ui.button(end.as_hex()).clicked() {
                    *devaddr = end.as_hex();
                }
            });
        }

        // ====================================================================
        ui.separator();

        let devaddr_field = egui::TextEdit::singleline(devaddr).hint_text("Devaddr");
        ui.horizontal(|ui| {
            ui.label("Devaddr:");
            ui.add(devaddr_field);
        });

        if let Ok(d) = Devaddr::new(devaddr) {
            ui.horizontal(|ui| {
                ui.label("Bin:");
                d.as_bin(ui, styles);
            });

            egui::Grid::new("devaddr_grid").show(ui, |ui| {
                // ======================
                ui.label("Hex:");
                ui.label(d.as_hex());
                ui.end_row();
                // ======================
                ui.label("Dec:");
                ui.label(d.as_dec());
                ui.end_row();
                // ======================
                ui.label("Arr:");
                ui.label(d.as_arr());
                ui.end_row();
                // ======================
                ui.label("NetID:");
                if ui.button(d.net_id().as_hex()).clicked() {
                    *net_id = d.net_id().as_hex();
                }
                ui.end_row();
                // ======================
                let net_id = d.net_id();
                let start = net_id.start_addr();
                let end = net_id.end_addr();
                let size = end.addr - start.addr + 1;

                let per = ((d.addr + 1) as f32 / size as f32) * 100.0;

                ui.label("Addr:");
                ui.label(format!(
                    "{} of {}",
                    (d.addr + 1).to_formatted_string(),
                    size.to_formatted_string()
                ));
                ui.label(format!("{}%", per));
                ui.end_row();
            });
        }
    }
}

#[derive(Debug)]
struct Devaddr {
    mem_type: u8,
    nwk_addr: u32,
    addr: u32,
}

struct NetID {
    mem_type: u8,
    id: u32,
    dec: u32,
}

impl Devaddr {
    fn new(input: &str) -> Result<Self> {
        let devaddr = match u32::from_str_radix(input, 16) {
            Err(_) => u32::from_str(input),
            Ok(devaddr) => Ok(devaddr),
        }?;

        let (mem_type, nwk_addr, addr) = {
            let mem_type = devaddr.leading_ones() as u8;

            let nwk_addr_start = mem_type as usize + 1;
            let nwk_addr_end = nwk_addr_start + nwk_id_offset_for_mem_type(mem_type);

            let addr_start = 32 - addr_offset_for_mem_type(mem_type);
            let addr_end = 32;
            (
                mem_type,
                new_num_from(devaddr, nwk_addr_start..=nwk_addr_end),
                new_num_from(devaddr, addr_start..=addr_end),
            )
        };
        Ok(Self {
            mem_type,
            nwk_addr,
            addr,
        })
    }

    fn with_fields(mem_type: u8, id: u32, addr: u32) -> Self {
        let addr_bits_to_shift = addr_offset_for_mem_type(mem_type);
        Self {
            mem_type,
            nwk_addr: id,
            addr: new_num_from(addr, (32 - addr_bits_to_shift)..=32),
        }
    }

    fn num(&self) -> u32 {
        let mem_type_bits = match self.mem_type {
            0 => 0,
            1 => 0b10 << 30,
            2 => 0b110 << 29,
            3 => 0b1110 << 28,
            4 => 0b11110 << 27,
            5 => 0b111110 << 26,
            6 => 0b1111110 << 25,
            7 => 0b11111110 << 24,
            _ => panic!("invalid mem_type: {}", self.mem_type),
        };

        let addr_bits_to_shift = addr_offset_for_mem_type(self.mem_type);

        let nwk_id_bits = self.nwk_addr << addr_bits_to_shift;
        let addr_bits = new_num_from(self.addr, (32 - addr_bits_to_shift)..=32);
        mem_type_bits | nwk_id_bits | addr_bits
    }

    fn net_id(&self) -> NetID {
        NetID::with_fields(self.mem_type, self.nwk_addr)
    }
}

impl NetID {
    fn new(input: &str) -> Result<Self> {
        match u32::from_str_radix(input, 16) {
            Ok(net_id) => {
                let mem_type = new_num_from(net_id, 8..=11);
                let nwk_addr = new_num_from(net_id, 12..=32);
                Ok(Self {
                    mem_type: mem_type as u8,
                    id: nwk_addr,
                    dec: net_id,
                })
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn with_fields(mem_type: u8, id: u32) -> Self {
        let leading = (mem_type as u32) << 21;
        let val = leading | id;
        Self {
            mem_type,
            id,
            dec: val,
        }
    }

    fn num(&self) -> u32 {
        let leading = (self.mem_type as u32) << 21;
        leading | self.id
    }

    fn start_addr(&self) -> Devaddr {
        Devaddr::with_fields(self.mem_type, self.id, u32::MIN)
    }

    fn end_addr(&self) -> Devaddr {
        Devaddr::with_fields(self.mem_type, self.id, u32::MAX)
    }
}

fn new_num_from(num: u32, range: std::ops::RangeInclusive<usize>) -> u32 {
    let end_bit = 32 - range.start() - 1;
    let start_bit = 32 - range.end(); // inclusive, don't subtract 1

    // Create a mask with ones in the specified range of bits.
    let mask = ((1 << (end_bit - start_bit + 1)) - 1) << start_bit;

    // Use the mask to extract the desired bits from num.
    (num & mask) >> start_bit
}

fn nwk_id_offset_for_mem_type(mem_type: u8) -> usize {
    match mem_type {
        0 => 6,
        1 => 6,
        2 => 9,
        3 => 11,
        4 => 12,
        5 => 13,
        6 => 15,
        7 => 17,
        _ => panic!("Invalid DevAddr: must have at least one leading zero"),
    }
}

fn addr_offset_for_mem_type(mem_type: u8) -> usize {
    match mem_type {
        0 => 25,
        1 => 24,
        2 => 20,
        3 => 17,
        4 => 15,
        5 => 13,
        6 => 10,
        7 => 7,
        _ => panic!("invalid mem_type: {}", mem_type),
    }
}

trait Printable {
    fn as_hex(&self) -> String;
    fn as_dec(&self) -> String;
    fn as_bin(&self, ui: &mut egui::Ui, styles: &Styles);
    fn as_arr(&self) -> String;
}

impl Printable for Devaddr {
    fn as_hex(&self) -> String {
        format!("{:08X}", self.num())
    }

    fn as_dec(&self) -> String {
        self.num().to_string()
    }

    fn as_bin(&self, ui: &mut egui::Ui, styles: &Styles) {
        ui.horizontal(|ui| {
            let type_bit = (self.mem_type + 1) as usize;
            let nwk_bit = nwk_id_offset_for_mem_type(self.mem_type) + type_bit;

            let binary = format!("{:032b}", self.num());
            for (idx, ch) in binary.chars().enumerate() {
                let ch = egui::RichText::new(format!(" {} ", ch));

                let out = match idx {
                    x if x < type_bit => ch.color(styles.devaddr_mem_type),
                    x if x < nwk_bit => ch.color(styles.devaddr_nwk_addr),
                    _ => ch.color(styles.devaddr_addr),
                };
                ui.label(out);

                if idx != 0 && (idx + 1) % 8 == 0 && idx != 31 {
                    ui.label(" | ");
                }
            }
        });
    }

    fn as_arr(&self) -> String {
        format!("{:?}", self.num().to_be_bytes())
    }
}

impl Printable for NetID {
    fn as_hex(&self) -> String {
        format!("{:06X}", self.num())
    }

    fn as_dec(&self) -> String {
        self.dec.to_string()
    }

    fn as_bin(&self, ui: &mut egui::Ui, styles: &Styles) {
        ui.horizontal(|ui| {
            use std::fmt::Write;

            let mut output = String::new();
            let rfu_bits = match self.mem_type {
                0 | 1 | 2 => 15 + 3, // include type bits
                _ => 0,
            };

            let binary = format!("{:024b}", self.num());
            for (idx, ch) in binary.chars().enumerate() {
                let ch = egui::RichText::new(format!(" {} ", ch));
                let out = match idx {
                    x if x < 3 => ch.color(styles.net_id_mem_type),
                    x if x < rfu_bits => ch.color(Color32::DARK_GRAY).strikethrough(),
                    _ => ch.color(styles.net_id_id),
                };

                ui.label(out);
                // write!(output, "{}", out).unwrap();

                if idx != 0 && idx % 8 == 0 {
                    ui.label(" | ");
                    write!(output, " | ").unwrap();
                }
            }
        });
        // output

        // print_binary_bytes(self.dec, 3)
    }

    fn as_arr(&self) -> String {
        let num: u32 = self.num();
        let arr: [u8; 4] = num.to_be_bytes();
        format!("[{}, {}, {}]", arr[1], arr[2], arr[3])
    }
}

impl FromStr for Devaddr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Devaddr::new(s).map_err(|_| "Invalid Devaddr".to_owned())
    }
}

impl FromStr for NetID {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NetID::new(s).map_err(|_| "Invalid Net ID".to_owned())
    }
}
