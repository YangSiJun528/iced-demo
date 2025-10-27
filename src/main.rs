use iced::{Background, Color, Element, Length, Task, Theme, widget::{Column as WidgetColumn, Container, column, container, row, scrollable, text}, Border};

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
            .map(|col| col.view())
            .map(|col| {
                container(col)
                    .style(column_container_style)
                    .into()
            })
            .collect::<Vec<_>>();

        let content = scrollable(row(column_views))
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new().width(10).scroller_width(10),
            ))
            .width(Length::Shrink)
            .height(Length::Shrink);

        container(content)
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

        scrollable(WidgetColumn::with_children(items).spacing(0))
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new().width(10).scroller_width(10).spacing(0),
            ))
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

        container(
            row![text(icon).size(16), text(name).size(14),]
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

fn root_container_style(theme: &Theme) -> container::Style {
    let bg = match theme {
        Theme::Dark => Color::from_rgb8(30, 30, 40),
        Theme::Light => Color::from_rgb8(240, 240, 245),
        _ => panic!("Unknown theme"),
    };
    container::Style {
        background: Some(Background::Color(bg)),
        ..container::Style::default()
    }
}

fn column_container_style(theme: &Theme) -> container::Style {
    let bg = match theme {
        Theme::Dark => Color::from_rgb8(58, 58, 77),
        Theme::Light => Color::from_rgb8(250, 250, 252),
        _ => panic!("Unknown theme"),
    };
    container::Style {
        background: Some(Background::Color(bg)),
        ..container::Style::default()
    }
}
