use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::Local;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1250.0, 800.0])
            .with_min_inner_size([1000.0, 750.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CyberVault Pro - Secure Data Manager v3.2",
        options,
        Box::new(|cc| {
            // Use clean dark theme
            let mut visuals = egui::Visuals::dark();
            // Balanced colors for good readability
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 55, 65);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 70, 85);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(70, 120, 200);
            cc.egui_ctx.set_visuals(visuals);
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
    ActivityLog,
    SecurityScan,
    BackupRestore,
    Analytics,
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
    encryption_running: bool,
    // Password manager state
    passwords: HashMap<String, String>,
    password_visible: HashMap<String, bool>,
    new_site: String,
    new_password: String,
    // Vault state
    vault_items: Vec<VaultItem>,
    // Activity Log state
    activity_logs: Vec<ActivityLog>,
    // Security Scan state
    scan_progress: f32,
    scan_running: bool,
    threats_found: u32,
    // Backup/Restore state
    backup_location: String,
    last_backup: String,
    backup_history: Vec<(String, String, String)>,
    // Analytics state
    analytics_data: Vec<(String, f32)>,
    // Settings
    auto_lock: bool,
    notifications: bool,
    require_password: bool,
    two_factor: bool,
    // Encryption settings
    use_aes256: bool,
    delete_original: bool,
    add_timestamp: bool,
    // Security scan settings
    scan_encrypted: bool,
    scan_passwords: bool,
    check_vulnerabilities: bool,
    deep_scan: bool,
    // Clipboard state
    clipboard_message: Option<String>,
    clipboard_timer: f32,
}

struct ActivityLog {
    timestamp: String,
    action: String,
    status: String,
    icon: String,
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

        let activity_logs = vec![
            ActivityLog {
                timestamp: "2026-02-01 08:30:15".to_string(),
                action: "File encrypted".to_string(),
                status: "Success".to_string(),
                icon: "🔒".to_string(),
            },
            ActivityLog {
                timestamp: "2026-02-01 08:25:42".to_string(),
                action: "Password added".to_string(),
                status: "Success".to_string(),
                icon: "🔑".to_string(),
            },
            ActivityLog {
                timestamp: "2026-02-01 08:20:10".to_string(),
                action: "Security scan completed".to_string(),
                status: "No threats".to_string(),
                icon: "🛡️".to_string(),
            },
        ];

        let analytics_data = vec![
            ("Encryption".to_string(), 45.0),
            ("Passwords".to_string(), 30.0),
            ("Vault".to_string(), 15.0),
            ("Other".to_string(), 10.0),
        ];

        let backup_history = vec![
            ("2026-02-01 07:00:00".to_string(), "Full Backup".to_string(), "2.4 GB".to_string()),
            ("2026-01-31 07:00:00".to_string(), "Full Backup".to_string(), "2.3 GB".to_string()),
            ("2026-01-30 07:00:00".to_string(), "Incremental".to_string(), "150 MB".to_string()),
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
            encryption_running: false,
            passwords,
            password_visible: HashMap::new(),
            new_site: String::new(),
            new_password: String::new(),
            vault_items,
            activity_logs,
            scan_progress: 0.0,
            scan_running: false,
            threats_found: 0,
            backup_location: "/backups/cybervault".to_string(),
            last_backup: "2026-02-01 07:00:00".to_string(),
            backup_history,
            analytics_data,
            auto_lock: true,
            notifications: true,
            require_password: true,
            two_factor: true,
            use_aes256: true,
            delete_original: true,
            add_timestamp: false,
            scan_encrypted: true,
            scan_passwords: true,
            check_vulnerabilities: true,
            deep_scan: false,
            clipboard_message: None,
            clipboard_timer: 0.0,
        }
    }
}

impl eframe::App for LicenseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update animations and timers
        self.update_animations(ctx);
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

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.label(egui::RichText::new("ADVANCED").size(11.0).color(egui::Color32::GRAY));

            let activity_selected = self.current_tab == Tab::ActivityLog;
            if ui.add_enabled(self.unlocked, egui::Button::new("📋 Activity Log").selected(activity_selected)).clicked() {
                self.current_tab = Tab::ActivityLog;
            }

            let security_selected = self.current_tab == Tab::SecurityScan;
            if ui.add_enabled(self.unlocked, egui::Button::new("🛡️ Security Scan").selected(security_selected)).clicked() {
                self.current_tab = Tab::SecurityScan;
            }

            let backup_selected = self.current_tab == Tab::BackupRestore;
            if ui.add_enabled(self.unlocked, egui::Button::new("💾 Backup/Restore").selected(backup_selected)).clicked() {
                self.current_tab = Tab::BackupRestore;
            }

            let analytics_selected = self.current_tab == Tab::Analytics;
            if ui.add_enabled(self.unlocked, egui::Button::new("📊 Analytics").selected(analytics_selected)).clicked() {
                self.current_tab = Tab::Analytics;
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
                Tab::ActivityLog => self.show_activity_log(ui),
                Tab::SecurityScan => self.show_security_scan(ui),
                Tab::BackupRestore => self.show_backup_restore(ui),
                Tab::Analytics => self.show_analytics(ui),
                Tab::Settings => self.show_settings(ui),
            }
        });
    }
}

impl LicenseApp {
    fn update_animations(&mut self, ctx: &egui::Context) {
        // Update clipboard message timer
        if let Some(_) = self.clipboard_message {
            self.clipboard_timer -= ctx.input(|i| i.unstable_dt);
            if self.clipboard_timer <= 0.0 {
                self.clipboard_message = None;
            }
        }

        // Update encryption progress
        if self.encryption_running && self.encryption_progress < 1.0 {
            self.encryption_progress = (self.encryption_progress + ctx.input(|i| i.unstable_dt * 0.5)).min(1.0);
            if self.encryption_progress >= 1.0 {
                self.encryption_running = false;
                self.add_activity_log("File encrypted".to_string(), "Success".to_string(), "🔒".to_string());
            }
            ctx.request_repaint();
        }

        // Update security scan progress
        if self.scan_running && self.scan_progress < 1.0 {
            self.scan_progress = (self.scan_progress + ctx.input(|i| i.unstable_dt * 0.3)).min(1.0);
            if self.scan_progress >= 1.0 {
                self.scan_running = false;
                // Use timestamp-based pseudo-random for threats
                let now = Local::now().timestamp() as u32;
                self.threats_found = if now % 10 < 3 { 1 } else { 0 };
                self.add_activity_log("Security scan completed".to_string(), 
                    if self.threats_found == 0 { "No threats".to_string() } else { "Threats found".to_string() },
                    "🛡️".to_string());
            }
            ctx.request_repaint();
        }
    }

    fn add_activity_log(&mut self, action: String, status: String, icon: String) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.activity_logs.insert(0, ActivityLog {
            timestamp,
            action,
            status,
            icon,
        });
        // Keep only last 50 logs
        if self.activity_logs.len() > 50 {
            self.activity_logs.truncate(50);
        }
    }

    fn copy_to_clipboard(&mut self, text: &str) {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(text);
            self.clipboard_message = Some("Copied to clipboard!".to_string());
            self.clipboard_timer = 2.0;
        }
    }

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
        ui.heading("📊 Dashboard");
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
                        let path_opt: Option<PathBuf> = rfd::FileDialog::new()
                            .add_filter("All Files", &["*"])
                            .pick_file();
                        if let Some(path) = path_opt {
                            self.file_to_encrypt = path.to_string_lossy().into_owned();
                        }
                    }
                });

                ui.add_space(12.0);

                if ui.add_enabled(!self.encryption_running && !self.file_to_encrypt.is_empty(), 
                    egui::Button::new(egui::RichText::new("🔒 Encrypt File").size(14.0))).clicked() {
                    self.encryption_progress = 0.0;
                    self.encryption_running = true;
                }

                if self.encryption_progress > 0.0 {
                    ui.add_space(10.0);
                    ui.add(egui::ProgressBar::new(self.encryption_progress).text("Encrypting..."));
                    if self.encryption_progress >= 1.0 && !self.encryption_running {
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("✓ File encrypted successfully!").color(egui::Color32::from_rgb(100, 255, 100)));
                        ui.label(format!("Encrypted: {}", self.file_to_encrypt));
                    }
                }
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("ENCRYPTION SETTINGS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        ui.checkbox(&mut self.use_aes256, "Use AES-256 encryption");
        ui.checkbox(&mut self.delete_original, "Delete original after encryption");
        ui.checkbox(&mut self.add_timestamp, "Add timestamp to encrypted files");
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
                        self.add_activity_log(format!("Password added for {}", self.new_site.clone()), 
                            "Success".to_string(), "🔑".to_string());
                        self.new_site.clear();
                        self.new_password.clear();
                    }
                }
            });

        ui.add_space(20.0);

        // Stored passwords
        ui.label(egui::RichText::new("STORED PASSWORDS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        let mut sites_to_remove = Vec::new();
        let mut passwords_to_copy = Vec::new();
        let mut passwords_to_toggle = Vec::new();
        
        egui::ScrollArea::vertical().max_height(500.0).show(ui, |ui| {
            for (site, password) in &self.passwords {
                let visible = self.password_visible.get(site).copied().unwrap_or(false);
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(25, 30, 40))
                    .corner_radius(6.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(site).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("🗑").clicked() {
                                    sites_to_remove.push(site.clone());
                                }
                                if ui.small_button("📋").clicked() {
                                    passwords_to_copy.push(password.clone());
                                }
                                if ui.small_button(if visible { "🙈" } else { "👁" }).clicked() {
                                    passwords_to_toggle.push(site.clone());
                                }
                            });
                        });
                        if visible {
                            ui.label(egui::RichText::new(password).color(egui::Color32::from_rgb(200, 200, 200)));
                        } else {
                            ui.label(egui::RichText::new("••••••••••").color(egui::Color32::GRAY));
                        }
                    });
                ui.add_space(6.0);
            }
        });
        
        // Process actions after the UI loop
        for password in passwords_to_copy {
            self.copy_to_clipboard(&password);
        }
        for site in passwords_to_toggle {
            let current = self.password_visible.get(&site).copied().unwrap_or(false);
            self.password_visible.insert(site, !current);
        }
        for site in sites_to_remove {
            self.passwords.remove(&site);
            self.password_visible.remove(&site);
            self.add_activity_log(format!("Password removed for {}", site), 
                "Success".to_string(), "🗑".to_string());
        }

        // Show clipboard message
        if let Some(ref msg) = self.clipboard_message {
            ui.add_space(10.0);
            ui.label(egui::RichText::new(msg).color(egui::Color32::from_rgb(100, 255, 100)));
        }
    }

    fn show_vault(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗄 Secure Vault");
        ui.add_space(10.0);
        ui.label("Your encrypted files in the secure vault.");
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            if ui.button("➕ Add File").clicked() {
                let path_opt: Option<PathBuf> = rfd::FileDialog::new()
                    .add_filter("All Files", &["*"])
                    .pick_file();
                if let Some(path) = path_opt {
                    let file_name: String = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let size = if let Ok(metadata) = std::fs::metadata(&path) {
                        let bytes = metadata.len();
                        if bytes < 1024 {
                            format!("{} B", bytes)
                        } else if bytes < 1024 * 1024 {
                            format!("{:.1} KB", bytes as f64 / 1024.0)
                        } else {
                            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                        }
                    } else {
                        "Unknown".to_string()
                    };
                    let category = if file_name.ends_with(".pdf") || file_name.ends_with(".doc") || file_name.ends_with(".xlsx") {
                        "Documents".to_string()
                    } else if file_name.ends_with(".jpg") || file_name.ends_with(".png") || file_name.ends_with(".zip") {
                        "Media".to_string()
                    } else {
                        "Other".to_string()
                    };
                    self.vault_items.push(VaultItem {
                        name: file_name.clone(),
                        category,
                        size,
                        encrypted: true,
                    });
                    self.add_activity_log(format!("File added to vault: {}", file_name), 
                        "Success".to_string(), "🗄".to_string());
                }
            }
            if ui.button("📥 Import").clicked() {
                let path_opt: Option<PathBuf> = rfd::FileDialog::new()
                    .add_filter("All Files", &["*"])
                    .pick_file();
                if let Some(path) = path_opt {
                    let file_name: String = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    self.vault_items.push(VaultItem {
                        name: file_name.clone(),
                        category: "Imported".to_string(),
                        size: "Unknown".to_string(),
                        encrypted: false,
                    });
                    self.add_activity_log(format!("File imported: {}", file_name), 
                        "Success".to_string(), "📥".to_string());
                }
            }
            if ui.button("📤 Export").clicked() {
                let dir_opt: Option<PathBuf> = rfd::FileDialog::new().pick_folder();
                if let Some(dir) = dir_opt {
                    self.add_activity_log(format!("Vault exported to: {}", dir.to_string_lossy()), 
                        "Success".to_string(), "📤".to_string());
                }
            }
        });

        ui.add_space(15.0);

        // Vault items
        let mut items_to_remove = Vec::new();
        let mut download_actions = Vec::new();
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

                    for (idx, item) in self.vault_items.iter().enumerate() {
                        ui.label(&item.name);
                        ui.label(&item.category);
                        ui.label(&item.size);
                        if item.encrypted {
                            ui.label(egui::RichText::new("🔒 Encrypted").color(egui::Color32::from_rgb(100, 255, 100)));
                        } else {
                            ui.label(egui::RichText::new("⚠ Unencrypted").color(egui::Color32::from_rgb(255, 180, 0)));
                        }
                        ui.horizontal(|ui| {
                            let item_name = item.name.clone();
                            if ui.small_button("📥").clicked() {
                                let _dir_opt: Option<PathBuf> = rfd::FileDialog::new().pick_folder();
                                if _dir_opt.is_some() {
                                    download_actions.push(item_name);
                                }
                            }
                            if ui.small_button("🗑").clicked() {
                                items_to_remove.push(idx);
                            }
                        });
                        ui.end_row();
                    }
                });
        });
        // Process download actions after the loop
        for item_name in download_actions {
            self.add_activity_log(format!("File downloaded: {}", item_name), 
                "Success".to_string(), "📥".to_string());
        }
        // Remove deleted items (in reverse order to maintain indices)
        items_to_remove.sort_unstable();
        items_to_remove.reverse();
        for idx in items_to_remove {
            if idx < self.vault_items.len() {
                let item_name = self.vault_items[idx].name.clone();
                self.vault_items.remove(idx);
                self.add_activity_log(format!("File removed from vault: {}", item_name), 
                    "Success".to_string(), "🗑".to_string());
            }
        }
    }

    fn show_activity_log(&mut self, ui: &mut egui::Ui) {
        ui.heading("📋 Activity Log");
        ui.add_space(10.0);
        ui.label("View all security events and user activities.");
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh").clicked() {
                self.add_activity_log("Activity log refreshed".to_string(), 
                    "Success".to_string(), "🔄".to_string());
            }
            if ui.button("📥 Export").clicked() {
                let path_opt: Option<PathBuf> = rfd::FileDialog::new()
                    .add_filter("Text Files", &["txt"])
                    .set_file_name("activity_log.txt")
                    .save_file();
                if let Some(path) = path_opt {
                    // In a real app, would write logs to file
                    self.add_activity_log(format!("Activity log exported to: {}", path.to_string_lossy()), 
                        "Success".to_string(), "📥".to_string());
                }
            }
            if ui.button("🗑 Clear").clicked() {
                self.activity_logs.clear();
                self.add_activity_log("Activity log cleared".to_string(), 
                    "Success".to_string(), "🗑".to_string());
            }
        });

        ui.add_space(15.0);

        egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
            for log in &self.activity_logs {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(25, 30, 40))
                    .corner_radius(6.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&log.icon).size(20.0));
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&log.action).strong());
                                ui.label(egui::RichText::new(&log.timestamp).size(11.0).color(egui::Color32::GRAY));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let color = if log.status == "Success" {
                                    egui::Color32::from_rgb(100, 255, 100)
                                } else {
                                    egui::Color32::from_rgb(255, 180, 0)
                                };
                                ui.label(egui::RichText::new(&log.status).color(color).strong());
                            });
                        });
                    });
                ui.add_space(6.0);
            }
        });
    }

    fn show_security_scan(&mut self, ui: &mut egui::Ui) {
        ui.heading("🛡️ Security Scan");
        ui.add_space(10.0);
        ui.label("Scan your system for security threats and vulnerabilities.");
        ui.add_space(15.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("System Security Scan").strong());
                ui.add_space(12.0);

                if !self.scan_running {
                    if ui.button(egui::RichText::new("▶ Start Scan").size(16.0)).clicked() {
                        self.scan_running = true;
                        self.scan_progress = 0.0;
                        self.threats_found = 0;
                        self.add_activity_log("Security scan started".to_string(), 
                            "In progress".to_string(), "🛡️".to_string());
                    }
                } else {
                    ui.add(egui::ProgressBar::new(self.scan_progress).text("Scanning..."));
                    ui.add_space(8.0);
                    if ui.button("⏸ Pause").clicked() {
                        self.scan_running = false;
                    }
                    if ui.button("⏹ Stop").clicked() {
                        self.scan_running = false;
                        self.scan_progress = 0.0;
                        self.add_activity_log("Security scan stopped".to_string(), 
                            "Cancelled".to_string(), "⏹".to_string());
                    }
                }

                if self.scan_progress >= 1.0 {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("✓ Scan Complete").color(egui::Color32::from_rgb(100, 255, 100)).strong());
                    ui.label(format!("Threats Found: {}", self.threats_found));
                    if self.threats_found == 0 {
                        ui.label(egui::RichText::new("🛡️ System is secure!").color(egui::Color32::from_rgb(100, 255, 100)));
                    }
                }
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("SCAN OPTIONS").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.checkbox(&mut self.scan_encrypted, "Scan encrypted files");
                ui.checkbox(&mut self.scan_passwords, "Scan password database");
                ui.checkbox(&mut self.check_vulnerabilities, "Check for vulnerabilities");
                ui.checkbox(&mut self.deep_scan, "Deep scan (slower)");
            });
    }

    fn show_backup_restore(&mut self, ui: &mut egui::Ui) {
        ui.heading("💾 Backup & Restore");
        ui.add_space(10.0);
        ui.label("Create backups and restore your encrypted data.");
        ui.add_space(15.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Backup Configuration").strong());
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    ui.label("Backup Location:");
                    ui.text_edit_singleline(&mut self.backup_location);
                    if ui.button("📁 Browse").clicked() {
                        let dir_opt: Option<PathBuf> = rfd::FileDialog::new().pick_folder();
                        if let Some(dir) = dir_opt {
                            self.backup_location = dir.to_string_lossy().into_owned();
                        }
                    }
                });

                ui.add_space(12.0);
                ui.label(format!("Last Backup: {}", self.last_backup));

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("💾 Create Backup").size(14.0)).clicked() {
                        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        self.last_backup = timestamp.clone();
                        let backup_type = if self.backup_history.is_empty() || 
                            self.backup_history[0].1 == "Incremental" {
                            "Full Backup"
                        } else {
                            "Incremental"
                        };
                        let size = if backup_type == "Full Backup" {
                            "2.4 GB"
                        } else {
                            "150 MB"
                        };
                        self.backup_history.insert(0, (timestamp.clone(), backup_type.to_string(), size.to_string()));
                        self.add_activity_log(format!("Backup created: {}", backup_type), 
                            "Success".to_string(), "💾".to_string());
                    }
                    if ui.button(egui::RichText::new("📥 Restore Backup").size(14.0)).clicked() {
                        let path_opt: Option<PathBuf> = rfd::FileDialog::new()
                            .add_filter("Backup Files", &["bak", "backup"])
                            .pick_file();
                        if let Some(path) = path_opt {
                            self.add_activity_log(format!("Backup restored from: {}", path.to_string_lossy()), 
                                "Success".to_string(), "📥".to_string());
                        }
                    }
                });
            });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("BACKUP HISTORY").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                let mut backups_to_remove = Vec::new();
                let mut restore_actions = Vec::new();
                for (idx, (date, backup_type, size)) in self.backup_history.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(date).strong());
                        ui.label(backup_type);
                        ui.label(size);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("📥 Restore").clicked() {
                                restore_actions.push((date.clone(), backup_type.clone()));
                            }
                            if ui.small_button("🗑").clicked() {
                                backups_to_remove.push(idx);
                            }
                        });
                    });
                    ui.add_space(8.0);
                }
                // Process restore actions after the loop
                for (date, backup_type) in restore_actions {
                    self.add_activity_log(format!("Backup restored: {} ({})", date, backup_type), 
                        "Success".to_string(), "📥".to_string());
                }
                // Remove deleted backups
                backups_to_remove.sort_unstable();
                backups_to_remove.reverse();
                for idx in backups_to_remove {
                    if idx < self.backup_history.len() {
                        let backup_info = self.backup_history[idx].0.clone();
                        self.backup_history.remove(idx);
                        self.add_activity_log(format!("Backup deleted: {}", backup_info), 
                            "Success".to_string(), "🗑".to_string());
                    }
                }
            });
    }

    fn show_analytics(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 Analytics");
        ui.add_space(10.0);
        ui.label("View detailed statistics and usage analytics.");
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(30, 50, 70))
                .corner_radius(8.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_min_width(200.0);
                    ui.label(egui::RichText::new("Total Operations").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("1,247").size(32.0).strong());
                    ui.label(egui::RichText::new("+12% this month").size(11.0).color(egui::Color32::from_rgb(100, 255, 100)));
                });

            ui.add_space(10.0);

            egui::Frame::new()
                .fill(egui::Color32::from_rgb(50, 30, 70))
                .corner_radius(8.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_min_width(200.0);
                    ui.label(egui::RichText::new("Data Protected").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("2.4 GB").size(32.0).strong());
                    ui.label(egui::RichText::new("127 files").size(11.0).color(egui::Color32::GRAY));
                });

            ui.add_space(10.0);

            egui::Frame::new()
                .fill(egui::Color32::from_rgb(70, 50, 30))
                .corner_radius(8.0)
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.set_min_width(200.0);
                    ui.label(egui::RichText::new("Security Score").size(12.0).color(egui::Color32::GRAY));
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("98%").size(32.0).strong());
                    ui.label(egui::RichText::new("Excellent").size(11.0).color(egui::Color32::from_rgb(100, 255, 100)));
                });
        });

        ui.add_space(20.0);

        ui.label(egui::RichText::new("USAGE BREAKDOWN").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(10.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(30, 35, 45))
            .corner_radius(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                for (category, percentage) in &self.analytics_data {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(category).strong());
                        ui.add_space(10.0);
                        ui.add(egui::ProgressBar::new(*percentage / 100.0).show_percentage());
                        ui.label(format!("{:.1}%", percentage));
                    });
                    ui.add_space(8.0);
                }
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
                ui.checkbox(&mut self.require_password, "Require password on startup");
                ui.checkbox(&mut self.two_factor, "Enable two-factor authentication");
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