use iced::widget::{column, text, combo_box, container, scrollable, vertical_space};
use iced::{Alignment, Element, Sandbox, Length};

pub struct MatchupTool {
    champions: combo_box::State<Champion>,
    selected_champion: Option<Champion>,
    text: String,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    Selected(Champion),
    OptionHovered(Champion),
    Closed,
}

impl Sandbox for MatchupTool {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            champions: combo_box::State::new(Champion::ALL.to_vec()),
            selected_champion: None,
            text: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("FEOH Matchup Tool")
    }
    
    fn view(&self) -> Element<Message> {
        let combo_box = combo_box(
            &self.champions,
            "Select a champion...",
            self.selected_champion.as_ref(),
            Message::Selected,
        )
        .on_option_hovered(Message::OptionHovered)
        .on_close(Message::Closed)
        .width(250);

        let content = column![
            text(&self.text),
            "Champion",
            combo_box,
            vertical_space(150),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        //column![
        //    button("+").on_press(Message::IncrementPressed),
        //    text(self.value).size(50),
        //    button("-").on_press(Message::DecrementPressed),
        //]
        //.padding(20)
        //.align_items(Alignment::Center)
        //.into()
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Selected(champ) => {
                self.selected_champion = Some(champ);
                self.text = champ.test().to_string();
                self.champions.unfocus();
            }
            Message::OptionHovered(champ) => {
                self.text = champ.test().to_string();
            }
            Message::Closed => {
                self.text = self
                    .selected_champion
                    .map(|champ| champ.test().to_string())
                    .unwrap_or_default();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Champion {
    #[default]
    Aatrox,
    Ahri,
    Akali,
    Akshan,
    Alistar,
    Amumu,
    Anivia,
}

impl Champion {
    const ALL: [Champion; 7] = [
        Champion::Aatrox,
        Champion::Ahri,
        Champion::Akali,
        Champion::Akshan,
        Champion::Alistar,
        Champion::Amumu,
        Champion::Anivia,
    ];

    fn test(&self) -> &str {
        match self {
            Champion::Aatrox => "aa",
            Champion::Ahri => "ah",
            Champion::Akali => "ak",
            Champion::Akshan => "aks",
            Champion::Alistar => "al",
            Champion::Amumu => "am",
            Champion::Anivia => "an",
        }
    }
}

impl std::fmt::Display for Champion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Champion::Aatrox => "Aatrox",
                Champion::Ahri => "Ahri",
                Champion::Akali => "Akali",
                Champion::Akshan => "Akshan",
                Champion::Alistar => "Alistar",
                Champion::Amumu => "Amumu",
                Champion::Anivia => "Anivia",
            }
        )
    }
}
