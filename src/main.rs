use iced::widget::{text, Scrollable};
use iced::{
    Background, Color, Element, Length, Task, Theme,
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
                        RowFile::Dir("Documents".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::File("readme.txt".into()),
                    ]),
                    Column::new(vec![
                        RowFile::Dir("Projects".into()),
                        RowFile::File("config.json".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                    ]),
                    Column::new(vec![RowFile::Dir("Projects".into())]),
                    Column::new(vec![
                        RowFile::Dir("Projects".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                    ]),
                    Column::new(vec![
                        RowFile::Dir("Projects".into()),
                        RowFile::File("config.json".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
                        RowFile::Dir("Downloads".into()),
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
    File(String),
    Dir(String),
}

impl RowFile {
    fn view(&self) -> Element<Message> {
        let (icon, name) = match self {
            RowFile::File(name) => ("F", name),
            RowFile::Dir(name) => ("D", name),
        };

        Container::new(
            row![text(icon).size(16), text(name).size(14)]
                .spacing(8)
                .align_y(iced::alignment::Vertical::Center),
        )
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
