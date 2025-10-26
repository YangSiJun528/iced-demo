use iced::{
    widget::{container, scrollable, text, column},
    Element, Length, Theme, Color, Background, Border, Pixels,
};

struct ScrollableExample;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl ScrollableExample {
    fn new() -> Self {
        Self
    }

    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        // 스크롤 가능한 컨텐츠 생성
        let content = (0..50)
            .map(|i| text(format!("아이템 {}", i)).size(20))
            .fold(column!(), |col, txt| col.push(txt))
            .spacing(10);

        // Scrollbar 설정으로 두께 조절
        let vertical_scrollbar = scrollable::Scrollbar::new()
            .width(16.0)          // 스크롤바 트랙의 너비 (두께)
            .scroller_width(14.0) // 스크롤러(움직이는 부분)의 너비
            .margin(2.0);         // 스크롤바와 경계 사이의 여백

        let horizontal_scrollbar = scrollable::Scrollbar::new()
            .width(12.0)          // 가로 스크롤바의 높이 (두께)
            .scroller_width(10.0) // 가로 스크롤러의 높이
            .margin(1.0);

        // Direction으로 Scrollbar 설정
        let direction = scrollable::Direction::Both {
            vertical: vertical_scrollbar,
            horizontal: horizontal_scrollbar,
        };

        // 스크롤러블 위젯 생성
        let scrollable = scrollable::Scrollable::with_direction(content, direction)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(custom_scrollbar_style); // 스타일도 함께 적용

        // 전체 화면을 채우는 컨테이너
        container(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}

// 스크롤바 스타일 함수 (두께와 함께 색상도 커스터마이징)
fn custom_scrollbar_style(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let default_style = scrollable::default(theme, status);

    let custom_rail = scrollable::Rail {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: Color::from_rgb(0.8, 0.8, 0.8),
        },
        scroller: scrollable::Scroller {
            color: Color::from_rgb(0.4, 0.4, 0.7),
            border: Border {
                radius: 7.0.into(), // 스크롤러의 모서리 둥글기 (두께와 조화롭게)
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    let hovered_rail = scrollable::Rail {
        scroller: scrollable::Scroller {
            color: Color::from_rgb(0.5, 0.5, 0.8),
            ..custom_rail.scroller
        },
        ..custom_rail
    };

    let dragged_rail = scrollable::Rail {
        scroller: scrollable::Scroller {
            color: Color::from_rgb(0.6, 0.6, 0.9),
            ..custom_rail.scroller
        },
        ..custom_rail
    };

    match status {
        scrollable::Status::Active => scrollable::Style {
            container: default_style.container,
            vertical_rail: custom_rail,
            horizontal_rail: custom_rail,
            gap: default_style.gap,
        },
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered
        } => {
            let vertical_rail = if is_vertical_scrollbar_hovered {
                hovered_rail
            } else {
                custom_rail
            };

            let horizontal_rail = if is_horizontal_scrollbar_hovered {
                hovered_rail
            } else {
                custom_rail
            };

            scrollable::Style {
                container: default_style.container,
                vertical_rail,
                horizontal_rail,
                gap: default_style.gap,
            }
        },
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged
        } => {
            let vertical_rail = if is_vertical_scrollbar_dragged {
                dragged_rail
            } else {
                custom_rail
            };

            let horizontal_rail = if is_horizontal_scrollbar_dragged {
                dragged_rail
            } else {
                custom_rail
            };

            scrollable::Style {
                container: default_style.container,
                vertical_rail,
                horizontal_rail,
                gap: default_style.gap,
            }
        },
    }
}

fn main() -> iced::Result {
    iced::application(
        "Iced 0.13.1 Scrollbar 두께 조절 예제",
        ScrollableExample::update,
        ScrollableExample::view,
    )
        .run_with(|| (ScrollableExample::new(), iced::Task::none()))
}
