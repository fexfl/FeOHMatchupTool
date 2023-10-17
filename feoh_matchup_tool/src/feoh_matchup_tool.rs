use iced::widget::{column, text, combo_box, container, scrollable, vertical_space};
use iced::{Alignment, Element, Sandbox, Length};
//use crate::matchup_data_reader::champion_struct::Champion;
//use crate::matchup_data_reader::read_file as read_file;

pub struct MatchupTool {
    champions: combo_box::State<ChampEnum>,
    selected_champion: Option<ChampEnum>,
    text: String,
    //champion_obj_array: Vec<Champion>,
}

impl MatchupTool {
    /*
    pub fn get_champion_from_enum(&self, champ: ChampEnum) -> &Champion {
        let index = self.champion_obj_array.iter().position(|struct_obj| struct_obj.equals(&champ.to_string())).unwrap();
        return &self.champion_obj_array[index];
    }
    */
    /*
    pub fn print_counters(&self, champ: &Champion) -> String {
        let mut out_string = String::from("Counters:");
        for cntr in &champ.counters {
            let (st, _ms) = cntr;
            out_string.push_str(" ");
            out_string.push_str(&st);
        }
        return out_string;
    }
    */
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Selected(ChampEnum),
    OptionHovered(ChampEnum),
    Closed,
}

impl Sandbox for MatchupTool {
    type Message = Message;

    fn new() -> Self {
        Self {
            champions: combo_box::State::new(ChampEnum::ALL.to_vec()),
            selected_champion: None,
            text: String::new(),
            //champion_obj_array: read_file(),
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
            "Counters",
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
                self.selected_champion = Some(obj);
                //self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.text = obj.to_string();
                self.champions.unfocus();
            }
            Message::OptionHovered(obj) => {
                //self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.text = obj.to_string();
            }
            Message::Closed => {
                //self.text = self
                    //.selected_champion
                    //.map(|champ| matchup_data_reader::champion_struct::Champion::get_champion_with_name(&champ, &self.champion_array).print_counters())
                    //.unwrap_or_default();
                self.text = self
                    .selected_champion
                    .map(|champ| champ.hello())
                    .unwrap_or_default()
            }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChampEnum {
    #[default]
    Aatrox,
    Ahri,
    Akali,
    Akshan,
    Alistar,
    Amumu,
    Anivia,
}

impl ChampEnum {
    const ALL: [ChampEnum; 7] = [
        ChampEnum::Aatrox,
        ChampEnum::Ahri,
        ChampEnum::Akali,
        ChampEnum::Akshan,
        ChampEnum::Alistar,
        ChampEnum::Amumu,
        ChampEnum::Anivia,
    ];

    fn hello(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for ChampEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self
        )
    }
}

