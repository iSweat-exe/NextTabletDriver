use crate::app::state::TabletMapperApp;
use crate::core::config::models::MappingConfig;
use eframe::egui;

pub fn render_stats_settings(app: &TabletMapperApp, ui: &mut egui::Ui, config: &mut MappingConfig) {
    let stats = app
        .shared
        .stats
        .read()
        .map(|g| *g)
        .unwrap_or_else(|e| *e.into_inner());

    ui.horizontal(|ui| {
        ui.checkbox(
            &mut config.speed_stats.enabled,
            "Enable HandSpeed Stats WebSocket Server",
        );
    });
    ui.add_space(12.0);

    ui.vertical(|ui| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().item_spacing.y = 8.0;

        // LIVE STATS SECTION
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Live Statistics").strong().size(14.0));
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("Current HandSpeed:");
                    let unit_str = match config.speed_stats.unit {
                        crate::core::config::models::SpeedUnit::MillimetersPerSecond => "mm/s",
                        crate::core::config::models::SpeedUnit::MetersPerSecond => "m/s",
                        crate::core::config::models::SpeedUnit::KilometersPerHour => "km/h",
                        crate::core::config::models::SpeedUnit::MilesPerHour => "mph",
                    };
                    ui.label(
                        egui::RichText::new(format!("{:.2} {}", stats.handspeed, unit_str))
                            .color(egui::Color32::from_rgb(0, 150, 255))
                            .strong(),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Total Distance:");
                    let dist = stats.total_distance_mm;
                    let dist_text = if dist < 1000.0 {
                        format!("{:.1} mm", dist)
                    } else if dist < 1000000.0 {
                        format!("{:.3} m", dist / 1000.0)
                    } else {
                        format!("{:.3} km", dist / 1000000.0)
                    };
                    ui.label(
                        egui::RichText::new(dist_text)
                            .color(egui::Color32::from_rgb(0, 150, 255))
                            .strong(),
                    );

                    if ui.button("Reset").clicked()
                        && let Ok(mut stats) = app.shared.stats.write()
                    {
                        stats.total_distance_mm = 0.0;
                    }
                });
            });
        });

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("IP Address:");
            ui.text_edit_singleline(&mut config.speed_stats.ip);
        });

        ui.horizontal(|ui| {
            ui.label("Port:");
            let mut port_f32 = config.speed_stats.port as f32;
            if ui
                .add(
                    egui::DragValue::new(&mut port_f32)
                        .speed(1.0)
                        .range(1.0..=65535.0),
                )
                .changed()
            {
                config.speed_stats.port = port_f32 as u16;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Speed Unit:");
            egui::ComboBox::from_id_salt("speed_unit_combo")
                .selected_text(match config.speed_stats.unit {
                    crate::core::config::models::SpeedUnit::MillimetersPerSecond => "mm/s",
                    crate::core::config::models::SpeedUnit::MetersPerSecond => "m/s",
                    crate::core::config::models::SpeedUnit::KilometersPerHour => "km/h",
                    crate::core::config::models::SpeedUnit::MilesPerHour => "mph",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::core::config::models::SpeedUnit::MillimetersPerSecond,
                        "mm/s",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::core::config::models::SpeedUnit::MetersPerSecond,
                        "m/s",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::core::config::models::SpeedUnit::KilometersPerHour,
                        "km/h",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::core::config::models::SpeedUnit::MilesPerHour,
                        "mph",
                    );
                });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new("WebSocket Information").strong());
        ui.label(format!(
            "Server Address: ws://{}:{}",
            config.speed_stats.ip, config.speed_stats.port
        ));
        ui.label(
            "JSON format: { \"handspeed\": float, \"timestamp\": u128, \"total_distance\": float }",
        );
    });
}
