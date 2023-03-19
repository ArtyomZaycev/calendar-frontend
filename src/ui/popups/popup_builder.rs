use egui::{Align, Color32, InnerResponse, Layout, RichText, WidgetText};

use crate::{
    state::State,
    ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal},
};

pub struct ContentUiInfo<'a> {
    pub info: ContentInfo,
    pub buttons:
        Vec<Box<dyn FnOnce(&mut egui::Ui, &mut ContentInfoBuilder, bool) -> egui::Response + 'a>>,
    pub error: Option<String>,
}

impl<'a> ContentUiInfo<'a> {
    pub fn new() -> Self {
        Self {
            info: ContentInfo::new(),
            buttons: vec![],
            error: None,
        }
    }

    pub fn builder<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ContentInfoBuilder),
    {
        Self {
            info: self.info.builder(f),
            ..self
        }
    }

    pub fn close_button(self, text: impl Into<WidgetText> + 'a) -> Self {
        self.button(|ui, builder, _| {
            let response = ui.button(text);
            if response.clicked() {
                builder.close();
            }
            response
        })
    }
    pub fn button<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut egui::Ui, &mut ContentInfoBuilder, bool) -> egui::Response + 'a,
    {
        self.buttons.push(Box::new(f));
        self
    }

    // First is shown
    pub fn error(self, is_error: bool, msg: &str) -> Self {
        self.some_error(is_error.then_some(msg.to_owned()))
    }
    pub fn some_error(self, error: Option<String>) -> Self {
        Self {
            error: self.error.or(error),
            ..self
        }
    }
}

pub struct ContentInfo {
    pub signals: Vec<AppSignal>,
    pub is_closed: bool,
}

impl ContentInfo {
    pub fn new() -> Self {
        Self {
            signals: vec![],
            is_closed: false,
        }
    }

    pub fn with_builder<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ContentInfoBuilder),
    {
        let mut builder = ContentInfoBuilder::new();
        f(&mut builder);
        self.is_closed |= builder.is_closed;
        self.signals.append(&mut builder.signals);
    }

    pub fn builder<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ContentInfoBuilder),
    {
        self.with_builder(f);
        self
    }
}

pub struct ContentInfoBuilder {
    signals: Vec<AppSignal>,
    is_closed: bool,
}

impl ContentInfoBuilder {
    fn new() -> ContentInfoBuilder {
        ContentInfoBuilder {
            signals: vec![],
            is_closed: false,
        }
    }

    pub fn signal(&mut self, signal: impl Into<AppSignal>) {
        self.signals.push(signal.into());
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

pub trait PopupBuilder<'a> {
    fn build(
        &'a mut self,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> InnerResponse<ContentInfo> + 'a> {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                let ContentUiInfo {
                    mut info,
                    buttons,
                    error,
                } = self.content(ui, ctx, state).inner;
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    info.with_builder(|builder| {
                        buttons.into_iter().rev().for_each(|button| {
                            button(ui, builder, error.is_some());
                        });
                    });
                    if let Some(error) = error.clone() {
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            ui.add(
                                egui::Label::new(RichText::new(error).color(Color32::RED))
                                    .wrap(true),
                            );
                        });
                    }
                });
                info
            })
        })
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>>;
}

impl<'a> WidgetBuilder<'a> for dyn PopupBuilder<'a> {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context, state: &'a State) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        Box::new(move |ui| self.build(ctx, state)(ui).response)
    }
}
