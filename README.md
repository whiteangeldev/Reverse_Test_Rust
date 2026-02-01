# Reverse_Test_Rust

## CyberVault Pro - Secure Data Manager

A reverse engineering challenge featuring a production-ready license gate system built with Rust and egui.

### Overview

**CyberVault Pro** is a fictional premium security application that demonstrates a realistic software license activation system. The application features a polished, professional UI with multiple security-focused features that unlock upon successful license validation.

### Features

#### 🔓 **Unlocked Features (with valid license)**
- **📊 Dashboard** - Security overview and statistics
- **🔒 File Encryption** - AES-256 file encryption interface
- **🔑 Password Manager** - Secure password storage and management
- **🗄 Secure Vault** - Protected document storage with metadata
- **⚙ Settings** - Application configuration and license info

#### ⚠ **Trial Mode Limitations**
Without a valid license, users can explore the dashboard but cannot access premium features like encryption, password management, or full vault access.

### Technical Stack

- **Language:** Rust (Edition 2024)
- **GUI Framework:** egui/eframe 0.33.3
- **Theme:** Dark mode with modern color scheme
- **Architecture:** Single-binary desktop application

### Running the Application

```bash
cd license_gate
cargo run
```

### The Challenge

This is a reverse engineering challenge. The goal is to:

1. Analyze the license verification algorithm
2. Understand the key format and validation logic
3. Find or generate a valid license key
4. Unlock all premium features

**Key Format:** `XXXX-XXXX-XXXX` (14 characters including dashes)

### Application Structure

```
license_gate/
├── src/
│   └── main.rs          # Complete application (GUI + license verification)
├── Cargo.toml           # Dependencies
└── target/              # Build artifacts
```

### UI Highlights

- **Professional Branding:** Custom color scheme and modern typography
- **Multi-Panel Layout:** Top bar, side navigation, and main content area
- **Feature Cards:** Color-coded sections for different functionalities
- **Modal Dialogs:** License activation window overlay
- **Responsive Design:** Resizable with minimum size constraints
- **Rich Content:** Statistics, grids, forms, and scrollable areas
- **State Management:** Complex app state with multiple tabs and features

### License Verification

The license verification system includes:
- Custom hash-based validation algorithm
- State machine pattern for key processing
- Format validation (length, separators)
- Attempt tracking and error handling
- Anti-pattern detection (blocks common test patterns)

### Success Criteria

Upon successful license activation, you'll receive:
- Full access to all features
- A congratulatory message on the dashboard
- The challenge completion flag

### Development Notes

- Zero warnings on compilation (all deprecations fixed)
- Production-ready code quality
- Modern Rust idioms and best practices
- Comprehensive UI with realistic features

### Screenshots

The application features:
- 🎨 Dark theme with custom color palette
- 📱 Multi-tab navigation system
- 🔐 Secure license activation dialog
- 📊 Professional dashboard with statistics cards
- 🛠 Fully functional settings panel

---

**Challenge Difficulty:** Intermediate
**Category:** Reverse Engineering / Cryptography
**Target:** Desktop Application Binary

Good luck, and happy reversing! 🔓