//! If `visible` is set to `true`, behaves exactly as wrapped widget. If `false`,
//! takes the same space but does not draw.

use lingmo::iced::advanced::widget::{Operation, Tree};
use lingmo::iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, renderer};
use lingmo::iced::event::Event;
use lingmo::iced::{Length, Rectangle, Size};
use std::marker::PhantomData;

pub fn visibility_wrapper<'a, Msg, T: Into<lingmo::Element<'a, Msg>>>(
    inner: T,
    visible: bool,
) -> VisibilityWrapper<'a, Msg> {
    VisibilityWrapper {
        content: inner.into(),
        visible,
        _msg: PhantomData,
    }
}

pub struct VisibilityWrapper<'a, Msg> {
    content: lingmo::Element<'a, Msg>,
    visible: bool,
    _msg: PhantomData<Msg>,
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for VisibilityWrapper<'_, Msg> {
    delegate::delegate! {
        to self.content.as_widget() {
            fn size(&self) -> Size<Length>;
            fn size_hint(&self) -> Size<Length>;
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &lingmo::Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

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
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &lingmo::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &lingmo::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut lingmo::Renderer,
        theme: &lingmo::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if self.visible {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.content]);
    }
}

impl<'a, Msg: 'a> From<VisibilityWrapper<'a, Msg>> for lingmo::Element<'a, Msg> {
    fn from(widget: VisibilityWrapper<'a, Msg>) -> Self {
        lingmo::Element::new(widget)
    }
}
