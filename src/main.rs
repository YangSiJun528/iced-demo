use iced::alignment::Horizontal;
use iced::widget::{container, text, Scrollable, mouse_area, stack};
use iced::{
    widget::{
        container::Style, row,
        scrollable::{Direction, Scrollbar},
        Column as WidgetColumn,
        Container,
    },
    Background, Color, Element, Length, Task, Theme, Point, Border, Alignment,
};

fn main() -> iced::Result {
    iced::application("Column View", App::update, App::view).run_with(App::new)
}

#[derive(Debug, Clone)]
enum Message {
    DividerPressed(usize),
    DividerReleased,
    DividerMoved(Point),
}

struct App {
    columns: Vec<Column>,
    dragging_divider: Option<usize>,
    drag_start_x: f32,
    initial_width: f32,
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
                dragging_divider: None,
                drag_start_x: 0.0,
                initial_width: 0.0,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DividerPressed(index) => {
                self.dragging_divider = Some(index);
                self.initial_width = self.columns[index].width;
            }
            Message::DividerReleased => {
                self.dragging_divider = None;
            }
            Message::DividerMoved(point) => {
                if let Some(divider_index) = self.dragging_divider {
                    // 첫 이동 시 시작점 기록
                    if self.drag_start_x == 0.0 {
                        self.drag_start_x = point.x;
                        return Task::none();
                    }

                    let delta = point.x - self.drag_start_x;

                    // 최소/최대 너비 제한
                    const MIN_WIDTH: f32 = 100.0;
                    const MAX_WIDTH: f32 = 600.0;

                    if divider_index < self.columns.len() {
                        let new_width = (self.initial_width + delta)
                            .max(MIN_WIDTH)
                            .min(MAX_WIDTH);
                        self.columns[divider_index].width = new_width;
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let mut column_views = Vec::new();

        for (i, col) in self.columns.iter().enumerate() {
            // 마지막 컬럼이 아니면 구분선 포함한 컬럼 뷰 생성
            if i < self.columns.len() - 1 {
                column_views.push(col.view_with_divider(i));
            } else {
                // 마지막 컬럼은 구분선 없이
                column_views.push(
                    Container::new(col.view())
                        .style(column_container_style)
                        .into(),
                );
            }
        }

        let content = Scrollable::new(row(column_views))
            .direction(Direction::Horizontal(Scrollbar::new()))
            .width(Length::Shrink)
            .height(Length::Shrink);

        let inner = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(root_container_style);

        // 전역 mouse_area로 감싸서 전체 화면에서 마우스 이벤트 감지
        let mut global_area = mouse_area(inner)
            .on_release(Message::DividerReleased);

        // 드래그 중일 때만 move 이벤트 등록
        if self.dragging_divider.is_some() {
            global_area = global_area.on_move(Message::DividerMoved);
        }

        global_area.into()
    }
}

struct Column {
    width: f32,
    rows: Vec<RowFile>,
}

impl Column {
    fn new(width: f32, rows: Vec<RowFile>) -> Self {
        Self { width, rows }
    }

    fn view(&self) -> Element<Message> {
        let items = self.rows.iter().map(RowFile::view).collect::<Vec<_>>();

        Scrollable::new(WidgetColumn::with_children(items))
            .direction(Direction::Vertical(Scrollbar::new().spacing(0)))
            .width(self.width)
            .height(600)
            .into()
    }

    fn view_with_divider(&self, index: usize) -> Element<Message> {
        let content = self.view();

        // 구분선 버튼 (우측 하단 고정)
        let divider = Container::new(
            mouse_area(
                Container::new(text("||").size(14))
                    .padding([4, 6])
                    .style(divider_style),
            )
                .on_press(Message::DividerPressed(index)),
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::End)
            .align_y(Alignment::End);

        // stack으로 컬럼 내용과 구분선 버튼을 겹침
        Container::new(stack![content, divider])
            .width(self.width)
            .style(column_container_style)
            .into()
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

fn column_container_style(_theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb8(250, 250, 252))),
        ..Style::default()
    }
}

fn divider_style(_theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb8(180, 180, 190))),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        ..Style::default()
    }
}
