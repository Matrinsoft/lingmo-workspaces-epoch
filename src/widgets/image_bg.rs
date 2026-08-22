// Renders image behind widget, and otherwise passes through all behavior

use lingmo::iced::advanced::layout::{self};
use lingmo::iced::advanced::widget::{Operation, Tree, tree};
use lingmo::iced::advanced::{Clipboard, Layout, Shell, Widget, mouse, overlay, renderer};
use lingmo::iced::core::Renderer;
use lingmo::iced::event::Event;
use lingmo::iced::{Length, Rectangle, Size, Vector};

use std::marker::PhantomData;

pub fn image_bg<'a, Msg, T1: Into<lingmo::Element<'a, Msg>>, T2: Into<lingmo::Element<'a, Msg>>>(
    content: T1,
    bg: T2,
) -> ImageBg<'a, Msg> {
    ImageBg {
        content: content.into(),
        bg: bg.into(),
        _msg: PhantomData,
    }
}

pub struct ImageBg<'a, Msg> {
    content: lingmo::Element<'a, Msg>,
    bg: lingmo::Element<'a, Msg>,
    _msg: PhantomData<Msg>,
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for ImageBg<'_, Msg> {
    delegate::delegate! {
        to self.content.as_widget() {
            fn tag(&self) -> tree::Tag;
            fn state(&self) -> tree::State;
            fn children(&self) -> Vec<Tree>;
            fn size(&self) -> Size<Length>;
            fn size_hint(&self) -> Size<Length>;
            fn mouse_interaction(
                &self,
                tree: &Tree,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                viewport: &Rectangle,
                renderer: &lingmo::Renderer,
            ) -> mouse::Interaction;
        }

        to self.content.as_widget_mut() {
            fn diff(&mut self, tree: &mut Tree);
            fn update(
                &mut self,
                tree: &mut Tree,
                event: &Event,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                renderer: &lingmo::Renderer,
                clipboard: &mut dyn Clipboard,
                shell: &mut Shell<'_, Msg>,
                viewport: &Rectangle,
            );
            fn overlay<'b>(
                &'b mut self,
                tree: &'b mut Tree,
                layout: Layout<'b>,
                renderer: &lingmo::Renderer,
                viewport: &Rectangle,
                translation: Vector,
            ) -> Option<overlay::Element<'b, Msg, lingmo::Theme, lingmo::Renderer>>;
            fn layout(
                    &mut self,
                    tree: &mut Tree,
                    renderer: &lingmo::Renderer,
                    limits: &layout::Limits,
                ) -> layout::Node;
            fn operate(
                    &mut self,
                    tree: &mut Tree,
                    layout: Layout<'_>,
                    renderer: &lingmo::Renderer,
                    operation: &mut dyn Operation<()>,
                );
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut lingmo::Renderer,
        theme: &lingmo::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.bg
            .as_widget()
            .draw(state, renderer, theme, style, layout, cursor, viewport);

        let bounds = layout.bounds();
        renderer.with_layer(bounds, |renderer| {
            self.content
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport)
        });
    }
}

impl<'a, Msg: 'static> From<ImageBg<'a, Msg>> for lingmo::Element<'a, Msg> {
    fn from(widget: ImageBg<'a, Msg>) -> Self {
        lingmo::Element::new(widget)
    }
}
