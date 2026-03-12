use crate::core::config::models::MappingConfig;
use crate::ui::theme::ui_setting_row;
use eframe::egui;

pub fn render_antichatter_settings(ui: &mut egui::Ui, config: &mut MappingConfig) {
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
