use crate::app::state::TabletMapperApp;
use eframe::egui;

pub fn render_update_dialog(app: &mut TabletMapperApp, ctx: &egui::Context) {
    let mut update_action = None;

    if let crate::app::autoupdate::UpdateStatus::Available(release) = &app.update_status {
        let mut open = true;
        let version = release.tag_name.clone();
        egui::Window::new("Update Available")
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(format!("A new version ({}) is available.", version));
                    ui.label("Would you like to install it now?");
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("Update Now").clicked() {
                            update_action = Some(true);
                        }
                        if ui.button("Later").clicked() {
                            update_action = Some(false);
                        }
                    });
                });
            });
    }

    if let Some(install) = update_action {
        if install {
            if let Some(release) = app.update_status.as_release() {
                let release_clone = release.clone();
                std::thread::spawn(move || {
                    let _ = crate::app::autoupdate::download_and_install(release_clone);
                });
                app.update_status = crate::app::autoupdate::UpdateStatus::Downloading(0.0);
            }
        } else {
            app.update_status = crate::app::autoupdate::UpdateStatus::Idle;
        }
    }
}
