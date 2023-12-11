use egui::{RichText, TextStyle, Ui};

pub struct PurePlotApp {
    tjanstepension_avsattning_kronor: u32,
    tjanstepension_avsattning_procent: u32,
    tjanstepension_avsattning_i_procent: bool,
    bonus_debiteringsgrad_procent: u32,
    bonus_debiteringsgrad_timmar: u32,
    timpris: u32,
    pure_andel: u32,
    lon: u32,
    arbetsgivaravgift_procent: f32,
    kommunal_skatt: f32,
    statlig_skatt_grans: u32,
    arbetstid: u32,
    arbetsdagar_per_manad: u32,
    arbetstimmar_per_manad: u32,
    arbetsdagar_i_timmar: bool,
}

impl Default for PurePlotApp {
    fn default() -> Self {
        Self {
            tjanstepension_avsattning_kronor: 6000,
            tjanstepension_avsattning_procent: 12,
            tjanstepension_avsattning_i_procent: false,
            bonus_debiteringsgrad_procent: 10,
            bonus_debiteringsgrad_timmar: 130,
            timpris: 860,
            pure_andel: 25,
            lon: 50000,
            arbetsgivaravgift_procent: 31.42,
            kommunal_skatt: 34.408,
            statlig_skatt_grans: 598_500 / 12,
            arbetstid: 100,
            arbetsdagar_per_manad: 21,
            arbetstimmar_per_manad: 21 * 8,
            arbetsdagar_i_timmar: false,
        }
    }
}

impl PurePlotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.style_mut(|style| {
            style.override_text_style = Some(TextStyle::Monospace);
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            for (text_style, font_id) in style.text_styles.iter_mut() {
                match text_style {
                    TextStyle::Body => font_id.size = 16.,
                    TextStyle::Heading => font_id.size = 20.,
                    TextStyle::Monospace => font_id.size = 17.,
                    TextStyle::Button => font_id.size = 17.,
                    _ => font_id.size = 100.,
                }
            }
        });

        Default::default()
    }

    fn input_grid(&mut self, ui: &mut Ui) {
        egui::Grid::new("input_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Timpris");
                ui.add(egui::Slider::new(&mut self.timpris, 400..=1500).text("kr/h"));
                ui.end_row();

                ui.label("Pure andel");
                ui.add(egui::Slider::new(&mut self.pure_andel, 0..=100).text("%"));
                ui.end_row();

                ui.label("Pure andel över bonusgräns");
                ui.add(
                    egui::Slider::new(&mut self.bonus_debiteringsgrad_procent, 0..=self.pure_andel)
                        .text("%"),
                );
                ui.end_row();

                ui.label("Bonusgräns");
                ui.add(
                    egui::Slider::new(&mut self.bonus_debiteringsgrad_timmar, 0..=200)
                        .text("timmar/månad"),
                );
                ui.end_row();
            });
    }

    fn tax_grid(&mut self, ui: &mut Ui) {
        egui::Grid::new("skatt_grid")
            .num_columns(2)
            .spacing([80.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Arbetsgivaravgift");
                ui.add(
                    egui::Slider::new(&mut self.arbetsgivaravgift_procent, 25.0..=40.0).text("%"),
                );
                ui.end_row();
                ui.label("Kommunalskatt");
                ui.add(egui::Slider::new(&mut self.kommunal_skatt, 25.0..=40.0).text("%"));
                ui.end_row();
                ui.label("Statlig skattgräns");
                ui.add(
                    egui::Slider::new(&mut self.statlig_skatt_grans, 45000..=60000)
                        .text("kr/månad"),
                );
                ui.end_row();
                ui.label("Arbetsdagar per månad");
                if self.arbetsdagar_i_timmar {
                    ui.add(
                        egui::Slider::new(&mut self.arbetstimmar_per_manad, 0..=200)
                            .text("timmar/månad"),
                    );
                } else {
                    ui.add(
                        egui::Slider::new(&mut self.arbetsdagar_per_manad, 18..=23)
                            .text("dagar/månad"),
                    );
                }

                ui.checkbox(&mut self.arbetsdagar_i_timmar, "eller ange i timmar");
                ui.end_row();
            });
    }

    fn choice_grid(&mut self, ui: &mut Ui) {
        egui::Grid::new("brutto_cost_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Lön");
                ui.add(egui::Slider::new(&mut self.lon, 0..=80000).text("kr/månad"));
                ui.end_row();
                ui.label("Tjänstepension avsättning");
                ui.horizontal(|ui| {
                    if self.tjanstepension_avsattning_i_procent {
                        ui.add(
                            egui::Slider::new(&mut self.tjanstepension_avsattning_procent, 0..=20)
                                .text("%"),
                        );
                    } else {
                        ui.add(
                            egui::Slider::new(
                                &mut self.tjanstepension_avsattning_kronor,
                                0..=20000,
                            )
                            .text("kr/mån"),
                        );
                    }
                    ui.checkbox(&mut self.tjanstepension_avsattning_i_procent, "i %");
                });
                ui.end_row();
                if !self.arbetsdagar_i_timmar {
                    ui.label("Arbetstid");
                    ui.add(egui::Slider::new(&mut self.arbetstid, 0..=100).text("%"));
                    ui.end_row();
                }
            });
    }

    fn result_ui(&mut self, ui: &mut Ui) {
        // 860 kr / h
        let timpris = self.timpris as f32;
        // 21
        let arbetsdagar_per_manad = self.arbetsdagar_per_manad as f32;
        // 1.0
        let arbetstid_procent = self.arbetstid as f32 / 100.0;
        // 0.25
        let pure_andel_procent = self.pure_andel as f32 / 100.0;
        // 0.1
        let bonus_debiteringsgrad_procent = self.bonus_debiteringsgrad_procent as f32 / 100.0;
        // 60_000 kr / månad
        let lon = self.lon as f32;
        // 54_000 kr / månad
        let statlig_grans = self.statlig_skatt_grans as f32;
        // 10_000 kr / månad
        let mut tjanstepension_avsattning_kronor = self.tjanstepension_avsattning_kronor as f32;
        // 130 timmar / månad
        let bonus_debiteringsgrad_timmar = self.bonus_debiteringsgrad_timmar as f32;

        if self.tjanstepension_avsattning_i_procent {
            tjanstepension_avsattning_kronor =
                lon * self.tjanstepension_avsattning_procent as f32 / 100.0;
            self.tjanstepension_avsattning_kronor = tjanstepension_avsattning_kronor as u32;
        }
        let timmar;
        if self.arbetsdagar_i_timmar {
            timmar = self.arbetstimmar_per_manad as f32;
        } else {
            timmar = 8.0 * arbetsdagar_per_manad * arbetstid_procent;
        }
        let pure_bonus_andel;
        if bonus_debiteringsgrad_timmar < timmar {
            // 860 * (168 - 130) * 0.1 = 3_268
            pure_bonus_andel =
                timpris * (timmar - bonus_debiteringsgrad_timmar) * bonus_debiteringsgrad_procent;
        } else {
            pure_bonus_andel = 0.0;
        }
        let per_manad = timpris * timmar;
        // 860 * 130 * 0.25 = 22_360
        let pure_andel = timpris * bonus_debiteringsgrad_timmar.min(timmar) * pure_andel_procent;
        // 144_480 - 32_508 = 111_972
        let inkomst_efter_pure_avdrag = per_manad - pure_andel - pure_bonus_andel;
        // 60_000 * 0.3415 = 20_490
        let arbetsgivar_avgift = lon * self.arbetsgivaravgift_procent / 100.0;
        let mut statlig_skatt_avgift = 0.0;
        // statlig skatt
        if lon > statlig_grans {
            let inkomst_efter_statlig_skatt_grans = lon - statlig_grans;
            statlig_skatt_avgift = inkomst_efter_statlig_skatt_grans * 0.2;
        }
        let semester_avsattning = lon * 0.12;
        let summa = inkomst_efter_pure_avdrag
            - arbetsgivar_avgift
            - lon
            - tjanstepension_avsattning_kronor
            - statlig_skatt_avgift
            - semester_avsattning;
        egui::Grid::new("res_grid")
            .num_columns(2)
            .spacing([100.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Inkomst per månad");
                ui.label(format!("{} kr/månad", per_manad));
                ui.end_row();

                ui.label(format!("Pures andel ({} %)", self.pure_andel));
                ui.label(format!("{} kr", pure_andel));
                ui.end_row();

                ui.label(format!(
                    "Pures bonus andel ({} %)",
                    self.bonus_debiteringsgrad_procent
                ));
                ui.label(format!("{} kr", pure_bonus_andel));
                ui.end_row();

                ui.label("Inkomst efter andel");
                ui.label(format!("{} kr/månad", inkomst_efter_pure_avdrag));
                ui.end_row();

                ui.label("Arbetsgivaravgift");
                ui.label(format!("{} kr/månad", arbetsgivar_avgift));
                ui.end_row();

                ui.label("Statlig skatt avgift");
                ui.label(format!("{} kr/månad", statlig_skatt_avgift));
                ui.end_row();

                ui.label("Semesteravsättning");
                ui.label(format!("{} kr/månad", semester_avsattning));
                ui.end_row();
            });
        ui.separator();
        ui.label(
            RichText::new(format!("Saldo ram: {} kr/månad", summa))
                .heading()
                .strong(),
        );
        ui.label(
            RichText::new(format!(
                "Netto lön: {} kr",
                (lon * (100.0 - self.kommunal_skatt) / 100.0).round()
            ))
            .heading()
            .strong(),
        );
    }
}

impl eframe::App for PurePlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.input_grid(ui);
            ui.separator();
            self.tax_grid(ui);
            ui.separator();
            self.choice_grid(ui);
            ui.separator();
            self.result_ui(ui);
        });
    }
}
