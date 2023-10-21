use iced::widget::{column, row, text, combo_box, container, scrollable, vertical_space, image, Column, button, pick_list};
use iced::theme::Theme;
use iced::{Alignment, Element, Sandbox, Length};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::matchup_data_reader::champion_struct::{Champion, MatchupSafety, export_champ_to_raw};
use crate::matchup_data_reader::{read_file as read_file, self, write_file};

pub struct MatchupTool {
    champions: combo_box::State<ChampEnum>,
    all_champions: [ChampEnum; 165],
    selected_champion: Option<ChampEnum>,
    selected_matchup_safety: Option<MatchupSafety>,
    selected_champ_toadd: Option<ChampEnum>,
    text: String,
    champion_obj_array: Vec<Champion>,
    selected_image: image::Handle,
}

impl MatchupTool {
    pub fn get_champion_from_enum(&self, champ: ChampEnum) -> &Champion {
        let index = self.champion_obj_array.iter().position(|struct_obj| struct_obj.equals(&champ.to_string())).unwrap();
        return &self.champion_obj_array[index];
    }

    pub fn get_champion_index_from_enum(&self, champ: ChampEnum) -> usize {
        self.champion_obj_array.iter().position(|struct_obj| struct_obj.equals(&champ.to_string())).unwrap()
    }

    pub fn print_counters(&self, champ: &Champion) -> String {
        let mut out_string = String::from("");
        for cntr in &champ.counters {
            let (st, ms) = cntr;
            out_string.push_str(&st);
            out_string.push_str(" ");
            out_string.push_str(&ms.to_string());
            out_string.push_str("\n");
        }
        return out_string;
    }
    pub fn get_default_image(&self) -> image::Handle {
        let path: String = format!(".\\img\\default_image.png");
        image::Handle::from_path(path)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Selected(ChampEnum),
    OptionHovered(ChampEnum),
    Closed,
    CounterAdded(Option<ChampEnum>, Option<MatchupSafety>),
    CounterRemoved(Option<ChampEnum>),
    MatchupSafetySelected(MatchupSafety),
    ChampToAddSelected(ChampEnum),
}

impl Sandbox for MatchupTool {
    type Message = Message;

    fn new() -> Self {
        Self {
            champions: combo_box::State::new(ChampEnum::all().to_vec()),
            all_champions: ChampEnum::all(),
            selected_champion: None,
            selected_matchup_safety: None,
            selected_champ_toadd: None,
            text: String::new(),
            champion_obj_array: read_file(),
            selected_image: image::Handle::from_path(format![".\\img\\default_image.png"]),
        }
    }

    fn title(&self) -> String {
        String::from("FEOH Matchup Tool")
    }
    
    fn view(&self) -> Element<Message> {
        let combo_box = combo_box(
            &self.champions,
            "Select a champion!",
            self.selected_champion.as_ref(),
            Message::Selected,
        )
        .on_option_hovered(Message::OptionHovered)
        .on_close(Message::Closed)
        .width(250);

        let pick_list_ms  = pick_list(&MatchupSafety::ALL[..], self.selected_matchup_safety, Message::MatchupSafetySelected)
        .placeholder("Pick a matchup safety");

        let pick_list_champ = pick_list(&self.all_champions[..], self.selected_champ_toadd, Message::ChampToAddSelected)
        .placeholder("Pick a counter");

        let counterstring_itr = self.text.lines();
        let mut counters_column: Column<'_, Message> = column!().align_items(Alignment::Start).spacing(10).width(Length::Fill);

        if self.text.is_empty() {
            counters_column = counters_column.push(text("This champion has no counters!"));
        } else {
            for subitr in counterstring_itr {
                let mut champ_and_safety = subitr.split_whitespace();
                let champ = champ_and_safety.next().unwrap();

                counters_column = counters_column.push(row![
                    image::viewer(matchup_data_reader::champion_struct::get_champion_image_from_name(champ).clone()).width(Length::Fixed(32.)).height(Length::Fixed(32.)).scale_step(0.),
                    text(champ),
                    text(champ_and_safety.next().unwrap()),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10)
                );
            }
        } 

        let content = column![
            row![
                image::viewer(self.selected_image.clone()).height(Length::Fixed(32.)).width(Length::Fixed(32.)).scale_step(0.),
                combo_box,
                counters_column,
            ]
            //.height(Length::Shrink)
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(50),

            vertical_space(150),
            row![
                pick_list_champ,
                pick_list_ms,
                button("Add Counter").on_press(Message::CounterAdded(self.selected_champ_toadd, self.selected_matchup_safety)),
                button("Remove Counter").on_press(Message::CounterRemoved(self.selected_champ_toadd)),
            ],
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
                self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.champions.unfocus();
                self.selected_image = self.get_champion_from_enum(obj).get_champion_image();
            }
            Message::OptionHovered(obj) => {
                self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.selected_image = self.get_champion_from_enum(obj).get_champion_image();
            }
            Message::Closed => {
                self.text = "Select a champion".to_string();
                self.selected_image = self.get_default_image();
            }
            Message::CounterAdded(obj, ms) => {
                match (obj,ms,self.selected_champion) {
                    (Some(champ),Some(matchupst),Some(upper_champ)) => {
                        let idx = self.get_champion_index_from_enum(upper_champ);
                        self.champion_obj_array[idx].counters.push((champ.to_string(),matchupst));
                        self.update(Message::Selected(upper_champ));

                        write_file(export_champ_to_raw(&self.champion_obj_array));
                    },
                    _ => {
                        println!("Added nothing!");
                    },
                }
            }
            Message::CounterRemoved(obj) => {
                match (obj,self.selected_champion) {
                    (Some(champ),Some(upper_champ)) => {
                        let idx = self.get_champion_index_from_enum(upper_champ);
                        let counter_idx = self.champion_obj_array[idx].counters.iter().position(|(tuple_string,_tuple_ms)| tuple_string == (&champ.to_string())).unwrap();
                        self.champion_obj_array[idx].counters.remove(counter_idx);
                        self.update(Message::Selected(upper_champ));

                        write_file(export_champ_to_raw(&self.champion_obj_array));
                    },
                    _ => {
                        println!{"Removed nothing!"};
                    },
                }
            }
            Message::MatchupSafetySelected(ms) => {
                self.selected_matchup_safety = Some(ms);
            }
            Message::ChampToAddSelected(obj) => {
                self.selected_champ_toadd = Some(obj);
            }
        }
    }

    fn theme (&self) -> Theme {
        Theme::Dark
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum ChampEnum {
    #[default]
    Aatrox,
    Ahri,
    Akali,
    Akshan,
    Alistar,
    Amumu,
    Anivia,
    Annie,
    Aphelios,
    Ashe,
    AurelionSol,
    Azir,
    Bard,
    BelVeth,
    Blitzcrank,
    Brand,
    Braum,
    Briar,
    Caitlyn,
    Camille,
    Cassiopeia,
    ChoGath,
    Corki,
    Darius,
    Diana,
    Draven,
    DrMundo,
    Ekko,
    Elise,
    Evelynn,
    Ezreal,
    Fiddlesticks,
    Fiora,
    Fizz,
    Galio,
    Gangplank,
    Garen,
    Gnar,
    Gragas,
    Graves,
    Gwen,
    Hecarim,
    Heimerdinger,
    Illaoi,
    Irelia,
    Ivern,
    Janna,
    JarvanIV,
    Jax,
    Jayce,
    Jhin,
    Jinx,
    KaiSa,
    Kalista,
    Karma,
    Karthus,
    Kassadin,
    Katarina,
    Kayle,
    Kayn,
    Kennen,
    KhaZix,
    Kindred,
    Kled,
    KogMaw,
    KSante,
    LeBlanc,
    LeeSin,
    Leona,
    Lillia,
    Lissandra,
    Lucian,
    Lulu,
    Lux,
    Malphite,
    Malzahar,
    Maokai,
    MasterYi,
    Milio,
    MissFortune,
    Mordekaiser,
    Morgana,
    Naafiri,
    Nami,
    Nasus,
    Nautilus,
    Neeko,
    Nidalee,
    Nilah,
    Nocturne,
    NunuWillump,
    Olaf,
    Orianna,
    Ornn,
    Pantheon,
    Poppy,
    Pyke,
    Qiyana,
    Quinn,
    Rakan,
    Rammus,
    RekSai,
    Rell,
    RenataGlasc,
    Renekton,
    Rengar,
    Riven,
    Rumble,
    Ryze,
    Samira,
    Sejuani,
    Senna,
    Seraphine,
    Sett,
    Shaco,
    Shen,
    Shyvana,
    Singed,
    Sion,
    Sivir,
    Skarner,
    Sona,
    Soraka,
    Swain,
    Sylas,
    Syndra,
    TahmKench,
    Taliyah,
    Talon,
    Taric,
    Teemo,
    Thresh,
    Tristana,
    Trundle,
    Tryndamere,
    TwistedFate,
    Twitch,
    Udyr,
    Urgot,
    Varus,
    Vayne,
    Veigar,
    VelKoz,
    Vex,
    Vi,
    Viego,
    Viktor,
    Vladimir,
    Volibear,
    Warwick,
    Wukong,
    Xayah,
    Xerath,
    XinZhao,
    Yasuo,
    Yone,
    Yorick,
    Yuumi,
    Zac,
    Zed,
    Zeri,
    Ziggs,
    Zilean,
    Zoe,
    Zyra
}

impl ChampEnum {
    pub fn all() -> [ChampEnum; 165] {
        let mut out_array: [ChampEnum; 165] = [ChampEnum::Aatrox; 165];
        let mut index: usize = 0;
        for enmitr in ChampEnum::iter() {
            out_array[index] = enmitr;
            index += 1;
        }
        return out_array;
    }

    pub fn iconname_from_enum(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Display for ChampEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let outstring: &str = match self {
            ChampEnum::AurelionSol => "Aurelion Sol",
            ChampEnum::BelVeth => "Bel\u{0027}Veth",
            ChampEnum::ChoGath => "Cho\u{0027}Gath",
            ChampEnum::DrMundo => "Dr. Mundo",
            ChampEnum::JarvanIV => "Jarvan IV",
            ChampEnum::KaiSa => "Kai\u{0027}Sa",
            ChampEnum::KhaZix => "Kha\u{0027}Zix",
            ChampEnum::KogMaw => "Kog\u{0027}Maw",
            ChampEnum::KSante => "K\u{0027}Sante",
            ChampEnum::LeeSin => "Lee Sin",
            ChampEnum::MasterYi => "Master Yi",
            ChampEnum::MissFortune => "Miss Fortune",
            ChampEnum::NunuWillump => "Nunu & Willump",
            ChampEnum::RekSai => "Rek\u{0027}Sai",
            ChampEnum::RenataGlasc => "Renata Glasc",
            ChampEnum::TahmKench => "Tahm Kench",
            ChampEnum::TwistedFate => "Twisted Fate",
            ChampEnum::XinZhao => "Xin Zhao",
            _ => "None of the above",
        };
        if outstring == "None of the above" {
            return write!(
                f,
                "{:?}",
                self
            )
        } else {
            return write!(
                f,
                "{}",
                outstring
            )
        }
    }
}

