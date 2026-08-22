use lingmo::iced::advanced::widget::{Id, Operation, Tree, tree};
use lingmo::iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer};
use lingmo::iced::event::Event;
use lingmo::iced::{Length, Rectangle, Size, Vector};
use std::marker::PhantomData;

mod image_bg;
mod workspace_bar;
pub use workspace_bar::workspace_bar;
mod size_cross_nth;
pub use size_cross_nth::size_cross_nth;
mod mouse_interaction_wrapper;
mod toplevels;
pub use toplevels::toplevels;
mod visibility_wrapper;
pub use visibility_wrapper::visibility_wrapper;
mod match_size;
pub use match_size::match_size;

// Widget for debugging
#[allow(dead_code)]
pub fn layout_wrapper<'a, Msg, T: Into<lingmo::Element<'a, Msg>>>(
    inner: T,
) -> LayoutWrapper<'a, Msg> {
    LayoutWrapper {
        content: inner.into(),
        _msg: PhantomData,
    }
}

pub struct LayoutWrapper<'a, Msg> {
    content: lingmo::Element<'a, Msg>,
    _msg: PhantomData<Msg>,
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for LayoutWrapper<'_, Msg> {
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &lingmo::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget_mut().layout(tree, renderer, limits)
    }

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
            fn mouse_interaction(
                &self,
                _tree: &Tree,
                _layout: Layout<'_>,
                _cursor: mouse::Cursor,
                _viewport: &Rectangle,
                _renderer: &lingmo::Renderer,
            ) -> mouse::Interaction;
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
                transation: Vector,
            ) -> Option<overlay::Element<'b, Msg, lingmo::Theme, lingmo::Renderer>>;
            fn set_id(&mut self, id: Id);
            fn operate(
                    &mut self,
                    tree: &mut Tree,
                    layout: Layout<'_>,
                    renderer: &lingmo::Renderer,
                    operation: &mut dyn Operation<()>,
                );
        }
    }
}

impl<'a, Msg: 'a> From<LayoutWrapper<'a, Msg>> for lingmo::Element<'a, Msg> {
    fn from(widget: LayoutWrapper<'a, Msg>) -> Self {
        lingmo::Element::new(widget)
    }
}
