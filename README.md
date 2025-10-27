# Iced 사용해보기

일단 문서화가 거의 안되어있음. 확실히 Slint이랑 비교해보면 프로덕션 레벨은 아닌 듯.

메인테이너 한명이서 혼자 개발하는거고 릴리즈도 자주 하지 않아서 좀 유지보수 생각하면 별로 좋은 선택은 아닐수도? 
그래도 system76에서 COSMIC Desktop Environment(GNOME 대체용)에서 자체 앱들 UI에 iced를 사용한다는거 보면 그나마 Rust 진영에서는 쓸만한 거 같음.
https://blog.system76.com/post/cosmic-team-interview-byoux

gtk 포팅해서 쓰는 것도 방법인데, 이건 리눅스 지원이 잘 되는거고 크로스 컴파일 기능이 좀 떨어진다고 함. (slint나 iced 대비)

Rust 진영에서 데스트톱 앱 개발하려면 tauri, slint, iced, egui, gtk-rs 요정도가 있고, 거기에 좀 애매한것도 하면 flutter + rust 나 godot 같은 게임엔진 정도가 있을 듯.

내가 원하던 크로스컴파일, 디자인 자유롭게, 괜찮은 성능을 고려하면 iced가 제일 나은 선택이라고 생각함.

Slint 대비 장점이라면 Rust로 UI가 되어있어서 가능한 UI 설정 옵션을 IDE로 뜯어보기 쉽다는거? 그래서 문서가 좀 부족해도 바로바로 찾아볼 수 있음.
Slint 언어가 가독성은 좋아보이는데, IDE 지원이 떨어저서 개발 경험이 좋지 않았음. 문법 자동완성도 없었고. + 자료형 부족. 정수형 타입이 int(4바이트)만 있었음

---

이거 문서가 잘 되어있음.
https://github.com/fogarecious/iced_tutorial/blob/main/README.md
https://jl710.github.io/iced-guide/index.html << 이건 안읽어보긴 함.
---

근데 갑자기 든 생각은 Elm이야 함수형이라 괜찮았는데, Rust의 경우 view 가 이상하게 구현되거나 Message를 update에서 전부 처리하지 않아도 컴파일 시점에 못 막는거 아닌가?

Rust AST 처리하는 clippy 같은 도구 만들어서 연결하면 될 거 같기도 한데?

그 외에도 생각되는 문제가
1. Rust에서 Nested TEA를 그대로 구현하면 MyApp을 수정 가능한 문제
   - Elm의 Nested TEA에선 자식이 상위의 상태를 변경할 수 없지만. 
   - Rust에선 update에서 MyApp `&mut`을 받게 되므로 자식에서 부모의 Model 상태를 변경할 수 있다.   
   - 이건 trait 방식의 문제로 보임.
2. Model이 불변이 아니다
    - 대신 view() 에선 `&`로 수정되지 않게 함.
    - 이렇게 된 이유는 Rust는 순수 함수형 언어가 아니라, 불변 데이터를 새로 만들어도 값을 재사용하는 기능이 없어서 메모리 낭비가 심해지기 때문임.
    - 이렇게 되면 동시성 문제가 생길 수 있는거 아닌가? Msg를 처리하는게 싱글 스레드라면 괜찮을지도?

암튼 그래서 Page 트레잇에 MyApp에서 dyn으로 보는게 1번 문제를 해결할 수 있음. 대신 Elm과 다르게 코드만 보고 동작이 명확하지 않게 됨.

Model이 불변이 아닌건 단점이 맞을 듯. 그리고 그냥 구조체만 관리하는 방식 같은데, 이러면 Undo 기능도 없는거 아닌가? 뭐 쓸일이 있을진 모르겠지만.  

---

## 문법 메모

- 함수 파라미터 이름 앞에 `_` 붙은건 본문에서 사용하지 않음을 나타냄. 시그니처 상 필요하지만 경고를 막고싶은 경우 사용. (Rust 문법인데 찾아본김에...)

- 기본적인 사용법이나 레이아웃 관련된건 그냥 목차 보고 열기. 그리 복잡하진 않음.
- 상태관리 같은건 TEA 알고있으니까 넘어가도 됨.
- 실행하기
  - `iced::run`: `application().run()`의 축약형. 간단한 앱용.
  - `iced::application`: Application 인스턴스 반환. 초기화 커스터마이징 가능. 
    - `run_with()`: 시작 시 Task 실행
    - `theme()`: 테마 설정
- Task
  - 비동기 작업. 동시 작업(concurrent operations) 처리를 위해 사용. update에서 반환 타입으로 사용됨.
  - Elm의 `Cmd Msg`와 Iced의 `Task<Message>` 가 동일한 역할이라 보면 됨.
- Subscription
  - 특정 이벤트 발생 시 Message로 변환하는 역할
  - Elm의 `Sub Msg`와 `Subscription<Message>`가 동일한 역할의 타입 
  - `listen_with()`으로 이벤트를 받고, 인자인 클로저에서 적절한 코드로 분기처리하는게 일반적인 패턴으로 보임
- Custom Widgets
  - 이거 유지보수나 깔끔하게 개발하려면 필요하긴 할 듯?
  - 여기저기 쓰이는 UI 아니면 그냥 for문이 더 나으려나?
  - 아니면 다른 예시 더 찾아보고
    - 커스텀 위젯 구현 예시로 괜찮아보임: https://github.com/iced-rs/iced_aw
