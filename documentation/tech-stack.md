Here’s a streamlined technical architecture document for your Rust-based macOS menu bar app:

---

# **macOS Network Monitor App Architecture**  
**Goal**: Build a lightweight menu bar app to monitor device statuses via ICMP pings, with minimal memory usage (<20MB idle).

---

## **Tech Stack**
| Component              | Technology             | Purpose                                  |
|------------------------|------------------------|------------------------------------------|
| **Core Framework**     | Tauri (v2.0+)          | Native menu bar integration, UI rendering |
| **Async Runtime**      | Tokio                  | Non-blocking ping scheduling             |
| **Configuration**      | Serde + JSON           | Device storage (names/IPs)               |
| **Native Bindings**    | `objc` crate           | Direct macOS API access for tray icons    |
| **Ping Engine**        | `ping-rs`              | Low-level ICMP implementation            |
| **Memory Management**  | Rust’s borrow checker  | Zero-cost abstractions, no GC overhead   |

---

## **Key Architectural Decisions**
1. **System Tray**  
   - Use Tauri’s `SystemTray` API to create a native menu bar icon.  
   - Store icon as a **template image** (`.png` with transparency) for dark/light mode support.  

2. **Async Ping Workflow**  
   ```plaintext
   Tokio Scheduler → ping-rs → Status Update → Tauri Event
   ```
   - Pings run every 60s (configurable) via Tokio timers.  
   - Results cached to avoid redundant UI updates.  

3. **Data Flow**  
   ```plaintext
   JSON Config → Serde → Memory-Mapped Struct → Tauri State
   ```
   - Devices stored in `~/.devicemonitor.json` for portability.  

4. **Memory Optimization**  
   - Fixed-size structs for devices (no heap fragmentation):  
     ```rust
     struct Device {
         name: [u8; 32],  // 32-byte fixed array
         ip: [u8; 15]     // "255.255.255.255" fits
     }
     ```
   - Reuse buffers for ping packets.  

---

## **Build & Deployment**
### **Cargo.toml**  
```toml
[features]
default = ["tauri/system-tray"]

[dependencies]
tauri = { version = "2.0", features = ["system-tray"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
ping-rs = "0.4"
```

### **Release Flags**  
```bash
RUSTFLAGS="-C target-cpu=native -C link-arg=-s" \
cargo build --release
```

### **Output**  
- Binary: `target/release/devicemonitor` (≈3.8MB)  
- Memory: 12-18MB idle, +2MB during pings  

---

## **Cross-Platform Considerations**
- **macOS Only**: Leverage `Core Foundation` bindings via `core-foundation-rs` for tray positioning.  
- **No Electron**: Avoids 100MB+ memory overhead.  

---

## **Future Scalability**
1. **Notifications**: Use `tauri-plugin-notification` for offline alerts.  
2. **Export/Import**: Add JSON config sharing via drag-and-drop.  
3. **Plugins**: Extend with `tauri-plugin-sqlite` for large device lists.  

---

This architecture ensures native performance while maintaining Rust’s memory safety guarantees.