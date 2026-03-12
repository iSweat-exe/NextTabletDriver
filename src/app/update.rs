use crate::app::state::{AppTab, TabletMapperApp};
use crate::settings::save_last_session;
use crate::ui::panels::console::render_console_panel;
use crate::ui::panels::filters::render_filters_panel;
use crate::ui::panels::output::render_output_panel;
use crate::ui::panels::pen_settings::render_pen_settings_panel;
use crate::ui::panels::release::render_release_panel;
use crate::ui::panels::settings::render_settings_panel;
use crate::ui::panels::support::render_support_panel;
use eframe::egui;
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl eframe::App for TabletMapperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Event Driven Data Sync ---
        // Drain all pending tablet events to get the latest state
        let mut last_data = None;
        while let Ok(data) = self.tablet_receiver.try_recv() {
            last_data = Some(data);
        }
        if let Some(data) = last_data {
            let mut shared_data = self.shared.tablet_data.write().unwrap();
            *shared_data = data;
        }

        // Check for updates
        if let Ok(status) = self.update_receiver.try_recv() {
            if let crate::app::autoupdate::UpdateStatus::Available(release) = &status {
                log::info!(target: "Update", "Update available: {}", release.tag_name);
            }
            self.update_status = status;
        }

        crate::ui::components::update_dialog::render_update_dialog(self, ctx);

        // Get snapshot of data for UX
        // We modify local copies of config then push to shared if changed
        let mut config = self.shared.config.read().unwrap().clone();
        let initial_config = config.clone();

        // Calc Screen Bounds - Required for both Viz and Inputs
        let mut min_x = 0.0;
        let mut min_y = 0.0;
        let mut max_x = 1920.0;
        let mut max_y = 1080.0;
        if !self.displays.is_empty() {
            let mut mx = i32::MAX;
            let mut my = i32::MAX;
            let mut ax = i32::MIN;
            let mut ay = i32::MIN;
            for d in &self.displays {
                mx = mx.min(d.x);
                my = my.min(d.y);
                ax = ax.max(d.x + d.width as i32);
                ay = ay.max(d.y + d.height as i32);
            }
            min_x = mx as f32;
            min_y = my as f32;
            max_x = ax as f32;
            max_y = ay as f32;
        }

        // === UI PANELS (ORDER MATTERS) ===

        // 1. Top Menu Bar
        crate::ui::components::menu_bar::render_menu_bar(self, ctx);

        // 2. Tabs
        crate::ui::components::tabs::render_tabs(self, ctx);

        // 3. Bottom Footer
        crate::ui::components::footer::render_footer(self, ctx, &mut config);

        // 4. Central Content
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                AppTab::Output => {
                    render_output_panel(self, ui, &mut config, min_x, min_y, max_x, max_y);
                }
                AppTab::Filters => {
                    render_filters_panel(self, ui, &mut config);
                }
                AppTab::PenSettings => {
                    render_pen_settings_panel(self, ui, &mut config);
                }
                AppTab::Console => {
                    render_console_panel(self, ui);
                }
                AppTab::Settings => {
                    render_settings_panel(self, ui, &mut config);
                }
                AppTab::Support => {
                    render_support_panel(self, ui);
                }
                AppTab::Release => {
                    render_release_panel(self, ui);
                }
            });
        });

        // Push config only if actually changed by UI inputs
        if config != initial_config {
            {
                let mut shared_config = self.shared.config.write().unwrap();
                *shared_config = config.clone();
                // Signal change to backend thread
                self.shared.config_version.fetch_add(1, Ordering::SeqCst);
            }
            // Auto-save session
            let _ = save_last_session(&config);
        }

        // Ensure we keep polling occasionally for UI status (battery, connection, etc)
        ctx.request_repaint_after(std::time::Duration::from_millis(1000));
    }
}
