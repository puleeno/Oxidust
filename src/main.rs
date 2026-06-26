use std::cell::RefCell;
use std::rc::Rc;
use chrono::Local;
use slint::SharedString;

mod types;
use types::*;
mod data;

struct AppState {
    active_os: String,
    safety_level: String,
    blocks: Vec<DiskBlock>,
    presets: Vec<RulePreset>,
    ghosts: Vec<GhostFile>,
    fleet_nodes: Vec<FleetNode>,
    logs: Vec<LogEntry>,
    chat_history: Vec<ChatMessage>,
    reclaimed_bytes: i32,
    total_capacity: i32,
    initial_used: i32,
}

impl AppState {
    fn new() -> Self {
        let logs = vec![
            log_entry("info", "[OK] Oxidust Core v2.4.0 initialized"),
            log_entry("info", "[INFO] System sensors calibrated"),
        ];
        let chat_history = vec![ChatMessage {
            role: SharedString::from("assistant"),
            text: SharedString::from("Hello! I am the **Oxidust AI Systems Advisor**.\n\nI am optimized to help you perform server-grade space purging, analyze ghost handles, and formulate highly technical Rust-speed system optimizations.\n\nAsk me a question below, or trigger a **Comprehensive Storage Audit** to generate a diagnostic performance report!"),
        }];
        Self {
            active_os: "linux".to_string(),
            safety_level: "moderate".to_string(),
            blocks: vec![],
            presets: vec![],
            ghosts: vec![],
            fleet_nodes: data::get_initial_fleet_nodes(),
            logs,
            chat_history,
            reclaimed_bytes: 0,
            total_capacity: 500 * 1024 * 1024,
            initial_used: 384 * 1024 * 1024,
        }
    }

    fn load_os(&mut self, os: &str) {
        self.active_os = os.to_string();
        self.blocks = data::get_os_blocks(os);
        self.presets = data::get_os_presets(os);
        self.ghosts = data::get_os_ghosts(os);
        compute_treemap_layout(&mut self.blocks, 600.0, 350.0);

        self.add_log("ok", &format!("[OK] Shifted kernel compiler targeting to: {}", os.to_uppercase()));
        self.add_log("info", &format!("[INFO] Re-compiled and loaded OS-specific preset checklists ({} loaded)", self.presets.len()));
        self.add_log("info", "[INFO] TreeMap cache rebuilt. Storage maps synchronized.");
    }

    fn apply_safety(&mut self, level: &str) {
        self.safety_level = level.to_string();
        for p in &mut self.presets {
            p.checked = match level {
                "safe" => p.safety.as_str() == "safe",
                "moderate" => p.safety.as_str() == "safe" || p.safety.as_str() == "moderate",
                _ => true,
            };
        }
        self.add_log("exec", &format!("[EXEC] Adjusted Maximum Safety Threshold to: [{}]", level.to_uppercase()));
        self.add_log("ok", "[OK] Re-mapped preset checklists to align with safety matrix thresholds.");
    }

    fn total_reclaimable_selected(&self) -> i32 {
        self.presets.iter().filter(|p| p.checked).map(|p| p.size_bytes).sum()
    }

    fn used_percentage(&self) -> i32 {
        let current_used = (self.initial_used - self.reclaimed_bytes).max(0);
        ((current_used as f64 / self.total_capacity as f64) * 100.0) as i32
    }

    fn add_log(&mut self, level: &str, text: &str) {
        self.logs.push(LogEntry {
            level: SharedString::from(level),
            text: SharedString::from(text),
        });
    }

    fn time_str() -> String {
        Local::now().format("%H:%M:%S").to_string()
    }
}

fn log_entry(level: &str, text: &str) -> LogEntry {
    LogEntry {
        level: SharedString::from(level),
        text: SharedString::from(text),
    }
}

fn compute_treemap_layout(blocks: &mut [DiskBlock], width: f32, height: f32) {
    if blocks.is_empty() {
        return;
    }

    let mut sorted: Vec<usize> = (0..blocks.len()).collect();
    sorted.sort_by(|&a, &b| blocks[b].size.cmp(&blocks[a].size));

    struct LayoutItem {
        idx: usize,
        size: i32,
    }
    let items: Vec<LayoutItem> = sorted.iter().map(|&i| LayoutItem { idx: i, size: blocks[i].size }).collect();

    fn layout(items: &[LayoutItem], blocks: &mut [DiskBlock], x: f32, y: f32, w: f32, h: f32) {
        if items.is_empty() {
            return;
        }
        if items.len() == 1 {
            let idx = items[0].idx;
            blocks[idx].x = x;
            blocks[idx].y = y;
            blocks[idx].w = w;
            blocks[idx].h = h;
            return;
        }

        let total_size: i32 = items.iter().map(|it| it.size).sum();
        if total_size == 0 {
            return;
        }

        let mut accumulated: i32 = 0;
        let mut split_idx = 1;
        for (i, item) in items.iter().enumerate() {
            accumulated += item.size;
            if accumulated >= total_size / 2 && i < items.len() - 1 {
                split_idx = i + 1;
                break;
            }
        }

        let (left_items, right_items) = items.split_at(split_idx);
        let left_size: i32 = left_items.iter().map(|it| it.size).sum();
        let ratio = left_size as f64 / total_size as f64;

        if w > h {
            let lw = (w as f64 * ratio) as f32;
            layout(left_items, blocks, x, y, lw, h);
            layout(right_items, blocks, x + lw, y, w - lw, h);
        } else {
            let lh = (h as f64 * ratio) as f32;
            layout(left_items, blocks, x, y, w, lh);
            layout(right_items, blocks, x, y + lh, w, h - lh);
        }
    }

    layout(&items, blocks, 0.0, 0.0, width, height);
}

fn vec_to_model<T: Clone + 'static>(vec: Vec<T>) -> slint::ModelRc<T> {
    slint::ModelRc::new(slint::VecModel::from(vec))
}

fn update_app(app: &AppWindow, state: &AppState) {
    app.set_blocks(vec_to_model(state.blocks.clone()));
    app.set_presets(vec_to_model(state.presets.clone()));
    app.set_ghosts(vec_to_model(state.ghosts.clone()));
    app.set_fleet_nodes(vec_to_model(state.fleet_nodes.clone()));
    app.set_logs(vec_to_model(state.logs.clone()));
    app.set_chat_history(vec_to_model(state.chat_history.clone()));

    let reclaimable = state.total_reclaimable_selected();
    app.set_reclaimed_text(SharedString::from(format_bytes(state.reclaimed_bytes)));
    app.set_used_percentage(state.used_percentage());

    app.set_total_reclaimable(reclaimable);
    app.set_reclaimable_text(SharedString::from(format_bytes(reclaimable)));
    app.set_active_os(SharedString::from(&state.active_os));
    app.set_safety_level(SharedString::from(&state.safety_level));
}

fn main() {
    let app = AppWindow::new().unwrap();
    let app_weak = app.as_weak();
    let state = Rc::new(RefCell::new(AppState::new()));

    // Initial data load
    {
        let mut s = state.borrow_mut();
        s.load_os("linux");
        update_app(&app, &s);
    }

    // Tab changed
    app.on_tab_changed({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |tab| {
            let app = app_handle.upgrade().unwrap();
            let s = state.borrow();
            let _ = tab;
            update_app(&app, &s);
        }
    });

    // Toggle preset
    app.on_toggle_preset({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |id| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let id_str = id.as_str();
            if let Some(p) = s.presets.iter_mut().find(|p| p.id.as_str() == id_str) {
                p.checked = !p.checked;
            }
            update_app(&app, &s);
        }
    });

    // Select all / deselect all
    app.on_select_all({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |v| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            for p in &mut s.presets {
                p.checked = v;
            }
            update_app(&app, &s);
        }
    });

    // Change OS
    app.on_change_os({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |os| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            s.load_os(os.as_str());
            update_app(&app, &s);
        }
    });

    // Change safety
    app.on_change_safety({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |level| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            s.apply_safety(level.as_str());
            update_app(&app, &s);
        }
    });

    // Run dry scan
    app.on_run_dry_scan({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move || {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let ts = AppState::time_str();
            let active_os = s.active_os.clone();
            let safety_level = s.safety_level.clone();
            let block_len = s.blocks.len();
            let total_space: i32 = s.blocks.iter().map(|b| b.size).sum();
            let ghost_count = s.ghosts.iter().filter(|g| !g.released).count();

            s.add_log("exec", &format!("[{}] launching Dry-Run file diagnostics scan...", ts));
            s.add_log("cmd", &format!("$ oxidust core scan --os={} --safety={}", active_os, safety_level));
            s.add_log("ok", "[OK] Attached filesystem kernel sensor to absolute roots...");
            s.add_log("info", "[INFO] Auditing standard caches, local indexes, and dangling container registers...");
            s.add_log("info", &format!("[INFO] Discovered {} storage density blocks occupying system volume", block_len));
            s.add_log("lock", &format!("[LOCK] Found {} unlinked open process handles (phantom file structures)", ghost_count));
            s.add_log("success", &format!("[SUCCESS] Scan complete! Identified reclaimable threshold of {} across all policy checklists.", format_bytes(total_space)));

            update_app(&app, &s);
        }
    });

    // Purge checked
    app.on_purge_checked({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move || {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let ts = AppState::time_str();
            let checked_presets: Vec<usize> = s.presets.iter().enumerate().filter(|(_, p)| p.checked).map(|(i, _)| i).collect();

            if checked_presets.is_empty() {
                return;
            }

            s.add_log("exec", &format!("[{}] Initiating absolute purge sequence on checked rule nodes...", ts));
            s.add_log("cmd", "$ oxidust system clean --policy=active --force");

            let mut space_reclaimed: i32 = 0;

            // Remove matching blocks from treemap
            let targets: Vec<String> = checked_presets.iter().map(|&i| s.presets[i].target.as_str().to_string()).collect();
            s.blocks.retain(|b| {
                let matched = targets.iter().any(|t| b.path.as_str().starts_with(t) || t.starts_with(b.path.as_str()));
                if matched {
                    space_reclaimed += b.size;
                }
                !matched
            });
            compute_treemap_layout(&mut s.blocks, 600.0, 350.0);

            // Clear checked presets
            for &i in &checked_presets {
                s.presets[i].size_bytes = 0;
                s.presets[i].checked = false;
            }

            s.reclaimed_bytes += space_reclaimed;
            s.add_log("ok", "[OK] Wiped metadata headers and trimmed sector blocks.");
            s.add_log("success", &format!("[SUCCESS] Successfully released {} checked target rules.", checked_presets.len()));
            s.add_log("success", &format!("[SUCCESS] System disk reclaimed: +{} returned to root index.", format_bytes(space_reclaimed)));

            update_app(&app, &s);
        }
    });

    // Block clicked (purge single block)
    app.on_block_clicked({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |id| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            if let Some(idx) = s.blocks.iter().position(|b| b.id.as_str() == id.as_str()) {
                let block = s.blocks[idx].clone();
                let ts = AppState::time_str();
                s.add_log("exec", &format!("[{}] [EXEC] Granular direct purge triggered on block: {}", ts, block.name));
                s.add_log("cmd", &format!("$ rm -rf {}", block.path));

                s.blocks.remove(idx);
                compute_treemap_layout(&mut s.blocks, 600.0, 350.0);

                // Clear matched presets
                for p in &mut s.presets {
                    if block.path.as_str().starts_with(p.target.as_str()) || p.target.as_str().starts_with(block.path.as_str()) {
                        p.size_bytes = 0;
                        p.checked = false;
                    }
                }

                // Release ghost if applicable
                if block.category.as_str() == "Ghost" {
                    for g in &mut s.ghosts {
                        if block.path.as_str().starts_with(g.path.as_str()) || g.path.as_str().starts_with(block.path.as_str()) {
                            g.released = true;
                        }
                    }
                }

                s.reclaimed_bytes += block.size;
                s.add_log("ok", "[OK] Raw blocks purged cleanly.");
                s.add_log("success", &format!("[SUCCESS] Reclaimed {} from node sector.", format_bytes(block.size)));

                update_app(&app, &s);
            }
        }
    });

    // Release ghost
    app.on_release_ghost({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |id| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let idx = s.ghosts.iter().position(|g| g.id.as_str() == id.as_str());
            if let Some(i) = idx {
                let pid = s.ghosts[i].pid;
                let process = s.ghosts[i].process.clone();
                let size = s.ghosts[i].size;
                s.ghosts[i].released = true;

                let ts = AppState::time_str();
                s.add_log("lock", &format!("[{}] [LOCK] Closing locking descriptor for PID {} ({})", ts, pid, process));
                s.add_log("cmd", &format!("$ lsof -p {} | grep deleted", pid));

                s.blocks.retain(|b| b.category.as_str() != "Ghost");
                compute_treemap_layout(&mut s.blocks, 600.0, 350.0);

                s.reclaimed_bytes += size;
                s.add_log("ok", "[OK] Truncated socket handle fd. OS garbage collector cleared locked sectors.");
                s.add_log("success", &format!("[SUCCESS] Released Open Socket Lock. Reclaimed +{} phantom space.", format_bytes(size)));

                update_app(&app, &s);
            }
        }
    });

    // Kill process
    app.on_kill_process({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |pid| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let ts = AppState::time_str();

            let total_size: i32 = s.ghosts.iter().filter(|g| g.pid == pid).map(|g| g.size).sum();
            for g in &mut s.ghosts {
                if g.pid == pid {
                    g.released = true;
                }
            }

            s.add_log("critical", &format!("[{}] [CRITICAL] Issuing SIGKILL (kill -9) to daemon process PID {}", ts, pid));
            s.add_log("cmd", &format!("$ kill -9 {}", pid));

            s.blocks.retain(|b| b.category.as_str() != "Ghost");
            compute_treemap_layout(&mut s.blocks, 600.0, 350.0);

            s.reclaimed_bytes += total_size;
            s.add_log("ok", &format!("[OK] Process thread {} terminated. All locked kernel files descriptors closed.", pid));
            s.add_log("success", &format!("[SUCCESS] Freed locked system thread sockets. Reclaimed +{} memory-buffered disk blocks.", format_bytes(total_size)));

            update_app(&app, &s);
        }
    });

    // Scan ghosts (same as dry scan)
    app.on_scan_ghosts({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move || {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            s.add_log("exec", "[EXEC] Ghost-Hunter triggered file descriptor audit...");
            let ghost_count = s.ghosts.iter().filter(|g| !g.released).count();
            s.add_log("info", &format!("[INFO] Discovered {} ghost file handles held by active processes.", ghost_count));
            update_app(&app, &s);
        }
    });

    // Trigger fleet node scan
    app.on_trigger_node_scan({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |id| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            app.set_scanning_node_id(id.clone());
            let node_info = s.fleet_nodes.iter().find(|n| n.id.as_str() == id.as_str()).map(|n| {
                (n.name.clone(), n.ip.clone(), n.ghost_files_count)
            });
            if let Some((ref name, ref ip, ghost_count)) = node_info {
                let ts = AppState::time_str();
                s.add_log("exec", &format!("[{}] [EXEC] Synchronizing SSH session with edge node: {} ({})", ts, name, ip));
                s.add_log("cmd", &format!("$ ssh -i ~/.ssh/id_ed25519 root@{} \"oxidust core scan\"", ip));
                s.add_log("ok", "[OK] SSH Tunnel secured. Fetching device disk layout tables...");
                s.add_log("info", &format!("[INFO] Target node {} reports: {} unlinked locking processes.", name, ghost_count));
            }
            update_app(&app, &s);
            app.set_scanning_node_id(SharedString::default());
        }
    });

    // Remote fleet purge
    app.on_remote_node_purge({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |id| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            app.set_scanning_node_id(id.clone());
            let node_idx = s.fleet_nodes.iter().position(|n| n.id.as_str() == id.as_str());
            if let Some(idx) = node_idx {
                let name = s.fleet_nodes[idx].name.clone();
                let ip = s.fleet_nodes[idx].ip.clone();
                let disk_total = s.fleet_nodes[idx].disk_total;
                let disk_free = s.fleet_nodes[idx].disk_free;

                let ts = AppState::time_str();
                s.add_log("exec", &format!("[{}] [EXEC] Triggering edge purge command on node: {}", ts, name));
                s.add_log("cmd", &format!("$ ssh -i ~/.ssh/id_ed25519 root@{} \"oxidust system clean --force\"", ip));

                let space_recovered = (disk_total as f64 * 0.45) as i32;
                let new_free = disk_free + space_recovered;
                let new_usage = (((disk_total - new_free) as f64 / disk_total as f64) * 100.0) as i32;

                s.fleet_nodes[idx].disk_free = new_free;
                s.fleet_nodes[idx].disk_usage = new_usage;
                s.fleet_nodes[idx].ghost_files_count = 0;
                s.fleet_nodes[idx].status = SharedString::from("online");
                s.fleet_nodes[idx].last_scan = SharedString::from("Just now");

                s.add_log("ok", "[OK] Remote cleaning operations successful on target node.");
                s.add_log("success", "[SUCCESS] Edge node health state updated: Healthy.");
            }
            update_app(&app, &s);
            app.set_scanning_node_id(SharedString::default());
        }
    });

    // Add fleet node
    app.on_add_node({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |name, ip, os, total_space_str| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let total_space: i32 = total_space_str.parse().unwrap_or(250);
            let disk_total = (total_space as i64) * 1024 * 1024 * 1024;
            let disk_free = ((total_space as f64 * 0.65) as i64) * 1024 * 1024 * 1024;

            let new_node = FleetNode {
                id: SharedString::from(format!("fleet_node_{}", Local::now().timestamp())),
                name: name.clone(),
                ip: ip.clone(),
                os: os.clone(),
                status: SharedString::from("online"),
                disk_usage: 35,
                disk_total: disk_total as i32,
                disk_free: disk_free as i32,
                ghost_files_count: 0,
                last_scan: SharedString::from("Registered today"),
            };

            let ts = AppState::time_str();
            s.add_log("ok", &format!("[{}] [OK] Registered new edge node target: {} ({})", ts, name, ip));
            s.add_log("info", "[INFO] Establishing persistent telemetry channels... Connected.");
            s.fleet_nodes.insert(0, new_node);
            update_app(&app, &s);
        }
    });

    // Send chat message
    app.on_send_message({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move |msg| {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            let msg_str = msg.to_string();
            if msg_str.trim().is_empty() {
                return;
            }

            s.chat_history.push(ChatMessage {
                role: SharedString::from("user"),
                text: SharedString::from(msg_str.clone()),
            });

            // Simulated AI response
            let response = match msg_str.to_lowercase() {
                _ if msg_str.contains("nginx") || msg_str.contains("ghost") => {
                    "### Ghost Log Analysis\n\nTo safely release deleted Nginx file descriptors without restarting:\n\n1. Identify the PID holding the handle:\n   ```bash\n   lsof +L1 | grep nginx\n   ```\n2. Truncate the file handle:\n   ```bash\n   > /proc/PID/fd/FD_NUM\n   ```\n3. Or use Oxidust:\n   ```bash\n   oxidust ghost release --pid=PID\n   ```\n\n**Note:** This does not restart Nginx and is completely safe for production."
                }
                _ if msg_str.contains("docker") || msg_str.contains("volume") => {
                    "### Docker Volume Debloat Strategy\n\nSafe steps to purge orphaned Docker resources:\n\n1. List dangling volumes:\n   ```bash\n   docker volume ls -qf dangling=true\n   ```\n2. Remove orphaned volumes:\n   ```bash\n   docker volume prune --filter 'label!=\n   ```\n3. Clean build cache:\n   ```bash\n   docker builder prune -af\n   ```\n\n**Safety:** Use `--dry-run` first with any prune command."
                }
                _ if msg_str.contains("homebrew") || msg_str.contains("brew") || msg_str.contains("macos") => {
                    "### macOS Homebrew Cleanup\n\nRecommended safe cleanup targets:\n\n- `~/Library/Caches/Homebrew` - Source tarballs (safe to delete)\n- `~/Library/Logs/DiagnosticReports` - Crash logs (safe)\n- `~/Library/Developer/Xcode/DerivedData` - Xcode caches (safe, will regenerate)\n\n```bash\nbrew cleanup --prune=7\nrm -rf ~/Library/Caches/Homebrew/*\n```\n\nAlways verify with `oxidust scan --dry-run` before bulk deletion."
                }
                _ => {
                    "### System Analysis\n\nBased on your query, here are recommended actions:\n\n1. **Run a full scan** with `oxidust core scan` to identify reclaimable space.\n2. **Check ghost file handles** using the Ghost-Hunter tab - these often consume significant invisible space.\n3. **Review fleet nodes** if managing remote servers.\n\nWould you like me to elaborate on any specific area? I can provide detailed shell commands and safety assessments."
                }
            };

            s.chat_history.push(ChatMessage {
                role: SharedString::from("assistant"),
                text: SharedString::from(response),
            });

            update_app(&app, &s);
        }
    });

    // Generate audit
    app.on_generate_audit({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move || {
            let app = app_handle.upgrade().unwrap();
            let s = state.borrow();
            let report = format!(
                "## Oxidust Storage Audit Report\n\n**OS:** {}\n**Scan Status:** Complete\n\n### Findings\n\n- {} storage blocks identified\n- {} ghost file handles detected\n- {} fleet nodes monitored\n\n### Recommendations\n\n1. **Review Rule Presets**: {} active rules selected\n2. **Release Ghost Handles**: Use Ghost-Hunter tab to close file descriptors\n3. **Optimize Fleet**: {} nodes with critical disk usage\n\n### Summary\n\nReclaimable space: {}\n\n---\n*Generated by Oxidust AI Advisor*",
                s.active_os.to_uppercase(),
                s.blocks.len(),
                s.ghosts.iter().filter(|g| !g.released).count(),
                s.fleet_nodes.len(),
                s.presets.iter().filter(|p| p.checked).count(),
                s.fleet_nodes.iter().filter(|n| n.disk_usage > 80).count(),
                format_bytes(s.total_reclaimable_selected()),
            );
            app.set_audit_report(SharedString::from(report));
        }
    });

    // Clear logs
    app.on_clear_logs({
        let state = state.clone();
        let app_handle = app_weak.clone();
        move || {
            let app = app_handle.upgrade().unwrap();
            let mut s = state.borrow_mut();
            s.logs.clear();
            update_app(&app, &s);
        }
    });

    app.run().unwrap();
}
