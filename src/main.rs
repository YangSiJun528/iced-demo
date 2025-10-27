use iced::{Color, Element, Length, Task, alignment, widget::{column, container, row, scrollable, text}, Theme, Background};

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
        let column_views: Vec<Element<Message>> = self.columns
            .iter()
            .map(|col| {
                // 각 컬럼을 container로 감싸서 배경 + 패딩 + 그림자(선택) 적용
                container(col.view())
                    .width(200)
                    .height(600)
                    .style(|theme| {
                        let bg = match theme {
                            Theme::Dark => Color::from_rgb8(58, 58, 77),  // 밝은 다크
                            Theme::Light => Color::from_rgb8(250, 250, 252), // 밝은 라이트
                            _ => Color::from_rgb8(70, 70, 90),
                        };
                        container::Style {
                            background: Some(Background::Color(bg)),
                            ..container::Style::default()
                        }
                    })
                    .into()
            })
            .collect();

        let content = scrollable(row(column_views))
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new().width(10).scroller_width(10).spacing(0).margin(0),
            ))
            .width(Length::Shrink)
            .height(Length::Shrink);

        // 루트 배경 (가장 어두운/밝은 배경)
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme| {
                let bg = match theme {
                    Theme::Dark => Color::from_rgb8(30, 30, 40),
                    Theme::Light => Color::from_rgb8(240, 240, 245),
                    _ => Color::from_rgb8(45, 45, 55),
                };
                container::Style {
                    background: Some(Background::Color(bg)),
                    ..container::Style::default()
                }
            })
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
        let items: Vec<Element<Message>> =
            self.rows.iter().map(|row_file| row_file.view()).collect();

        scrollable(column(items).spacing(2))
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(10)
                    .scroller_width(10)
                    .spacing(0), // 항상 표시, 간격 0
            ))
            .width(200)
            .height(Length::Fill)
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

        container(
            row![text(icon).size(16), text(name).size(14),]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
        )
        .padding(8)
        .width(Length::Fill)
        .into()
    }
}
