use egui::{Direction, Response, RichText, Sense, Shape, Stroke, Ui, Vec2, Widget};

pub trait UiUtils {
    fn enabled_selectable_rich_text<F: FnOnce()>(
        &mut self,
        text: RichText,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
    fn enabled_selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_enabled: bool,
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
    fn enabled_selectable_rich_text<F: FnOnce()>(
        &mut self,
        mut text: RichText,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        if is_selected {
            text = text.underline()
        }

        let response = self.add_enabled(is_enabled, egui::Label::new(text).sense(Sense::click()));

        if response.clicked() {
            on_clicked();
        }

        response
    }

    fn enabled_selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.enabled_selectable_rich_text(
            RichText::new(text).heading(),
            is_enabled,
            is_selected,
            on_clicked,
        )
    }

    fn selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.enabled_selectable_header(text, true, is_selected, on_clicked)
    }
}

pub struct DirectionSymbol {
    direction: Direction,
}

impl DirectionSymbol {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
        }
    }
}

impl Widget for DirectionSymbol {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) = ui.allocate_painter(Vec2::splat(16.), Sense::click());
        let widget_visuals = &ui.style().visuals.widgets;
        let color = if ui.is_enabled() {
            if response.hovered() {
                widget_visuals.hovered.fg_stroke.color
            } else {
                widget_visuals.inactive.fg_stroke.color
            }
        } else {
            ui.style().visuals.gray_out(widget_visuals.noninteractive.fg_stroke.color)
        };
        let start = painter.clip_rect().center();
        let radius = 5.;

        let (mul, swap) = match self.direction {
            Direction::LeftToRight => (1., false),
            Direction::RightToLeft => (-1., false),
            Direction::TopDown => (1., true),
            Direction::BottomUp => (-1., true),
        };

        let points = [
            Vec2::new(radius, 0.),
            Vec2::new(-radius, radius),
            Vec2::new(-radius, -radius),
        ].into_iter();

        painter.add(Shape::convex_polygon(points.map(|v| start + mul * if swap {v.yx()} else {v}).collect(), color, Stroke::NONE));
        response
    }
}