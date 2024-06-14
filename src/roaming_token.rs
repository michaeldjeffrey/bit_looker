#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct State {
    input_token: String,
}

#[derive(Debug)]
struct Token {
    region: String,
    packet_time: u64,
    route_id: String,
    b58: String,
    animal_name: String,
}

impl State {
    pub fn side_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Links");
        ui.hyperlink_to("Roaming Token Src", "https://github.com/helium/helium-packet-router/blob/main/src/protocols/http/hpr_http_roaming.erl#L414-L446");
    }

    pub fn main_view(&mut self, ui: &mut egui::Ui) {
        let Self { input_token } = self;

        let token_field = egui::TextEdit::singleline(input_token).hint_text("FNSULToken");
        ui.horizontal(|ui| {
            ui.label("FNSULToken:");
            ui.add(token_field);
        });

        if let Some(token) = parse_token(input_token) {
            egui::Grid::new("roaming-token-grid").show(ui, |ui| {
                ui.label("Region:");
                ui.label(token.region);
                ui.end_row();

                let time = chrono::DateTime::from_timestamp_millis(token.packet_time as i64)
                    .expect("valid timestamp");
                let date = humantime::format_rfc3339_millis(time.into());
                let mut time_formatter = timeago::Formatter::new();
                time_formatter.num_items(4);
                let ago = time_formatter.convert_chrono(time, chrono::Utc::now());
                ui.label("Packet Time:");
                ui.label(format!("{} ({date})", token.packet_time));
                ui.label(ago);
                ui.end_row();

                ui.label("Route ID:");
                ui.label(token.route_id);
                ui.end_row();

                ui.label("Gateway:");
                ui.label(token.b58);
                ui.label(token.animal_name);
                ui.end_row();
            });
        } else {
            ui.label("Unparseable Token");
        }
    }
}

#[test]
fn test_parse_token() {
    //let input = "0x55533931353A3A313731373836373039363937383A3A313A3A01B9369F0B077DA6E65CA5D895565F507D912F32B190066F2DFD9520227A36AAC0".to_string();
    let input = "0x45553836383A3A323638393537393432343A3A31323739363333652D303661302D313165652D393839642D6637316363643537613231383A3A0072C4AE468379170A7F7955A4375C084A8327CAE9ACF0432DC91CFF2E66512257".to_string();
    let token = parse_token(&input);
    println!("{token:?}");
}

fn parse_token(input: &str) -> Option<Token> {
    // remove preceding 0x if it exists
    let input = input.trim_start_matches("0x");
    if input.is_empty() {
        return None;
    }

    let decoded = hex::decode(input).ok()?;

    // region::packet_time::route_id::pubkeybin
    let re = regex::bytes::Regex::new(
        "(?<region>.*)::(?<packet_time>.*)::(?<route_id>.*)::(?<pubkeybin>(?-u:.)*)",
    )
    .ok()?;
    let caps = re.captures(&decoded)?;

    let region_bytes = caps.name("region")?.as_bytes();
    let region = String::from_utf8(region_bytes.to_vec()).ok()?;

    let packet_time_bytes = caps.name("packet_time")?.as_bytes();
    let packet_time_str = String::from_utf8(packet_time_bytes.to_vec()).ok()?;
    let packet_time = u64::from_str_radix(&packet_time_str, 10).ok()?;

    let route_id_bytes = caps.name("route_id")?.as_bytes();
    let route_id = String::from_utf8(route_id_bytes.to_vec()).ok()?;

    // Regex may not grab all bytes of pubkey, get from start to the end manually.
    let pubkey_cap = &caps.name("pubkeybin")?;
    let pubkey_bytes = &decoded[pubkey_cap.start()..];
    // Taken from helium-crypto PubkeyBinary Display trait
    let mut pubkey = vec![0u8; pubkey_bytes.len() + 1];
    pubkey[1..].copy_from_slice(&pubkey_bytes);
    let b58 = bs58::encode(&pubkey).with_check().into_string();

    let animal_name = b58
        .parse::<angry_purple_tiger::AnimalName>()
        .expect("animal_name")
        .to_string();

    Some(Token {
        region,
        packet_time,
        route_id,
        b58,
        animal_name,
    })
}
