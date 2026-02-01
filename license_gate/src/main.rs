use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Reverse Test - License Gate",
        options,
        Box::new(|_cc| Ok(Box::new(LicenseApp::default()))),
    )
}

#[derive(Default)]
struct LicenseApp {
    key_input: String,
    status: String,
    tries: u32,
    unlocked: bool,
}

impl eframe::App for LicenseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("License Key Required");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.key_input);
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Unlock").clicked() {
                    self.tries = self.tries.saturating_add(1);

                    if verify_key(&self.key_input) {
                        self.unlocked = true;
                        self.status = "✅ Unlocked".to_string();
                    } else {
                        self.unlocked = false;
                        self.status = "❌ Locked (wrong key)".to_string();
                    }
                }

                if ui.button("Clear").clicked() {
                    self.key_input.clear();
                    self.status.clear();
                    self.unlocked = false;
                }
            });

            ui.add_space(10.0);
            ui.separator();

            ui.label(format!(
                "Status: {}",
                if self.status.is_empty() { "Locked" } else { &self.status }
            ));
            ui.label(format!("Tries: {}", self.tries));

            ui.add_space(14.0);
            ui.separator();
            ui.heading("Next Step (Higher Level)");

            ui.add_enabled_ui(self.unlocked, |ui| {
                ui.label("🎉 Pro Panel Enabled");
                ui.add_space(6.0);
                ui.label("This area represents the next level / higher functionality.");
                ui.label("Later we can put: encrypted content, level menu, export button, etc.");
                ui.add_space(8.0);

                if ui.button("Show secret message").clicked() {
                    self.status = "FLAG{level1_unlocked}".to_string();
                }
            });

            if !self.unlocked {
                ui.small("Locked: enter a valid key to access this section.");
            }
        });
    }
}

fn verify_key(input: &str) -> bool {
    let b = input.as_bytes();
    if b.len() != 14 { return false; }
    if b[4] != b'-' || b[9] != b'-' { return false; }

    if input.starts_with("TEST") || input.contains("AAAA") { return false; }

    let mut x: u32 = 0x1234_5678;

    let mut i: usize = 0;
    let mut state: u8 = 0;
    while i < b.len() {
        match state {
            0 => { x = mix(x, b[i]); state = 1; }
            1 => { x ^= 0x1111_1111; state = 2; }
            _ => { i += 1; state = 0; }
        }
    }
    eprintln!("DEBUG x = 0x{:08X}", x);
    //x == 0xDEAD_BEEF
    x == 0x85FD063D
}

fn mix(mut x: u32, byte: u8) -> u32 {
    x ^= byte as u32;
    x = x.rotate_left(5).wrapping_add(0x9E37_79B9);
    x ^ 0xA5A5_5A5A
}