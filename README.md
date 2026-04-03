# 🦀 Oxidust 🌬️

**The high-performance, cross-platform system optimizer for the modern era.**

[](https://opensource.org/licenses/MIT)
[](https://www.rust-lang.org/)
[](https://tauri.app/)

**Oxidust** is a lightning-fast, transparent, and multi-layered system cleaner designed to solve the "bloatware" problem on Windows, macOS, and Linux. Built with **Rust** and **Tauri**, it focuses on deep-system purging, real-time disk visualization, and remote fleet management.

-----

## 🚀 Why Oxidust?

Legacy tools like CCleaner have become bloated, invasive, and opaque. **Oxidust** flips the script by offering an "Invisible Architecture" that is ultra-lightweight and community-driven.

### Key Innovations:

  * **Interactive TreeMap (WinDirStat Style):** Don't just check boxes; see your disk. A high-performance Canvas-based map helps you visualize exactly where the space is going in real-time.
  * **The Ghost-Hunter Engine:** Detects "Ghost Files"—deleted file handles still held by active processes (common in Nginx/Docker environments)—that cause "disk full" errors without obvious reasons.
  * **Layered Rule Engine:** Preset-based cleaning logic separated by **OS Base** (Win/Mac/Linux), **Distribution** (Ubuntu/Arch/Fedora), and **App Specifics** (Docker/Browsers/IDEs).
  * **Server-Grade Precision:** Advanced cleaning for Docker layers, orphaned volumes, database WAL logs, and Inode-heavy directories.
  * **Unified Web UI:** Manage and trigger cleanup across a fleet of remote servers or workstations via a secure, edge-powered dashboard.

-----

## 🛠️ Tech Stack

  * **Core:** [Rust](https://www.rust-lang.org/) (High-concurrency scanning & memory safety)
  * **UI:** [Tauri](https://tauri.app/) (Native-speed GUI with a tiny footprint)
  * **Visualization:** WebGL/Canvas (Smooth interaction with millions of file nodes)
  * **Edge Backend:** [Cloudflare Workers](https://workers.cloudflare.com/) & [D1](https://developers.cloudflare.com/d1/) (Rule synchronization & analytics)

-----

## 📂 Project Structure (The Layered Approach)

Oxidust uses a unique hierarchy for its cleaning rules:

1.  **OS Base:** Root-level cleaning for Windows (Registry/Temp), macOS (Library), and Linux (\~/.cache).
2.  **Distribution:** Tailored rules for package managers (`apt`, `pacman`, `dnf`, `brew`).
3.  **Application Presets:** Dedicated rules for specific software (Chrome, Spotify, Docker, VS Code).

-----

## 💰 Roadmap & Sustainability

**Oxidust** is committed to being a powerful tool for everyone while maintaining a sustainable development path:

  * **Community Edition (Free):** A fully-featured Local GUI and a massive library of open-source presets. We rely on community donations and sponsorships to keep the core project free forever.
  * **Advanced Presets (Paid):** Expert-level rule sets for specialized environments (Game Dev, Video Production, High-Traffic Databases).
  * **Enterprise License (Paid):** Designed for businesses requiring centralized fleet management, automated purge policies, audit logs, and priority support.

-----

## 🤝 Contributing & Support

We welcome contributions from the community\! Whether you are a Rustacean, a UI/UX designer, or a SysAdmin with a great cleanup preset, we need your help.

  * **Sponsor the Project:** Support our infrastructure costs and development time via GitHub Sponsors or Ko-fi.
  * **Submit a Preset:** Contribute to our `presets/community` folder and help others keep their systems clean.

-----

## 🛡️ Privacy & Safety

  * **Dry Run First:** Always see exactly what will be deleted before you commit.
  * **Safety Levels:** Files are categorized from "Safe" (Cache) to "Dangerous" (System Configs).
  * **No Data Harvesting:** Oxidust does not track your files. Your system's data stays on your system.

-----

### **Let's turn the "Dust" back into "Oxide."**

*Ready to reclaim your disk space? Clone the repo and let's get started.*

`git clone https://github.com/your-username/oxidust.git`
`cd oxidust`
`cargo tauri dev`

-----

### **Follow the Journey**

[Twitter/X] | [Discord] | [Web Dashboard]
