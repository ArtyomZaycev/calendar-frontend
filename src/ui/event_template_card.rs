use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::db::aliases::EventTemplate;

use super::widget_signal::{AppSignal, StateSignal};

pub struct EventTemplateCard<'a> {
    signals: &'a mut Vec<AppSignal>,
    desired_size: Vec2,
    template: &'a EventTemplate,
    show_description: bool,
}

impl<'a> EventTemplateCard<'a> {
    pub fn new(
        signals: &'a mut Vec<AppSignal>,
        desired_size: Vec2,
        template: &'a EventTemplate,
    ) -> Self {
        Self {
            signals,
            desired_size,
            template,
            show_description: true,
        }
    }

    pub fn hide_description(self) -> Self {
        Self {
            show_description: false,
            ..self
        }
    }
}

impl<'a> Widget for EventTemplateCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            let EventTemplate {
                id: template_id,
                name,
                event_description,
                ..
            } = self.template;

            egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(1., Color32::LIGHT_BLUE))
                .inner_margin(4.)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
                            if ui.small_button("X").clicked() {
                                self.signals
                                    .push(StateSignal::DeleteEventTemplate(*template_id).into());
                            }
                            /*if ui.small_button("E").clicked() {
                                self.signals.push(AppSignal::ChangeEventTemplate(*template_id));
                            }*/
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                ui.add(egui::Label::new(name).wrap(true));
                            });
                        });
                        if self.show_description {
                            if let Some(description) = event_description {
                                ui.separator();
                                ui.label(description);
                            }
                        }
                    })
                })
                .response
        })
        .response
    }
}
