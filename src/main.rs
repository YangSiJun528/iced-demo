use iced::alignment::Horizontal;
use iced::widget::{Scrollable, container, text};
use iced::{Background, Color, Element, Length, Task, Theme,
    widget::{
        Column as WidgetColumn, Container,
        container::Style,
        row,
        scrollable::{Direction, Scrollbar},
    },
};

fn main() -> iced::Result {
    iced::application("Column View", App::update, App::view).run_with(App::new)
}

#[derive(Debug, Clone)]
enum Message {
    DividerDragged(usize, f32),
    DividerReleased(usize),
}

struct App {
    columns: Vec<Column>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                columns: vec![
                    Column::new(
                        200.0,
                        vec![
                            RowFile::Dir("12KB".into(), "Documents".into()),
                            RowFile::Dir("124MB".into(), "Downlasdffsaoads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::File("8KB".into(), "readme.txt".into()),
                        ],
                    ),
                    Column::new(
                        200.0,
                        vec![
                            RowFile::Dir("124MB".into(), "Projects".into()),
                            RowFile::File("235 bytes".into(), "config.json".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                        ],
                    ),
                    Column::new(200.0, vec![RowFile::Dir("124MB".into(), "Projects".into())]),
                    Column::new(
                        200.0,
                        vec![
                            RowFile::Dir("124MB".into(), "Projects".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                        ],
                    ),
                    Column::new(
                        200.0,
                        vec![
                            RowFile::Dir("124MB".into(), "Projects".into()),
                            RowFile::File("4KB".into(), "config.json".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                            RowFile::Dir("124MB".into(), "Downloads".into()),
                        ],
                    ),
                ],
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DividerDragged(index, delta) => {
                if index < self.columns.len() {
                    let col = &mut self.columns[index];
                    let new_width = (col.width + delta).max(MIN_WIDTH).min(MAX_WIDTH);
                    col.resize_offset = new_width - col.width;
                }
            }
            Message::DividerReleased(index) => {
                if index < self.columns.len() {
                    let col = &mut self.columns[index];
                    col.width += col.resize_offset;
                    col.resize_offset = 0.0;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let mut column_views = Vec::new();

        for (i, col) in self.columns.iter().enumerate() {
            column_views.push(col.view_with_divider(i));
        }

        let content = Scrollable::new(row(column_views))
            .direction(Direction::Horizontal(Scrollbar::new()))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .spacing(0); // 스크롤바 항상 활성화

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(root_container_style)
            .into()
    }
}

const MIN_WIDTH: f32 = 100.0;
const MAX_WIDTH: f32 = 600.0;

struct Column {
    width: f32,
    resize_offset: f32,
    rows: Vec<RowFile>,
}

impl Column {
    fn new(width: f32, rows: Vec<RowFile>) -> Self {
        Self {
            width,
            resize_offset: 0.0,
            rows,
        }
    }

    fn view(&self) -> Element<Message> {
        let items = self.rows.iter().map(RowFile::view).collect::<Vec<_>>();

        Scrollable::new(WidgetColumn::with_children(items))
            .direction(Direction::Vertical(Scrollbar::new().spacing(0)))
            .width(self.width + self.resize_offset)
            .height(600)
            .into()
    }

    fn view_with_divider(&self, index: usize) -> Element<Message> {
        divider::Divider::new(
            self.view(),
            move |delta| Message::DividerDragged(index, delta),
            Message::DividerReleased(index),
        ).into()
    }
}

#[derive(Clone)]
enum RowFile {
    File(String, String),
    Dir(String, String),
}

impl RowFile {
    fn view(&self) -> Element<Message> {
        let (size, name, is_dir) = match self {
            RowFile::File(size, name) => (size, name, false),
            RowFile::Dir(size, name) => (size, name, true),
        };

        let size_text = Container::new(text(size).size(12))
            .width(60)
            .align_x(Horizontal::Left);

        let name_text = Container::new(text(name).size(12))
            .width(Length::Fill)
            .height(16)
            .align_x(Horizontal::Left);

        let arrow = container(if is_dir { text(">") } else { text("") }).width(20);

        let row = row![size_text, name_text, arrow]
            .spacing(8)
            .align_y(iced::Alignment::Center);

        Container::new(row)
            .padding([6, 8])
            .width(Length::Fill)
            .into()
    }
}

fn root_container_style(_theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb8(240, 240, 245))),
        ..Style::default()
    }
}

mod divider {
    //Refence: https://github.com/tarkah/iced_table
    use iced_core::layout::{self, Layout};
    use iced_core::mouse::Cursor;
    use iced_core::widget::{self, Widget};
    use iced_core::{event, mouse, overlay, renderer, Background, Border, Clipboard, Color, Element, Length, Point, Rectangle, Shell, Size};

    #[derive(Debug, Clone, Copy, Default)]
    struct State {
        drag_origin: Option<Point>,
        is_hovered: bool,
    }

    pub struct Divider<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        content: Element<'a, Message, Theme, Renderer>,
        on_drag: Box<dyn Fn(f32) -> Message + 'a>,
        on_release: Message,
    }

    impl<'a, Message, Theme, Renderer> Divider<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        pub fn new<F>(
            content: Element<'a, Message, Theme, Renderer>,
            on_drag: F,
            on_release: Message,
        ) -> Self
        where
            F: Fn(f32) -> Message + 'a,
        {
            Self {
                content,
                on_drag: Box::new(on_drag),
                on_release,
            }
        }

        fn divider_bounds(&self, bounds: Rectangle) -> Rectangle {
            const HANDLE_WIDTH: f32 = 24.0;
            const HANDLE_HEIGHT: f32 = 24.0;
            Rectangle {
                x: bounds.x + bounds.width - HANDLE_WIDTH,
                y: bounds.y + bounds.height - HANDLE_HEIGHT,
                width: HANDLE_WIDTH,
                height: HANDLE_HEIGHT,
            }
        }

        fn hover_bounds(&self, divider_bounds: Rectangle) -> Rectangle {
            let mut bounds = divider_bounds;
            bounds.x -= 2.0;
            bounds.width += 4.0;
            bounds.y -= 2.0;
            bounds.height += 4.0;
            bounds
        }
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Divider<'a, Message, Theme, Renderer>
    where
        Message: Clone,
        Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced::Font>,
    {
        fn tag(&self) -> widget::tree::Tag {
            widget::tree::Tag::of::<State>()
        }

        fn state(&self) -> widget::tree::State {
            widget::tree::State::new(State::default())
        }

        fn children(&self) -> Vec<widget::Tree> {
            vec![widget::Tree::new(&self.content)]
        }

        fn diff(&self, tree: &mut widget::Tree) {
            tree.diff_children(&[&self.content]);
        }

        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &self,
            tree: &mut widget::Tree,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        }

        fn on_event(
            &mut self,
            tree: &mut widget::Tree,
            event: event::Event,
            layout: Layout<'_>,
            cursor: Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) -> event::Status {
            let state = tree.state.downcast_mut::<State>();

            let bounds = layout.bounds();
            let divider_bounds = self.divider_bounds(bounds);
            let hover_bounds = self.hover_bounds(divider_bounds);

            state.is_hovered = cursor.is_over(hover_bounds);

            let mut status = event::Status::Ignored;

            if let event::Event::Mouse(mouse_event) = event {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if cursor.is_over(hover_bounds) {
                            if let Some(position) = cursor.position() {
                                state.drag_origin = Some(position);
                                status = event::Status::Captured;
                            }
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.drag_origin.is_some() {
                            state.drag_origin = None;
                            shell.publish(self.on_release.clone());
                            status = event::Status::Captured;
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(position) = cursor.position() {
                            if let Some(origin) = state.drag_origin {
                                let delta = position.x - origin.x;
                                shell.publish((self.on_drag)(delta));
                                status = event::Status::Captured;
                            }
                        }
                    }
                    _ => {}
                }
            }

            let content_status = self.content.as_widget_mut().on_event(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            status.merge(content_status)
        }

        fn mouse_interaction(
            &self,
            tree: &widget::Tree,
            layout: Layout<'_>,
            cursor: Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            let state = tree.state.downcast_ref::<State>();

            if state.drag_origin.is_some() || state.is_hovered {
                mouse::Interaction::ResizingHorizontally
            } else {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    layout,
                    cursor,
                    viewport,
                    renderer,
                )
            }
        }

        fn draw(
            &self,
            tree: &widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            r_style: &renderer::Style,
            layout: Layout<'_>,
            cursor: Cursor,
            viewport: &Rectangle,
        ) {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                r_style,
                layout,
                cursor,
                viewport,
            );

            // Draw the divider handle
            let bounds = layout.bounds();
            let divider_bounds = self.divider_bounds(bounds);

            // Background quad
            renderer.fill_quad(
                renderer::Quad {
                    bounds: divider_bounds,
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    shadow: Default::default(),
                },
                Background::Color(Color::from_rgb8(180, 180, 190)),
            );

            // Text "||"
            let text_bounds = Rectangle {
                x: divider_bounds.x + 6.0,
                y: divider_bounds.y + 4.0,
                width: divider_bounds.width - 12.0,
                height: divider_bounds.height - 8.0,
            };

            renderer.fill_text(
                iced_core::text::Text {
                    content: "||".parse().unwrap(),
                    bounds: Size::new(text_bounds.width, text_bounds.height),
                    size: iced::Pixels(14.0),
                    font: iced::Font::DEFAULT,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    line_height: iced_core::text::LineHeight::default(),
                    shaping: iced_core::text::Shaping::Basic,
                    wrapping: Default::default(),
                },
                text_bounds.position(),
                Color::BLACK,
                *viewport,
            );
        }

        fn overlay<'b>(
            &'b mut self,
            tree: &'b mut widget::Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            translation: iced_core::Vector,
        ) -> Option<overlay::Element<'_, Message, Theme, Renderer>> {
            self.content.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                translation,
            )
        }
    }

    impl<'a, Message, Theme, Renderer> From<Divider<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Renderer: renderer::Renderer + iced_core::text::Renderer<Font = iced::Font> + 'a,
        Theme: 'a,
    {
        fn from(divider: Divider<'a, Message, Theme, Renderer>) -> Self {
            Element::new(divider)
        }
    }
}
