use crate::types::{format_bytes, *};
use slint::{Color, SharedString};

fn hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::from_rgb_u8(r, g, b)
}

pub fn get_os_blocks(os: &str) -> Vec<DiskBlock> {
    let blocks = match os {
        "linux" => vec![
            ("Docker overlay2", "/var/lib/docker/overlay2", 13421773, "Docker", "dangerous", "#0891b2"),
            ("Nginx Access Logs", "/var/log/nginx/access.log", 4404016, "Logs", "moderate", "#d97706"),
            ("Chromium Cache", "~/.cache/chromium", 3565158, "Caches", "safe", "#059669"),
            ("Spotify Offline Cache", "~/.local/share/Spotify", 2202009, "Caches", "safe", "#10b981"),
            ("Kernel Headers Backup", "/usr/src/kernels", 5767168, "System", "dangerous", "#be123c"),
            ("Nginx Ghost Descriptor", "/var/log/nginx/access.log.1 (deleted)", 4718592, "Ghost", "moderate", "#ea580c"),
            ("Apt Package Cache", "/var/cache/apt/archives", 2936012, "System", "moderate", "#e11d48"),
            ("User Home Temp", "~/downloads/tmp", 1572864, "UserFiles", "safe", "#2563eb"),
        ],
        "macos" => vec![
            ("Xcode DerivedData", "~/Library/Developer/Xcode/DerivedData", 8599604, "Caches", "safe", "#059669"),
            ("Docker Virtual Disk", "~/Library/Containers/com.docker.docker", 15205001, "Docker", "dangerous", "#0891b2"),
            ("CoreSymbolication Cache", "~/Library/Caches/com.apple.coresymbolicationd", 5452594, "Caches", "safe", "#10b981"),
            ("DiagnosticReports Logs", "~/Library/Logs/DiagnosticReports", 2936012, "Logs", "moderate", "#d97706"),
            ("Safari Ghost Descriptor", "~/Library/Caches/Safari/Cache.db (deleted)", 3984589, "Ghost", "moderate", "#ea580c"),
            ("Homebrew Downloads", "~/Library/Caches/Homebrew", 4090072, "System", "moderate", "#e11d48"),
            ("User Trash Can", "~/.Trash", 2306866, "UserFiles", "safe", "#2563eb"),
        ],
        "windows" => vec![
            ("Docker windowsfilter", "C:\\ProgramData\\Docker\\windowsfilter", 11744007, "Docker", "dangerous", "#0891b2"),
            ("Windows Update Cache", "C:\\Windows\\SoftwareDistribution\\Download", 4718592, "System", "dangerous", "#be123c"),
            ("Windows Event Logs", "C:\\Windows\\System32\\Winevt\\Logs", 4090072, "Logs", "moderate", "#d97706"),
            ("Spotify Local AppData", "C:\\Users\\AppData\\Local\\Spotify", 2936012, "Caches", "safe", "#059669"),
            ("Svchost Ghost Handle", "C:\\Windows\\Temp\\srv_socket_log.tmp (deleted)", 5347894, "Ghost", "moderate", "#ea580c"),
            ("User Recycler", "C:\\$Recycle.Bin", 3774874, "UserFiles", "safe", "#2563eb"),
            ("Chrome Browser Cache", "C:\\Users\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", 3355444, "Caches", "safe", "#10b981"),
        ],
        _ => vec![],
    };

    blocks
        .into_iter()
        .enumerate()
        .map(|(i, (name, path, size, cat, safety, color))| DiskBlock {
            id: SharedString::from(format!("{}_{}", os, i)),
            name: SharedString::from(name),
            path: SharedString::from(path),
            size,
            category: SharedString::from(cat),
            safety: SharedString::from(safety),
            color: hex_color(color),
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        })
        .collect()
}

pub fn get_os_presets(os: &str) -> Vec<RulePreset> {
    let presets = match os {
        "linux" => vec![
            ("Docker Layers Purge", "Clears dangling container layers and unused Docker build caches.", "app", "linux", "none", "/var/lib/docker/overlay2", 13421773, "dangerous"),
            ("Systemd Journal Log Rotation", "Archived journald binary files older than 7 days.", "os", "linux", "none", "/var/log/nginx/access.log", 4404016, "moderate"),
            ("Chromium Browser Cache", "Removes local indexed database indices and offline page icons.", "app", "linux", "none", "~/.cache/chromium", 3565158, "safe"),
            ("Spotify Streaming Cache", "Wipes local encodings of offline tracks and album thumbnails.", "app", "linux", "none", "~/.local/share/Spotify", 2202009, "safe"),
            ("Orphaned Kernel Backups", "Redundant kernel symbols and old device module frameworks.", "os", "linux", "none", "/usr/src/kernels", 5767168, "dangerous"),
            ("Apt Package Cellar Cache", "Cached .deb installation packages from apt-get commands.", "dist", "linux", "ubuntu", "/var/cache/apt/archives", 2936012, "moderate"),
            ("System Temp Directories", "Cleans discarded execution locks from the root temporal folder.", "os", "linux", "none", "~/downloads/tmp", 1572864, "safe"),
        ],
        "macos" => vec![
            ("Xcode Derived Data Assets", "Intermediate compile logs and module index caches.", "app", "macos", "none", "~/Library/Developer/Xcode/DerivedData", 8599604, "safe"),
            ("Docker Overlay Layers", "Removes dangling layer builds from local container services.", "app", "macos", "none", "~/Library/Containers/com.docker.docker", 15205001, "dangerous"),
            ("CoreSymbolication Caches", "System diagnostic crash caches.", "os", "macos", "none", "~/Library/Caches/com.apple.coresymbolicationd", 5452594, "safe"),
            ("Apple Diagnostic Crashlogs", "Archived diagnostic telemetry tables.", "os", "macos", "none", "~/Library/Logs/DiagnosticReports", 2936012, "moderate"),
            ("Homebrew Cellar Cache", "Purges unlinked source tarballs and stale brew bottles.", "dist", "macos", "brew", "~/Library/Caches/Homebrew", 4090072, "moderate"),
            ("Mac User Trash Volume", "Empties the local user directory Trash collection.", "os", "macos", "none", "~/.Trash", 2306866, "safe"),
        ],
        "windows" => vec![
            ("Docker Storage Filter", "Cleans cached storage layers and unused Docker network grids.", "app", "windows", "none", "C:\\ProgramData\\Docker\\windowsfilter", 11744007, "dangerous"),
            ("Windows Update Installer Cabinets", "Redundant update payloads.", "os", "windows", "none", "C:\\Windows\\SoftwareDistribution\\Download", 4718592, "dangerous"),
            ("Windows Application Event Logs", "Event viewer diagnostic trace logs.", "os", "windows", "none", "C:\\Windows\\System32\\Winevt\\Logs", 4090072, "moderate"),
            ("Spotify Album Caches", "Wipes cached metadata and offline catalog listings.", "app", "windows", "none", "C:\\Users\\AppData\\Local\\Spotify", 2936012, "safe"),
            ("Windows Recycler bin", "Deletes file structures locked inside the Recycle bin.", "os", "windows", "none", "C:\\$Recycle.Bin", 3774874, "safe"),
            ("Chrome Indexed caches", "Removes service worker caches and local browser memory indices.", "app", "windows", "none", "C:\\Users\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", 3355444, "safe"),
        ],
        _ => vec![],
    };

    presets
        .into_iter()
        .enumerate()
        .map(|(i, (name, desc, cat, os_val, dist, target, size, safety))| RulePreset {
            id: SharedString::from(format!("{}_{}", os, i)),
            name: SharedString::from(name),
            description: SharedString::from(desc),
            category: SharedString::from(cat),
            os: SharedString::from(os_val),
            dist: SharedString::from(dist),
            target: SharedString::from(target),
            size_bytes: size,
            size_text: SharedString::from(format_bytes(size)),
            safety: SharedString::from(safety),
            checked: safety == "safe",
        })
        .collect()
}

pub fn get_os_ghosts(os: &str) -> Vec<GhostFile> {
    let ghosts = match os {
        "linux" => vec![
            ("ghost_lin_0", 4092, "nginx", "/var/log/nginx/access.log.1 (deleted)", 4718592),
        ],
        "macos" => vec![
            ("ghost_mac_0", 501, "Safari", "~/Library/Caches/Safari/Cache.db (deleted)", 3984589),
        ],
        "windows" => vec![
            ("ghost_win_0", 1420, "svchost.exe", "C:\\Windows\\Temp\\srv_socket_log.tmp (deleted)", 5347894),
        ],
        _ => vec![],
    };

    ghosts
        .into_iter()
        .map(|(id, pid, process, path, size)| GhostFile {
            id: SharedString::from(id),
            pid,
            process: SharedString::from(process),
            path: SharedString::from(path),
            size,
            released: false,
        })
        .collect()
}

pub fn get_initial_fleet_nodes() -> Vec<FleetNode> {
    vec![
        FleetNode {
            id: SharedString::from("fleet_node_0"),
            name: SharedString::from("prod-nginx-lb-01"),
            ip: SharedString::from("10.128.0.12"),
            os: SharedString::from("linux"),
            status: SharedString::from("online"),
            disk_usage: 89,
            disk_total: 524288000,
            disk_free: 57671680,
            ghost_files_count: 2,
            last_scan: SharedString::from("12h ago"),
        },
        FleetNode {
            id: SharedString::from("fleet_node_1"),
            name: SharedString::from("k8s-docker-worker-4"),
            ip: SharedString::from("10.128.1.15"),
            os: SharedString::from("linux"),
            status: SharedString::from("online"),
            disk_usage: 76,
            disk_total: 262144000,
            disk_free: 62914560,
            ghost_files_count: 0,
            last_scan: SharedString::from("2h ago"),
        },
        FleetNode {
            id: SharedString::from("fleet_node_2"),
            name: SharedString::from("alice-workstation-imac"),
            ip: SharedString::from("192.168.1.101"),
            os: SharedString::from("macos"),
            status: SharedString::from("online"),
            disk_usage: 45,
            disk_total: 1048576000,
            disk_free: 576716800,
            ghost_files_count: 0,
            last_scan: SharedString::from("3 days ago"),
        },
        FleetNode {
            id: SharedString::from("fleet_node_3"),
            name: SharedString::from("legacy-win-db-08"),
            ip: SharedString::from("10.142.12.8"),
            os: SharedString::from("windows"),
            status: SharedString::from("warning"),
            disk_usage: 94,
            disk_total: 1048576000,
            disk_free: 62914560,
            ghost_files_count: 1,
            last_scan: SharedString::from("Never"),
        },
    ]
}
