use iced::{
    Background, Border, Color, Element, Length, Theme,
    widget::{column, container, row, scrollable, text},
};

struct ColumnView;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl ColumnView {
    fn new() -> Self {
        Self
    }

    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        // Main container that fills the entire screen
        container(
            row![
                // Sidebar (left)
                self.sidebar(),
                // Main content area (right)
                self.main_content()
            ]
                .width(Length::Fill)
                .height(Length::Fill),
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(sidebar_container_style)
            .into()
    }

    fn sidebar(&self) -> Element<Message> {
        // Sidebar items
        let sidebar_items = vec![ // 기본 폰트 인코딩이 이모지 텍스트 인식 못해서 지움.
            ("", "Recent", true),
            ("", "Applications", false),
            ("️", "Desktop", false),
            ("", "Documents", false),
            ("", "Downloads", false),
            ("️", "Pictures", false),
            ("", "Music", false),
            ("", "Movies", false),
        ];

        let sidebar_content = sidebar_items
            .into_iter()
            .map(|(icon, label, selected)| self.sidebar_item(icon, label, selected))
            .fold(column![].spacing(2), |col, item| col.push(item));

        // Wrap the content in a scrollable container
        let scrollable_content = container(sidebar_content)
            .width(Length::Fill)
            .height(Length::Shrink);

        container(
            scrollable(scrollable_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(hidden_scrollbar_style),
        )
            .width(200)
            .height(Length::Fill)
            .padding(10)
            .style(sidebar_style)
            .into()
    }

    fn sidebar_item<'a>(
        &self,
        icon: &'a str,
        label: &'a str,
        selected: bool,
    ) -> Element<'a, Message> {
        let content = row![text(icon).size(16).width(25), text(label).size(14)]
            .spacing(8)
            .align_y(iced::alignment::Vertical::Center);

        container(content)
            .width(Length::Fill)
            .padding([8, 12])
            .style(if selected {
                selected_sidebar_item_style
            } else {
                sidebar_item_style
            })
            .into()
    }

    fn main_content(&self) -> Element<Message> {
        // Column view data: category -> subcategory -> items
        let columns_data = vec![
            (
                "Category",
                vec![
                    ("Design", vec!["Figma", "Sketch", "Adobe XD"]),
                    ("Development", vec!["Visual Studio Code", "Xcode", "Android Studio"]),
                    ("Music", vec!["Logic Pro", "Ableton Live", "GarageBand"]),
                ],
            ),
            (
                "Applications",
                vec![
                    ("Figma", vec!["Figma.app", "Settings", "Templates"]),
                    ("Visual Studio Code", vec!["VS Code.app", "Extensions", "Snippets"]),
                ],
            ),
        ];

        let columns = columns_data
            .into_iter()
            .map(|(title, items)| self.column(title, items))
            .fold(row![].spacing(8), |row, col| row.push(col));

        let scrollable_content = container(columns)
            .width(Length::Shrink) // enable horizontal scroll
            .height(Length::Shrink);

        container(
            scrollable(scrollable_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::new().width(8.0).scroller_width(6.0),
                ))
                .style(thin_scrollbar_style),
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(main_content_style)
            .into()
    }

    fn column<'a>(&self, title: &'a str, items: Vec<(&'a str, Vec<&str>)>) -> Element<'a, Message> {
        let header = container(text(title).size(16))
            .width(Length::Fill)
            .padding([12, 16])
            .style(column_header_style);

        let items_list = items
            .into_iter()
            .map(|(item, _)| self.column_item(item))
            .fold(column![].spacing(1), |col, item| col.push(item));

        let scrollable_content = container(items_list)
            .width(Length::Fill)
            .height(Length::Shrink);

        container(
            column![
                header,
                container(scrollable_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(column_scrollable_style)
            ]
                .width(Length::Fill)
                .height(Length::Fill),
        )
            .width(240)
            .height(400)
            .style(column_style)
            .into()
    }

    fn column_item<'a>(&self, label: &'a str) -> Element<'a, Message> {
        container(text(label).size(14))
            .width(Length::Fill)
            .padding([10, 16])
            .style(column_item_style)
            .into()
    }
}

// Styles using container::Style
fn sidebar_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        ..container::Style::default()
    }
}

fn sidebar_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.96))),
        border: Border {
            width: 1.0,
            color: Color::from_rgb(0.8, 0.8, 0.8),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn sidebar_item_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn selected_sidebar_item_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.primary.weak.color)),
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn main_content_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        ..container::Style::default()
    }
}

fn column_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            width: 1.0,
            radius: 8.0.into(),
            color: Color::from_rgb(0.9, 0.9, 0.9),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn column_scrollable_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        ..container::Style::default()
    }
}

fn column_header_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.97, 0.97, 0.97))),
        border: Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn column_item_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 8.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

// Scrollbar styles
fn hidden_scrollbar_style(theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: scrollable::Rail {
            background: None,
            border: Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        horizontal_rail: scrollable::Rail {
            background: None,
            border: Border::default(),
            scroller: scrollable::Scroller {
                color: Color::TRANSPARENT,
                border: Border::default(),
            },
        },
        gap: None,
    }
}

fn thin_scrollbar_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let rail = scrollable::Rail {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
        border: Border {
            radius: 3.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        scroller: scrollable::Scroller {
            color: Color::from_rgb(0.6, 0.6, 0.6),
            border: Border {
                radius: 3.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    let hovered_rail = scrollable::Rail {
        scroller: scrollable::Scroller {
            color: Color::from_rgb(0.4, 0.4, 0.4),
            ..rail.scroller
        },
        ..rail
    };

    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: match status {
            scrollable::Status::Hovered { .. } => hovered_rail,
            _ => rail,
        },
        horizontal_rail: match status {
            scrollable::Status::Hovered { .. } => hovered_rail,
            _ => rail,
        },
        gap: None,
    }
}

fn main() -> iced::Result {
    iced::application("macOS-style Column View", ColumnView::update, ColumnView::view)
        // 이런 식으로 폰트 설정 가능, 근데 아직은 i18n 계획은 없으니 알아만 두기
        // 참고: https://github.com/iced-rs/iced/issues/213
        //.font(include_bytes!("../fonts/NotoSans-VariableFont_wdth,wght.ttf"))
        //.default_font(Font::with_name("NotoSans"))
        .run_with(|| (ColumnView::new(), iced::Task::none()))
}
