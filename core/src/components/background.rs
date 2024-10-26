use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, Pixmap, Point, Rect, SpreadMode, Transform,
};

use crate::{
    edges::{edge::Edge, padding::Padding},
    utils::color::RgbaColor,
};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error::{self, RenderError},
    style::{ComponentAlign, ComponentStyle, RawComponentStyle},
};

pub struct Background {
    children: Vec<Box<dyn Component>>,
    padding: Padding,
}

impl Background {
    pub fn new(padding: Padding, children: Vec<Box<dyn Component>>) -> Background {
        Background { children, padding }
    }

    pub fn has_background(padding: &Padding) -> bool {
        return padding.horizontal() != 0. || padding.vertical() != 0.;
    }
}

impl Component for Background {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        RawComponentStyle::default()
            .align(ComponentAlign::Column)
            .padding(self.padding.clone())
    }

    fn self_render_condition(&self) -> bool {
        Self::has_background(&self.padding)
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        _render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let mut paint = Paint::default();
        let w = pixmap.width() as f32;
        let h = pixmap.height() as f32;
        let params = &context.take_snapshot_params;

        paint.anti_alias = false;

        match &params.background {
            crate::config::Background::Solid(solid_background) => {
                let rgba_color: RgbaColor = solid_background.as_str().into();

                paint.set_color(rgba_color.into());
            }
            crate::config::Background::Gradient(gradient_background) => {
                let start = gradient_background.start.into_f32_point(w, h);
                let end = gradient_background.end.into_f32_point(w, h);

                paint.shader = LinearGradient::new(
                    Point::from_xy(start.x, start.y),
                    Point::from_xy(end.x, end.y),
                    gradient_background.stops.clone(),
                    SpreadMode::Pad,
                    Transform::identity(),
                )
                .unwrap();
            }
        };

        // if is_valid_hex_color(&params.background) {
        //     let rgba_color: RgbaColor = params.background.as_str().into();
        //
        //     paint.set_color(rgba_color.into());
        // } else {
        //     paint.shader = LinearGradient::new(
        //         Point::from_xy(0., 0.),
        //         Point::from_xy(w, 0.),
        //         Background::get_theme(&params.background)?,
        //         SpreadMode::Pad,
        //         Transform::identity(),
        //     )
        //     .unwrap();
        // }

        pixmap.fill_rect(
            Rect::from_xywh(0., 0., w, h).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );

        Ok(())
    }
}

impl Background {
    fn get_theme(theme: &str) -> render_error::Result<Vec<GradientStop>> {
        let theme = match theme {
            "default" => vec![
                GradientStop::new(0.0, Color::from_rgba8(58, 28, 113, 255)),
                GradientStop::new(0.5, Color::from_rgba8(215, 109, 119, 255)),
                GradientStop::new(0.95, Color::from_rgba8(255, 175, 123, 255)),
            ],
            "sea" => vec![
                GradientStop::new(0.0, Color::from_rgba8(31, 162, 255, 255)),
                GradientStop::new(0.4, Color::from_rgba8(18, 216, 250, 255)),
                GradientStop::new(0.95, Color::from_rgba8(166, 255, 203, 255)),
            ],
            "grape" => vec![
                GradientStop::new(0.28, Color::from_rgba8(103, 90, 247, 255)),
                GradientStop::new(0.95, Color::from_rgba8(189, 101, 250, 255)),
            ],
            "peach" => vec![
                GradientStop::new(0.22, Color::from_rgba8(221, 94, 137, 255)),
                GradientStop::new(0.95, Color::from_rgba8(247, 187, 151, 255)),
            ],
            "summer" => vec![
                GradientStop::new(0.28, Color::from_rgba8(248, 165, 194, 255)),
                GradientStop::new(0.95, Color::from_rgba8(116, 185, 255, 255)),
            ],
            "bamboo" => vec![
                GradientStop::new(0.22, Color::from_rgba8(107, 203, 165, 255)),
                GradientStop::new(0.95, Color::from_rgba8(202, 244, 194, 255)),
            ],
            "dusk" => vec![
                GradientStop::new(0.22, Color::from_rgba8(255, 98, 110, 255)),
                GradientStop::new(0.95, Color::from_rgba8(255, 190, 113, 255)),
            ],
            _ => return Err(RenderError::UnknownBackgroundTheme(theme.to_string())),
        };

        Ok(theme)
    }
}
