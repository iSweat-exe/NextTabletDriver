use crate::app::state::TabletMapperApp;
use crate::domain::MappingConfig;
use crate::ui::theme::ui_setting_row;
use eframe::egui;

pub fn render_filters_panel(
    app: &mut TabletMapperApp,
    ui: &mut egui::Ui,
    config: &mut MappingConfig,
) {
    ui.horizontal(|ui| {
        // SIDEBAR
        let sidebar_width = 160.0;
        let sidebar_height = ui.available_height();

        ui.allocate_ui_with_layout(
            egui::vec2(sidebar_width, sidebar_height),
            egui::Layout::top_down_justified(egui::Align::LEFT),
            |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_gray(245))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(220)))
                    .inner_margin(4.0)
                    .show(ui, |ui| {
                        ui.set_min_height(sidebar_height);

                        let filters = ["Devocub Antichatter", "HandSpeed WebSocket"];
                        for filter_name in filters {
                            let is_selected = app.selected_filter == filter_name;
                            let res = ui.selectable_label(is_selected, filter_name);
                            if res.clicked() {
                                app.selected_filter = filter_name.to_string();
                            }
                        }
                    });
            },
        );

        // CONTENT
        ui.add_space(8.0);

        ui.vertical(|ui| {
            ui.add_space(10.0);
            match app.selected_filter.as_str() {
                "Devocub Antichatter" => render_antichatter_settings(ui, config),
                "HandSpeed WebSocket" => render_stats_settings(app, ui, config),
                _ => {
                    ui.centered_and_justified(|ui| {
                        ui.label("Select a filter to configure");
                    });
                }
            }
            ui.add_space(20.0);
        });
    });
}

fn render_stats_settings(app: &TabletMapperApp, ui: &mut egui::Ui, config: &mut MappingConfig) {
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
                        crate::domain::SpeedUnit::MillimetersPerSecond => "mm/s",
                        crate::domain::SpeedUnit::MetersPerSecond => "m/s",
                        crate::domain::SpeedUnit::KilometersPerHour => "km/h",
                        crate::domain::SpeedUnit::MilesPerHour => "mph",
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

                    if ui.button("Reset").clicked() {
                        if let Ok(mut stats) = app.shared.stats.write() {
                            stats.total_distance_mm = 0.0;
                        }
                    }
                });
            });
        });

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("IP Address:");
            ui.text_edit_singleline(&mut config.speed_stats.ip);
        });

        // Use a wrapper or custom logic for port as ui_setting_row expects f32
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
                    crate::domain::SpeedUnit::MillimetersPerSecond => "mm/s",
                    crate::domain::SpeedUnit::MetersPerSecond => "m/s",
                    crate::domain::SpeedUnit::KilometersPerHour => "km/h",
                    crate::domain::SpeedUnit::MilesPerHour => "mph",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::domain::SpeedUnit::MillimetersPerSecond,
                        "mm/s",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::domain::SpeedUnit::MetersPerSecond,
                        "m/s",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::domain::SpeedUnit::KilometersPerHour,
                        "km/h",
                    );
                    ui.selectable_value(
                        &mut config.speed_stats.unit,
                        crate::domain::SpeedUnit::MilesPerHour,
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

fn render_antichatter_settings(ui: &mut egui::Ui, config: &mut MappingConfig) {
    ui.horizontal(|ui| {
        ui.checkbox(
            &mut config.antichatter.enabled,
            "Enable Devocub Antichatter",
        );
    });
    ui.add_space(12.0);

    ui.vertical(|ui| {
        ui.set_width(ui.available_width());

        ui.spacing_mut().item_spacing.y = 8.0;

        ui_setting_row(ui, "Latency", &mut config.antichatter.latency, "ms");
        ui_setting_row(
            ui,
            "Antichatter Strength",
            &mut config.antichatter.antichatter_strength,
            "",
        );
        ui_setting_row(
            ui,
            "Antichatter Multiplier",
            &mut config.antichatter.antichatter_multiplier,
            "",
        );
        ui_setting_row(
            ui,
            "Antichatter Offset X",
            &mut config.antichatter.antichatter_offset_x,
            "",
        );
        ui_setting_row(
            ui,
            "Antichatter Offset Y",
            &mut config.antichatter.antichatter_offset_y,
            "",
        );

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui.checkbox(&mut config.antichatter.prediction_enabled, "Prediction");
        ui.add_space(8.0);

        ui_setting_row(
            ui,
            "Prediction Strength",
            &mut config.antichatter.prediction_strength,
            "",
        );
        ui_setting_row(
            ui,
            "Prediction Sharpness",
            &mut config.antichatter.prediction_sharpness,
            "",
        );
        ui_setting_row(
            ui,
            "Prediction Offset X",
            &mut config.antichatter.prediction_offset_x,
            "",
        );
        ui_setting_row(
            ui,
            "Prediction Offset Y",
            &mut config.antichatter.prediction_offset_y,
            "",
        );

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(16.0);

        ui_setting_row(ui, "Frequency", &mut config.antichatter.frequency, "hz");
    });
}
