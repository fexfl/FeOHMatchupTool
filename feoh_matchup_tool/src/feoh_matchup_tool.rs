use iced::widget::{column, text, combo_box, container, scrollable, vertical_space};
use iced::{Alignment, Element, Sandbox, Length};
use crate::matchup_data_reader::{read_file as read_file, self};
use crate::matchup_data_reader::champion_struct::Champion as Champion;

pub struct MatchupTool {
    champions: combo_box::State<String>,
    champion_array: Vec<Champion>,
    selected_champion: Option<String>,
    text: String,
}

impl MatchupTool {
    fn get_champion_refs(champvec: &Vec<Champion>) -> Vec<&Champion> {
        let mut out_vec = vec![];
        for itr in champvec {
            out_vec.push(itr);
        }
        return out_vec;
    }

    fn create_id_array(champvec: &Vec<Champion>) -> Vec<usize> {
        let mut out_vec = vec![];
        for itr in champvec {
            out_vec.push(itr.id);
        }
        return out_vec;
    }

    fn create_name_array(champvec: &mut Vec<Champion>) -> Vec<String> {
        let mut out_vec = vec![];
        for itr in champvec {
            out_vec.push(itr.name);
        }
        return out_vec;
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Selected(String),
    OptionHovered(String),
    Closed,
}

impl Sandbox for MatchupTool {
    type Message = Message;

    fn new() -> Self {
        Self {
            champion_array: read_file(),
            champions: combo_box::State::new(MatchupTool::create_name_array(&mut read_file())),
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
            "Counters",
            text(&self.text),
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
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(obj) => {
                let index = self.champion_array.iter().position(|r| r.equals(&obj)).unwrap();
                self.text = self.champion_array[index].print_counters();
                self.selected_champion = Some(obj);
                self.champions.unfocus();
            }
            Message::OptionHovered(obj) => {
                let index = self.champion_array.iter().position(|r| r.equals(&obj)).unwrap();
                self.text = self.champion_array[index].print_counters();
            }
            Message::Closed => {
                //self.text = self
                    //.selected_champion
                    //.map(|champ| matchup_data_reader::champion_struct::Champion::get_champion_with_name(&champ, &self.champion_array).print_counters())
                    //.unwrap_or_default();
            }
        }
    }
}

/*
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
*/
