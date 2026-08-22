use lingmo::iced::advanced::widget::{Id, Operation, Tree, tree};
use lingmo::iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer};
use lingmo::iced::event::Event;
use lingmo::iced::{Length, Rectangle, Size, Vector};

use std::marker::PhantomData;

pub fn mouse_interaction_wrapper<'a, Msg, T: Into<lingmo::Element<'a, Msg>>>(
    mouse_interaction: mouse::Interaction,
    content: T,
) -> MouseInteractionWrapper<'a, Msg> {
    MouseInteractionWrapper {
        content: content.into(),
        mouse_interaction,
        _msg: PhantomData,
    }
}

pub struct MouseInteractionWrapper<'a, Msg> {
    content: lingmo::Element<'a, Msg>,
    mouse_interaction: mouse::Interaction,
    _msg: PhantomData<Msg>,
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for MouseInteractionWrapper<'_, Msg> {
    delegate::delegate! {
        to self.content.as_widget() {
            fn tag(&self) -> tree::Tag;
            fn state(&self) -> tree::State;
            fn children(&self) -> Vec<Tree>;
            fn size(&self) -> Size<Length>;
            fn size_hint(&self) -> Size<Length>;
            fn draw(
                &self,
                state: &Tree,
                renderer: &mut lingmo::Renderer,
                theme: &lingmo::Theme,
                style: &renderer::Style,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                viewport: &Rectangle,
            );
            fn id(&self) -> Option<Id>;
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
            fn set_id(&mut self, id: Id);
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

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &lingmo::Renderer,
    ) -> mouse::Interaction {
        self.mouse_interaction
    }
}

impl<'a, Msg: 'static> From<MouseInteractionWrapper<'a, Msg>> for lingmo::Element<'a, Msg> {
    fn from(widget: MouseInteractionWrapper<'a, Msg>) -> Self {
        lingmo::Element::new(widget)
    }
}
