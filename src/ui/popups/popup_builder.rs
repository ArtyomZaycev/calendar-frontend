use crate::ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal};

pub trait PopupBuilder<'a> {
    fn build(
        &'a mut self,
        ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn header(&'a self) -> Option<String> {
        None
    }

    fn signals(&'a self) -> Vec<AppSignal>;

    fn is_closed(&'a self) -> bool;
}

impl<'a> WidgetBuilder<'a> for dyn PopupBuilder<'a> {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        self.build(ctx)
    }
}
