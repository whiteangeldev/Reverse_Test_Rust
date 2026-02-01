# 🚀 CyberVault Pro - Quick Start Guide

## Getting Started

### Prerequisites
- Rust toolchain (1.70+)
- macOS, Linux, or Windows

### Installation & Run

```bash
cd license_gate
cargo run
```

The application will compile and launch automatically.

---

## 🎮 Using the Application

### First Launch

1. The app opens in **Trial Mode**
2. You'll see the Dashboard with limited statistics
3. A license activation dialog appears
4. Navigation items for premium features are disabled (grayed out)

### Trial Mode Features
- ✅ View the Dashboard
- ✅ See overview of features
- ❌ Cannot access Encryption
- ❌ Cannot access Password Manager
- ❌ Limited Vault access

---

## 🔓 Activating the License

### Option 1: Use the Activation Dialog
1. The dialog appears automatically on first launch
2. Enter a license key in format: `XXXX-XXXX-XXXX`
3. Click "🔓 Activate"
4. If valid: All features unlock! 🎉
5. If invalid: Try again or continue in Trial mode

### Option 2: Activate Later
1. Click "Continue Trial" to explore first
2. When ready, click "🔓 Activate License" in the top bar
3. Or click "🔓 Upgrade Now" in the side panel

---

## 🎯 The Challenge

**Your Mission**: Find or generate a valid license key!

### What You Know:
- ✅ Key format: `XXXX-XXXX-XXXX` (14 chars with dashes)
- ✅ Dashes must be at positions 4 and 9
- ✅ Keys starting with "TEST" are rejected
- ✅ Keys containing "AAAA" are rejected
- ✅ Source code is available in `src/main.rs`

### Hints:
1. 🔍 Read the `verify_key()` function
2. 🧮 Analyze the `mix()` helper function
3. 🎲 Understand the state machine (3 states: 0, 1, 2)
4. 🎯 Target hash value is in the code
5. 💡 Check terminal output - there's debug info!

### Approaches:
- **Static Analysis**: Study the algorithm logic
- **Dynamic Analysis**: Run with test keys and observe
- **Brute Force**: Write a key generator
- **Reverse Engineering**: Work backwards from target hash
- **Binary Analysis**: Examine the compiled binary

---

## 📱 Navigation Guide

### Dashboard (📊)
- Security overview
- Statistics cards
- Premium features list
- Success flag (when unlocked)

### File Encryption (🔒) - *Requires License*
- Enter file path
- Configure encryption settings
- Simulate encryption process
- View progress and results

### Password Manager (🔑) - *Requires License*
- Add new credentials
- View stored passwords
- Manage entries (view/copy/delete)
- Secure password masking

### Secure Vault (🗄) - *Requires License*
- Browse encrypted files
- View file metadata
- Import/export functions
- File management actions

### Settings (⚙)
- Security preferences
- License information
- Application details
- About section

---

## 🏆 Success Criteria

You've successfully completed the challenge when:

1. ✅ License activation succeeds
2. ✅ Top bar shows "✓ Licensed" in green
3. ✅ All navigation items become accessible
4. ✅ Dashboard displays congratulations message
5. ✅ You receive the challenge flag

### The Flag

Look for this on the Dashboard after successful activation:

```
FLAG{cybervault_pro_license_cracked_successfully}
```

---

## 🛠 Development Tips

### Build for Analysis
```bash
# Debug build (with symbols)
cargo build

# Release build (optimized)
cargo build --release

# Check binary location
ls -lh target/debug/license_gate
ls -lh target/release/license_gate
```

### Debugging
```bash
# Run with output
cargo run

# The verify_key function prints debug info:
# "DEBUG x = 0x85FD063D" (or whatever hash result)
```

### Reverse Engineering Tools
- **Ghidra**: Decompile the binary
- **IDA Pro**: Static analysis
- **GDB/LLDB**: Dynamic debugging
- **Binary Ninja**: Visual analysis
- **strings**: Extract readable text
- **objdump**: Disassemble sections

---

## 💡 Learning Resources

### Understanding the Code

**Key Validation Flow:**
```rust
1. Check length (must be 14)
2. Check dash positions (4 and 9)
3. Reject known patterns (TEST*, *AAAA*)
4. Initialize hash: x = 0x12345678
5. Process each byte through state machine
6. Compare final hash to target
```

**State Machine:**
```
State 0: Apply mix() function → State 1
State 1: XOR with 0x11111111 → State 2
State 2: Increment position → State 0
```

**Mix Function:**
```rust
fn mix(x, byte):
    x ^= byte
    x = rotate_left(x, 5)
    x = x + 0x9E3779B9
    return x ^ 0xA5A55A5A
```

---

## ❓ FAQ

**Q: Can I view the source code?**
A: Yes! It's in `src/main.rs` - part of the challenge!

**Q: Is there a valid key in the code?**
A: No, you need to find or generate one.

**Q: How many valid keys exist?**
A: Many! The hash function creates multiple valid inputs.

**Q: Can I modify the code?**
A: You can, but the challenge is to crack the original binary.

**Q: What if I get stuck?**
A: 
1. Look at the debug output in terminal
2. Try small inputs first
3. Understand each operation step-by-step
4. Consider writing a solver program

**Q: Is this realistic?**
A: The algorithm is simplified for educational purposes, but the concepts apply to real software protection.

---

## 🎓 Next Steps

After completing this challenge:

1. **Try harder variants**: Modify the hash to increase difficulty
2. **Add features**: Extend the UI with new modules
3. **Study protections**: Research real software licensing
4. **Learn cryptography**: Study proper key validation
5. **Practice more**: Find other reverse engineering challenges

---

## 📞 Support

If you find bugs or have questions:
- Check `FEATURES.md` for detailed documentation
- Read `README.md` for project overview
- Study the source code in `src/main.rs`

---

**Good luck, and enjoy the challenge!** 🔐✨

Remember: The journey of understanding is just as valuable as finding the flag!
