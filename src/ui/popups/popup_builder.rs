use egui::{Align, Color32, InnerResponse, Layout, RichText, WidgetText};

use crate::{
    state::State,
    ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal},
};

pub struct ContentInfo<'a> {
    pub buttons: Vec<Box<dyn FnOnce(&mut egui::Ui, bool) -> egui::Response + 'a>>,
    pub error: Option<String>,
}

impl<'a> ContentInfo<'a> {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
            error: None,
        }
    }

    pub fn close_button(self, text: impl Into<WidgetText> + 'a, is_closed: &'a mut bool) -> Self {
        self.button(|ui, _| {
            let response = ui.button(text);
            if response.clicked() {
                *is_closed = true;
            }
            response
        })
    }
    pub fn button<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut egui::Ui, bool) -> egui::Response + 'a,
    {
        self.buttons.push(Box::new(f));
        self
    }

    pub fn buttons<F>(
        self,
        buttons: Vec<Box<dyn FnOnce(&mut egui::Ui, bool) -> egui::Response + 'a>>,
    ) -> Self {
        Self { buttons, ..self }
    }

    // First is shown
    pub fn error(self, error: Option<String>) -> Self {
        Self {
            error: self.error.or(error),
            ..self
        }
    }
}

pub trait PopupBuilder<'a> {
    fn build(
        &'a mut self,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                let content = self.content(ui, ctx, state).inner;
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    content.buttons.into_iter().rev().for_each(|button| {
                        button(ui, content.error.is_some());
                    });
                    if let Some(error) = content.error {
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            ui.add(
                                egui::Label::new(RichText::new(error).color(Color32::RED))
                                    .wrap(true),
                            );
                        });
                    }
                });
            })
            .response
        })
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentInfo<'a>>;

    fn title(&'a self) -> Option<String> {
        None
    }

    fn signals(&'a self) -> Vec<AppSignal>;

    fn is_closed(&'a self) -> bool;
}

impl<'a> WidgetBuilder<'a> for dyn PopupBuilder<'a> {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context, state: &'a State) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        self.build(ctx, state)
    }
}
