pub trait AppWidgetBuilder<'a> {
    type OutputWidget;
    type Signal;

    fn build(&'a mut self, ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a;

    fn signals(&'a self) -> Vec<Self::Signal>;

    fn is_closed(&'a self) -> bool;
}
