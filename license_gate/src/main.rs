use eframe::egui;
use std::collections::HashMap;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CyberVault Pro - Secure Data Manager v3.2",
        options,
        Box::new(|cc| {
            // Use dark theme
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(LicenseApp::default()))
        }),
    )
}

#[derive(PartialEq, Clone)]
enum Tab {
    Dashboard,
    Encryption,
    Passwords,
    Vault,
    Settings,
}

struct LicenseApp {
    key_input: String,
    status: String,
    tries: u32,
    unlocked: bool,
    show_activation: bool,
    current_tab: Tab,
    // Encryption feature state
    file_to_encrypt: String,
    encryption_progress: f32,
    // Password manager state
    passwords: HashMap<String, String>,
    new_site: String,
    new_password: String,
    // Vault state
    vault_items: Vec<VaultItem>,
    // Settings
    auto_lock: bool,
    notifications: bool,
}

struct VaultItem {
    name: String,
    category: String,
    size: String,
    encrypted: bool,
}

impl Default for LicenseApp {
    fn default() -> Self {
        let mut passwords = HashMap::new();
        passwords.insert("github.com".to_string(), "***********".to_string());
        passwords.insert("email@work.com".to_string(), "***********".to_string());

        let vault_items = vec![
            VaultItem {
                name: "Financial_Report_2025.xlsx".to_string(),
                category: "Documents".to_string(),
                size: "2.4 MB".to_string(),
                encrypted: true,
            },
            VaultItem {
                name: "Personal_Photos.zip".to_string(),
                category: "Media".to_string(),
                size: "150 MB".to_string(),
                encrypted: true,
            },
            VaultItem {
                name: "Backup_Codes.txt".to_string(),
                category: "Security".to_string(),
                size: "1.2 KB".to_string(),
                encrypted: true,
            },
        ];

        Self {
            key_input: String::new(),
            status: String::new(),
            tries: 0,
            unlocked: false,
            show_activation: true,
            current_tab: Tab::Dashboard,
            file_to_encrypt: String::new(),
            encryption_progress: 0.0,
            passwords,
            new_site: String::new(),
            new_password: String::new(),
            vault_items,
            auto_lock: true,
            notifications: true,
        }
    }
}

impl eframe::App for LicenseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.heading(egui::RichText::new("🔐 CyberVault Pro").size(24.0).color(egui::Color32::from_rgb(100, 200, 255)));
                ui.add_space(16.0);
                ui.label(egui::RichText::new("Secure Data Manager").size(14.0).color(egui::Color32::GRAY));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(16.0);
                    if self.unlocked {
                        ui.label(egui::RichText::new("✓ Licensed").color(egui::Color32::from_rgb(100, 255, 100)));
                        if ui.button("⚙ Settings").clicked() {
                            self.current_tab = Tab::Settings;
                        }
                    } else {
                        ui.label(egui::RichText::new("⚠ Trial Mode").color(egui::Color32::from_rgb(255, 180, 0)));
                        if ui.button("🔓 Activate License").clicked() {
                            self.show_activation = true;
                        }
                    }
                });
            });
            ui.add_space(8.0);
        });

        // License activation window
        if self.show_activation && !self.unlocked {
            egui::Window::new("License Activation")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(400.0);
                    
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("CyberVault Pro - Enterprise Edition").size(18.0).strong());
                    ui.add_space(8.0);
                    
                    ui.label("Enter your license key to unlock all premium features:");
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("License Key:");
                        let response = ui.add_sized(
                            [250.0, 20.0],
                            egui::TextEdit::singleline(&mut self.key_input)
                                .hint_text("XXXX-XXXX-XXXX")
                                .password(false)
                        );
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.attempt_unlock();
                        }
                    });

                    ui.add_space(8.0);

                    if !self.status.is_empty() {
                        let color = if self.unlocked {
                            egui::Color32::from_rgb(100, 255, 100)
                        } else {
                            egui::Color32::from_rgb(255, 100, 100)
                        };
                        ui.colored_label(color, &self.status);
                    }

                    if self.tries > 0 {
                        ui.label(format!("Activation attempts: {}", self.tries));
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("🔓 Activate").size(14.0)).clicked() {
                            self.attempt_unlock();
                        }

                        if ui.button("Clear").clicked() {
                            self.key_input.clear();
                            self.status.clear();
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Continue Trial").clicked() {
                                self.show_activation = false;
                            }
                        });
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.small("Note: This is a reverse engineering challenge.");
                    ui.small("Format: XXXX-XXXX-XXXX (14 characters including dashes)");
                });
        }

        // Side panel for navigation
        egui::SidePanel::left("side_panel").min_width(200.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.label(egui::RichText::new("NAVIGATION").size(12.0).color(egui::Color32::GRAY));
            ui.add_space(10.0);

            let dashboard_selected = self.current_tab == Tab::Dashboard;
            if ui.selectable_label(dashboard_selected, "📊 Dashboard").clicked() {
                self.current_tab = Tab::Dashboard;
            }

            let encryption_selected = self.current_tab == Tab::Encryption;
            if ui.add_enabled(self.unlocked, egui::Button::new("🔒 File Encryption").selected(encryption_selected)).clicked() {
                self.current_tab = Tab::Encryption;
            }

            let passwords_selected = self.current_tab == Tab::Passwords;
            if ui.add_enabled(self.unlocked, egui::Button::new("🔑 Password Manager").selected(passwords_selected)).clicked() {
                self.current_tab = Tab::Passwords;
            }

            let vault_selected = self.current_tab == Tab::Vault;
            if ui.add_enabled(self.unlocked, egui::Button::new("🗄 Secure Vault").selected(vault_selected)).clicked() {
                self.current_tab = Tab::Vault;
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            if !self.unlocked {
                ui.label(egui::RichText::new("⚠ Trial Limitations:").color(egui::Color32::from_rgb(255, 180, 0)).strong());
                ui.add_space(6.0);
                ui.small("• Limited file encryption");
                ui.small("• No password manager");
                ui.small("• Read-only vault access");
                ui.small("• Watermarked exports");
                ui.add_space(10.0);
                if ui.button("🔓 Upgrade Now").clicked() {
                    self.show_activation = true;
                }
            } else {
                ui.label(egui::RichText::new("✓ Full Access").color(egui::Color32::from_rgb(100, 255, 100)).strong());
                ui.add_space(6.0);
                ui.small("All features unlocked");
            }
        });

        // Main content panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            match self.current_tab {
                Tab::Dashboard => self.show_dashboard(ui),
                Tab::Encryption => self.show_encryption(ui),
                Tab::Passwords => self.show_passwords(ui),
                Tab::Vault => self.show_vault(ui),
                Tab::Settings => self.show_settings(ui),
            }
        });
    }
}

impl LicenseApp {
    fn attempt_unlock(&mut self) {
        self.tries = self.tries.saturating_add(1);

        if verify_key(&self.key_input) {
            self.unlocked = true;
            self.status = "✅ License activated successfully! All features unlocked.".to_string();
            self.show_activation = false;
            self.current_tab = Tab::Dashboard;
        } else {
            self.unlocked = false;
            if self.tries >= 5 {
                self.status = "❌ Invalid license key. Too many attempts - contact support.".to_string();
            } else {
                self.status = format!("❌ Invalid license key. Please check your key and try again.");
            }
        }
    }

    fn show_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard");
        ui.add_space(10.0);

        // Welcome section
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                if self.unlocked {
                    ui.label(egui::RichText::new("Welcome to CyberVault Pro!").size(20.0).strong());
                    ui.add_space(8.0);
                    ui.label("Your data is secured with military-grade encryption.");
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("🏆 Premium License Active").color(egui::Color32::from_rgb(255, 215, 0)));
                } else {
                    ui.label(egui::RichText::new("Trial Version").size(20.0).strong());
                    ui.add_space(8.0);
                    ui.label("You're using the trial version with limited features.");
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Activate your license to unlock all features!").color(egui::Color32::from_rgb(255, 180, 0)));
                }
            });

        ui.add_space(20.0);

        // Stats section
        ui.label(egui::RichText::new("SECURITY OVERVIEW").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Encrypted Files
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(25, 60, 45))
                .corner_radius(6.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(150.0);
                    ui.label(egui::RichText::new("🔒 Encrypted Files").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(if self.unlocked { "127" } else { "3" }).size(28.0).strong());
                });

            ui.add_space(10.0);

            // Passwords Secured
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(45, 35, 60))
                .corner_radius(6.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(150.0);
                    ui.label(egui::RichText::new("🔑 Passwords Secured").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(if self.unlocked { "42" } else { "2" }).size(28.0).strong());
                });

            ui.add_space(10.0);

            // Security Score
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(60, 45, 25))
                .corner_radius(6.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(150.0);
                    ui.label(egui::RichText::new("⭐ Security Score").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(if self.unlocked { "98%" } else { "45%" }).size(28.0).strong());
                });
        });

        ui.add_space(20.0);

        // Features section
        ui.label(egui::RichText::new("PREMIUM FEATURES").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        let features = vec![
            ("🔒 Military-Grade Encryption", "AES-256 encryption for all your files", self.unlocked),
            ("🔑 Password Manager", "Securely store and manage unlimited passwords", self.unlocked),
            ("🗄 Secure Vault", "Protected storage for sensitive documents", self.unlocked),
            ("☁️ Cloud Sync", "Automatic backup to encrypted cloud storage", self.unlocked),
            ("🔔 Security Alerts", "Real-time notifications for security events", self.unlocked),
            ("📊 Activity Reports", "Detailed logs and analytics", self.unlocked),
        ];

        egui::Grid::new("features_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                for (i, (title, desc, enabled)) in features.iter().enumerate() {
                    egui::Frame::new()
                        .fill(if *enabled { egui::Color32::from_rgb(30, 35, 45) } else { egui::Color32::from_rgb(35, 30, 30) })
                        .corner_radius(6.0)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.set_min_width(250.0);
                            let color = if *enabled { egui::Color32::WHITE } else { egui::Color32::GRAY };
                            ui.label(egui::RichText::new(*title).size(14.0).color(color).strong());
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(*desc).size(11.0).color(egui::Color32::GRAY));
                            if !enabled {
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("🔒 License Required").size(10.0).color(egui::Color32::from_rgb(255, 180, 0)));
                            }
                        });

                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });

        if !self.unlocked {
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            if ui.button(egui::RichText::new("🔓 Activate License Now").size(16.0)).clicked() {
                self.show_activation = true;
            }
        } else {
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            ui.label(egui::RichText::new("🎉 Congratulations! You've unlocked all features.").size(14.0).color(egui::Color32::from_rgb(100, 255, 100)));
            ui.add_space(6.0);
            ui.label("Your secret challenge flag:");
            ui.code(egui::RichText::new("FLAG{cybervault_pro_license_cracked_successfully}").color(egui::Color32::from_rgb(255, 215, 0)));
        }
    }

    fn show_encryption(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔒 File Encryption");
        ui.add_space(10.0);
        ui.label("Encrypt your sensitive files with AES-256 encryption.");
        ui.add_space(15.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Select File to Encrypt").strong());
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.file_to_encrypt);
                    if ui.button("📁 Browse").clicked() {
                        self.file_to_encrypt = "example_document.pdf".to_string();
                    }
                });

                ui.add_space(12.0);

                if ui.button(egui::RichText::new("🔒 Encrypt File").size(14.0)).clicked() {
                    self.encryption_progress = 1.0;
                }

                if self.encryption_progress > 0.0 {
                    ui.add_space(10.0);
                    ui.add(egui::ProgressBar::new(self.encryption_progress).text("Encrypting..."));
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("✓ File encrypted successfully!").color(egui::Color32::from_rgb(100, 255, 100)));
                }
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("ENCRYPTION SETTINGS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        ui.checkbox(&mut true, "Use AES-256 encryption");
        ui.checkbox(&mut true, "Delete original after encryption");
        ui.checkbox(&mut false, "Add timestamp to encrypted files");
    }

    fn show_passwords(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔑 Password Manager");
        ui.add_space(10.0);
        ui.label("Store and manage your passwords securely.");
        ui.add_space(15.0);

        // Add new password section
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Add New Password").strong());
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Site:");
                    ui.text_edit_singleline(&mut self.new_site);
                });

                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.add(egui::TextEdit::singleline(&mut self.new_password).password(true));
                });

                ui.add_space(8.0);

                if ui.button("➕ Add Password").clicked() {
                    if !self.new_site.is_empty() && !self.new_password.is_empty() {
                        self.passwords.insert(self.new_site.clone(), self.new_password.clone());
                        self.new_site.clear();
                        self.new_password.clear();
                    }
                }
            });

        ui.add_space(20.0);

        // Stored passwords
        ui.label(egui::RichText::new("STORED PASSWORDS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for (site, _password) in &self.passwords {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(25, 30, 40))
                    .corner_radius(6.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(site).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("🗑").clicked() {
                                    // Would delete in real app
                                }
                                if ui.small_button("📋").clicked() {
                                    // Would copy in real app
                                }
                                if ui.small_button("👁").clicked() {
                                    // Would show in real app
                                }
                            });
                        });
                        ui.label(egui::RichText::new("••••••••••").color(egui::Color32::GRAY));
                    });
                ui.add_space(6.0);
            }
        });
    }

    fn show_vault(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗄 Secure Vault");
        ui.add_space(10.0);
        ui.label("Your encrypted files in the secure vault.");
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            if ui.button("➕ Add File").clicked() {
                // Would add file in real app
            }
            if ui.button("📥 Import").clicked() {
                // Would import in real app
            }
            if ui.button("📤 Export").clicked() {
                // Would export in real app
            }
        });

        ui.add_space(15.0);

        // Vault items
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("vault_grid")
                .num_columns(5)
                .spacing([10.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Name").strong());
                    ui.label(egui::RichText::new("Category").strong());
                    ui.label(egui::RichText::new("Size").strong());
                    ui.label(egui::RichText::new("Status").strong());
                    ui.label(egui::RichText::new("Actions").strong());
                    ui.end_row();

                    for item in &self.vault_items {
                        ui.label(&item.name);
                        ui.label(&item.category);
                        ui.label(&item.size);
                        if item.encrypted {
                            ui.label(egui::RichText::new("🔒 Encrypted").color(egui::Color32::from_rgb(100, 255, 100)));
                        } else {
                            ui.label(egui::RichText::new("⚠ Unencrypted").color(egui::Color32::from_rgb(255, 180, 0)));
                        }
                        ui.horizontal(|ui| {
                            if ui.small_button("📥").clicked() {
                                // Download
                            }
                            if ui.small_button("🗑").clicked() {
                                // Delete
                            }
                        });
                        ui.end_row();
                    }
                });
        });
    }

    fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ Settings");
        ui.add_space(15.0);

        ui.label(egui::RichText::new("SECURITY SETTINGS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.checkbox(&mut self.auto_lock, "Auto-lock after 15 minutes of inactivity");
                ui.checkbox(&mut self.notifications, "Enable security notifications");
                ui.checkbox(&mut true, "Require password on startup");
                ui.checkbox(&mut true, "Enable two-factor authentication");
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("LICENSE INFORMATION").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(format!("License Type: {}", if self.unlocked { "Enterprise Pro" } else { "Trial" }));
                ui.label(format!("Version: 3.2.1"));
                ui.label(format!("Build: 20260201"));
                ui.add_space(8.0);
                if !self.unlocked {
                    if ui.button("🔓 Activate License").clicked() {
                        self.show_activation = true;
                    }
                } else {
                    ui.label(egui::RichText::new("✓ Licensed").color(egui::Color32::from_rgb(100, 255, 100)));
                }
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("ABOUT").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label("CyberVault Pro - Secure Data Manager");
                ui.label("Version 3.2.1 (2026)");
                ui.add_space(8.0);
                ui.small("This is a reverse engineering challenge application.");
                ui.small("Find the correct license key to unlock all features.");
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