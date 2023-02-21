pub trait WidgetBuilder<'a> {
    type OutputWidget;

    fn build(&'a mut self, ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a;
}
