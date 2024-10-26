use cosmic_text::{Align, Attrs, Family};
use tiny_skia::Pixmap;

use crate::{
    config,
    edges::margin::Margin,
    utils::{color::parse_hex_to_cosmic_color, text::FontRenderer},
};

use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentStyle, RawComponentStyle},
};

pub struct Watermark {
    children: Vec<Box<dyn Component>>,
    config: Option<config::Watermark>,
}

impl Component for Watermark {
    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let config = self.config.clone().unwrap();
        let attrs = Attrs::new()
            .color(parse_hex_to_cosmic_color(&config.color))
            .family(Family::Name(&config.font_family));

        FontRenderer::new(
            20.,
            20.,
            context.scale_factor,
            context.take_snapshot_params.fonts_folder.clone(),
        )
        .draw_line(
            0.,
            render_params.y,
            pixmap.width() as f32,
            pixmap.height() as f32,
            &config.content,
            attrs,
            Some(Align::Center),
            pixmap,
        );

        Ok(())
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self) -> bool {
        self.config.is_some()
    }

    fn style(&self) -> RawComponentStyle {
        let default_style = RawComponentStyle::default();

        match &self.config {
            Some(_) => default_style.margin(Margin {
                bottom: 22.,
                top: 50.,
                ..Margin::default()
            }),
            None => default_style,
        }
    }
}

impl Watermark {
    pub fn new(config: Option<config::Watermark>) -> Watermark {
        Watermark {
            children: vec![],
            config,
        }
    }
}
