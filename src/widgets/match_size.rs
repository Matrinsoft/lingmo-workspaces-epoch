//! Show one surface, sized to match the size of another (invisible) widget

use lingmo::iced::advanced::widget::{Operation, Tree};
use lingmo::iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, renderer};
use lingmo::iced::event::Event;
use lingmo::iced::{Length, Rectangle, Size};
use std::marker::PhantomData;

pub fn match_size<
    'a,
    Msg,
    T1: Into<lingmo::Element<'a, Msg>>,
    T2: Into<lingmo::Element<'a, Msg>>,
>(
    matched: T1,
    shown: T2,
) -> MatchSize<'a, Msg> {
    MatchSize {
        matched: matched.into(),
        shown: shown.into(),
        _msg: PhantomData,
    }
}

pub struct MatchSize<'a, Msg> {
    matched: lingmo::Element<'a, Msg>,
    shown: lingmo::Element<'a, Msg>,
    _msg: PhantomData<Msg>,
}

impl<Msg> Widget<Msg, lingmo::Theme, lingmo::Renderer> for MatchSize<'_, Msg> {
    delegate::delegate! {
        to self.matched.as_widget() {
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
        self.matched
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
        self.shown
            .as_widget_mut()
            .operate(&mut tree.children[1], layout, renderer, operation);
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
        self.shown.as_widget_mut().update(
            &mut tree.children[1],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &lingmo::Renderer,
    ) -> mouse::Interaction {
        self.shown.as_widget().mouse_interaction(
            &tree.children[1],
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
        // TODO?
        self.matched
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
        self.shown.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.matched), Tree::new(&self.shown)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.matched, &mut self.shown]);
    }
}

impl<'a, Msg: 'a> From<MatchSize<'a, Msg>> for lingmo::Element<'a, Msg> {
    fn from(widget: MatchSize<'a, Msg>) -> Self {
        lingmo::Element::new(widget)
    }
}
