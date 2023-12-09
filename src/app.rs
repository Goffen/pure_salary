pub struct PurePlotApp {
    tjanstepension_avsattning_kronor: u16,
    tjanstepension_avsattning_procent: u8,
    tjanstepension_avsattning_i_procent: bool,
    bonus_debiteringsgrad: bool,
    bonus_debiteringsgrad_procent: u8,
    timpris: u16,
    pure_andel: u8,
    lon: u32,
    skatt: u8,
}

impl Default for PurePlotApp {
    fn default() -> Self {
        Self {
            tjanstepension_avsattning_kronor: 5000,
            tjanstepension_avsattning_procent: 5,
            tjanstepension_avsattning_i_procent: false,
            bonus_debiteringsgrad: true,
            bonus_debiteringsgrad_procent: 90,
            timpris: 860,
            pure_andel: 75,
            lon: 50000,
            skatt: 33,
        }
    }
}

impl PurePlotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.style_mut(|style| {
            style.override_text_style = Some(egui::TextStyle::Monospace);
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        });
        Default::default()
    }
}

impl eframe::App for PurePlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Timpris: ");
                ui.add(egui::Slider::new(&mut self.timpris, 400..=1500).text("kr/h"));
            });
            ui.horizontal(|ui| {
                ui.label("Skatt: ");
                ui.add(egui::Slider::new(&mut self.skatt, 25..=40).text("%"));
            });
            ui.horizontal(|ui| {
                ui.label("Tjänstepension avsättning: ");
                ui.checkbox(&mut self.tjanstepension_avsattning_i_procent, "i %");
                if self.tjanstepension_avsattning_i_procent {
                    ui.add(egui::Slider::new(&mut self.tjanstepension_avsattning_procent, 0..=10).text("%"));
                }
                else {
                    ui.add(egui::Slider::new(&mut self.tjanstepension_avsattning_kronor, 0..=20000).text("kr/mån"));
                }
            });
            ui.horizontal(|ui| {
                ui.label("Pure andel: ");
                ui.add(egui::Slider::new(&mut self.pure_andel, 50..=100).text("%"));
            });
            ui.horizontal(|ui| {
                ui.label("Bonus vid hög debitering");
                ui.checkbox(&mut self.bonus_debiteringsgrad, "Ja/Nej");
                if self.bonus_debiteringsgrad {
                    ui.add(egui::Slider::new(&mut self.bonus_debiteringsgrad_procent, 0..=100).text("%"));
                }
            });
            ui.horizontal(|ui| {
                ui.label("Lön: ");
                ui.add(egui::Slider::new(&mut self.lon, 0..=80000).text("kr/månad"));
            });
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                egui::warn_if_debug_build(ui);
            });

        });
    }
}
