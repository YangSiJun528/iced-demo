use iced::alignment::Horizontal;
use iced::widget::{container, text, Scrollable};
use iced::{
    widget::{
        container::Style, row,
        scrollable::{Direction, Scrollbar},
        Column as WidgetColumn,
        Container,
    }, Background, Color, Element, Length, Task,
    Theme,
};
use iced::widget::text::Wrapping;

fn main() -> iced::Result {
    iced::application("Column View", App::update, App::view).run_with(App::new)
}

#[derive(Debug, Clone)]
enum Message {}

struct App {
    columns: Vec<Column>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                columns: vec![
                    Column::new(vec![
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
                    ]),
                    Column::new(vec![
                        RowFile::Dir("124MB".into(), "Projects".into()),
                        RowFile::File("235 bytes".into(), "config.json".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                    ]),
                    Column::new(vec![RowFile::Dir("124MB".into(), "Projects".into())]),
                    Column::new(vec![
                        RowFile::Dir("124MB".into(), "Projects".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                    ]),
                    Column::new(vec![
                        RowFile::Dir("124MB".into(), "Projects".into()),
                        RowFile::File("4KB".into(), "config.json".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                        RowFile::Dir("124MB".into(), "Downloads".into()),
                    ]),
                ],
            },
            Task::none(),
        )
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let column_views = self
            .columns
            .iter()
            .map(|col| {
                Container::new(col.view())
                    .style(column_container_style)
                    .into()
            })
            .collect::<Vec<_>>();

        let content = Scrollable::new(row(column_views))
            .direction(Direction::Horizontal(Scrollbar::new()))
            .width(Length::Shrink)
            .height(Length::Shrink);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(root_container_style)
            .into()
    }
}

struct Column {
    rows: Vec<RowFile>,
}

impl Column {
    fn new(rows: Vec<RowFile>) -> Self {
        Self { rows }
    }

    fn view(&self) -> Element<Message> {
        let items = self.rows.iter().map(RowFile::view).collect::<Vec<_>>();

        Scrollable::new(WidgetColumn::with_children(items))
            .direction(Direction::Vertical(Scrollbar::new().spacing(0))) // Scrollbar의 spacing이 None이 아니므로 Embedded Scrollbars로 동작
            .width(200)
            .height(600)
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

        // 고정 폭: 파일 크기
        let size_text = Container::new(text(size).size(12))
            .width(60) // "1111 bytes" 정도 공간 확보
            .align_x(Horizontal::Left);

        // 파일 이름 영역, 남는 공간을 채움
        let name_text = Container::new(text(name).size(12)
                    //.wrapping(Wrapping::None)// 줄바꿈 등 단위 변경
                                       // https://discourse.iced.rs/t/how-to-implement-text-overflow-ellipsis-or-text-overflow-clip/1059
                                       // 해당 이슈가 아직 해결이 안됨, 이걸 해결된 버전이 릴리즈 되지 않았음.
                                       // 0.13.1에서는 높이 설정해서 2줄 되면 짤리는 식으로 구현해야 함.
        )
            //.clip(true)
            .width(Length::Fill) // 남는 공간 모두 사용
            .height(16)
            .align_x(Horizontal::Left);

        // 우측 고정: '>' 표시
        let arrow = container(if is_dir { text(">") } else { text("") }).width(20); //⟩ 처럼 각도가 넓은걸 쓰고 싶은데, 글자로 인식을 못함.

        let row = row![size_text, name_text, arrow]
            .spacing(8)
            .align_y(iced::Alignment::Center);

        Container::new(row)
            .padding([6, 8])
            .width(Length::Fill)
            .into()
    }
}

// ======================
// 스타일 함수
// ======================

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
