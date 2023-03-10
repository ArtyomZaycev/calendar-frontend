use egui::{Response, RichText, Sense, Ui};

pub trait UiUtils {
    fn selectable_rich_text<F: FnOnce()>(
        &mut self,
        text: RichText,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
    fn selectable_label<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
    fn selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
}

impl UiUtils for Ui {
    fn selectable_rich_text<F: FnOnce()>(
        &mut self,
        mut text: RichText,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        if is_selected {
            text = text.underline()
        }

        let response = self.add(egui::Label::new(text).sense(Sense::click()));

        if response.clicked() {
            on_clicked();
        }

        response
    }

    fn selectable_label<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.selectable_rich_text(RichText::new(text), is_selected, on_clicked)
    }

    fn selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.selectable_rich_text(RichText::new(text).heading(), is_selected, on_clicked)
    }
}
