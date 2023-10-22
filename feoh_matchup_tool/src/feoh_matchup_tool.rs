use std::path::PathBuf;
use std::vec;

use iced::widget::{column, row, text, combo_box, container, scrollable, horizontal_space, image, Column, button, pick_list, ComboBox, vertical_space, text_input};
use iced::theme::Theme;
use iced::{Alignment, Element, Application, Length, Command, Renderer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::matchup_data_reader::champion_struct::{Champion, MatchupSafety, export_champ_to_raw, get_images_with_ownership};
use crate::matchup_data_reader::{read_file as read_file, write_file, self};

pub struct MatchupTool {
    champions: combo_box::State<ChampEnum>,
    selected_champion: Option<ChampEnum>,
    hovered_champion: Option<ChampEnum>,
    selected_matchup_safety: Option<MatchupSafety>,
    selected_champ_toadd: Option<ChampEnum>,
    champ_toadd_cbs: combo_box::State<ChampEnum>,
    text: String,
    champion_obj_array: Vec<Champion>,
    selected_image: image::Handle,
    counters_images: Vec<image::Handle>,
    c_loaded: CountersLoaded,
    old_file_location_string: String,
}

pub enum CountersLoaded {
    Loading,
    Loaded,
    Errored,
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
        let path: String = format!(".\\img\\system\\default_image.png");
        image::Handle::from_path(path)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Selected(ChampEnum),
    IconFound(Result<(image::Handle,Vec<image::Handle>),FeohError>),
    OptionHovered(ChampEnum),
    Closed,
    CounterAdded(Option<ChampEnum>, Option<MatchupSafety>),
    CounterRemoved(Option<ChampEnum>),
    MatchupSafetySelected(MatchupSafety),
    ChampToAddSelected(ChampEnum),
    ChampToAddClosed,
    ImportOldData(String),
    OldFileLocationChanged(String),
}

#[derive(Debug, Clone)]
pub enum FeohError {
    APIError,
}

impl From<reqwest::Error> for FeohError {
    fn from(error: reqwest::Error) -> FeohError {
        dbg!(error);

        FeohError::APIError
    }
}

impl Application for MatchupTool {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Self {
            champions: combo_box::State::new(ChampEnum::all().to_vec()),
            selected_champion: None,
            hovered_champion: None,
            selected_matchup_safety: None,
            selected_champ_toadd: None,
            champ_toadd_cbs: combo_box::State::new(ChampEnum::all().to_vec()),
            text: String::new(),
            champion_obj_array: read_file(),
            selected_image: image::Handle::from_path(format![".\\img\\system\\default_image.png"]),
            counters_images: vec![],
            c_loaded: CountersLoaded::Loaded,
            old_file_location_string: "Location to import".to_string(),
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("FEOH Matchup Tool")
    }
    
    fn view(&self) -> Element<Message> {
        let combo_box_main = combo_box(
            &self.champions,
            "Select a champion!",
            self.selected_champion.as_ref(),
            Message::Selected,
        )
        .on_option_hovered(Message::OptionHovered)
        .on_close(Message::Closed)
        .width(250);

        let pick_list_ms = pick_list(&MatchupSafety::ALL[..], self.selected_matchup_safety, Message::MatchupSafetySelected)
        .placeholder("Pick a matchup safety");

        let cmb_box_champ: ComboBox<'_, ChampEnum, Message> = combo_box(
            &self.champ_toadd_cbs,
            "Select a counter", 
            self.selected_champ_toadd.as_ref(), 
            Message::ChampToAddSelected
        )
        .width(250)
        .on_close(Message::ChampToAddClosed);

        let counterstring_itr = self.text.lines();
        let mut counters_column: Column<'_, Message> = column!().align_items(Alignment::Start).spacing(10).width(Length::Fill);
        let mut counterimages_itr = self.counters_images.iter();

        match self.c_loaded {
            CountersLoaded::Loaded =>
                if self.text.is_empty() {
                    counters_column = counters_column.push(text("This champion has no counters!"));
                } else {
                    counters_column = counters_column.push(row![
                        image::viewer(self.selected_image.clone()).width(Length::Fixed(32.)).height(Length::Fixed(32.)).scale_step(0.),
                        text(format!("{} Counters: ", self.selected_champion.unwrap())).size(24),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding(0),
                    );
                    counters_column = counters_column.push(vertical_space(30));
                    for subitr in counterstring_itr {
                        let mut champ_and_safety = subitr.split_whitespace();
                        let champ = champ_and_safety.next().unwrap();
                        let sfty = champ_and_safety.next().unwrap();
                        let img = counterimages_itr.next();
        
                        counters_column = counters_column.push(row![
                            image::viewer(img.unwrap().clone()).width(Length::Fixed(32.)).height(Length::Fixed(32.)).scale_step(0.),
                            text(champ),
                            text(" - "),
                            text(sfty),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(10)
                        .padding(0)
                        );
                    }
                },
            CountersLoaded::Loading => 
                counters_column = counters_column.push(text("Loading...")),
            CountersLoaded::Errored => 
                counters_column = counters_column.push(text("Error!")),
        }
        let ofl = &self.old_file_location_string;
        let old_data_file_location_input: iced::widget::TextInput<'_, Message, Renderer> = text_input("Location to import", ofl)
        .on_input(Message::OldFileLocationChanged(ofl));
        

        let content = column![
            row![
                image::viewer(image::Handle::from_path(".\\img\\system\\icon.png").clone()).height(Length::Fixed(128.)).width(Length::Fixed(128.)).scale_step(0.),
                text("FeOH Matchup Tool").size(60),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
            row![
                image::viewer(self.selected_image.clone()).height(Length::Fixed(32.)).width(Length::Fixed(32.)).scale_step(0.),
                combo_box_main,
                horizontal_space(150),
                cmb_box_champ,
                pick_list_ms,
                button("Add Counter").on_press(Message::CounterAdded(self.selected_champ_toadd, self.selected_matchup_safety)),
                button("Remove Counter").on_press(Message::CounterRemoved(self.selected_champ_toadd)),
            ]
            //.height(Length::Shrink)
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(50),
            row![
                horizontal_space(450),
                old_data_file_location_input,
                button("Import old data format file").on_press(Message::ImportOldData(ofl.to_string())),
            ]
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(50),
            row![
                counters_column,
            ]
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(50),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            //.center_y()
            .into()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Selected(obj) => {
                // Preparing for loading
                self.c_loaded = CountersLoaded::Loading;
                self.selected_image = self.get_default_image();

                // Initiiating loading
                self.selected_champion = Some(obj);
                self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.champions.unfocus();
                let champ_ref = self.get_champion_from_enum(obj).clone();
                Command::perform(get_images_with_ownership(champ_ref), Message::IconFound)
            }
            Message::IconFound(res) => {
                let (path,counter_paths) = match res {
                    Ok(byt) => byt,
                    Err(_err) => panic!("Error! Image couldnt be created from bytes!"),
                };
                self.selected_image = path;
                self.counters_images = counter_paths;
                self.c_loaded = CountersLoaded::Loaded;
                Command::none()
            }
            Message::OptionHovered(obj) => {
                //self.text = self.print_counters(self.get_champion_from_enum(obj)).to_string();
                self.hovered_champion = Some(obj);
                //let champ_ref = self.get_champion_from_enum(obj).clone();
                //Command::perform(get_images_with_ownership(champ_ref), Message::IconFound)
                Command::none()
            }
            Message::Closed => {
                self.text = "Select a champion".to_string();
                self.selected_image = self.get_default_image();
                Command::none()
            }
            Message::CounterAdded(obj, ms) => {
                match (obj,ms,self.selected_champion) {
                    (Some(champ),Some(matchupst),Some(upper_champ)) => {
                        let idx = self.get_champion_index_from_enum(upper_champ);
                        self.champion_obj_array[idx].counters.push((champ.to_string(),matchupst));

                        write_file(export_champ_to_raw(&self.champion_obj_array));
                        
                        self.update(Message::Selected(upper_champ))

                    },
                    _ => {
                        println!("Added nothing!");
                        Command::none()
                    },
                }
            }
            Message::CounterRemoved(obj) => {
                match (obj,self.selected_champion) {
                    (Some(champ),Some(upper_champ)) => {
                        let idx = self.get_champion_index_from_enum(upper_champ);
                        let counter_idx = self.champion_obj_array[idx].counters.iter().position(|(tuple_string,_tuple_ms)| tuple_string == (&champ.to_string())).unwrap();
                        self.champion_obj_array[idx].counters.remove(counter_idx);

                        write_file(export_champ_to_raw(&self.champion_obj_array));

                        self.update(Message::Selected(upper_champ))

                        
                    },
                    _ => {
                        println!{"Removed nothing!"};
                        Command::none()
                    },
                }
                
            }
            Message::MatchupSafetySelected(ms) => {
                self.selected_matchup_safety = Some(ms);
                Command::none()
            }
            Message::ChampToAddSelected(obj) => {
                self.selected_champ_toadd = Some(obj);
                self.champ_toadd_cbs.unfocus();
                Command::none()
            }
            Message::ChampToAddClosed => {
                Command::none()
            }
            Message::ImportOldData(pathstr) => {
                println!("{}",pathstr);
                /*
                let path = PathBuf::from(pathstr);
                matchup_data_reader::import_old_data_file(&mut self.champion_obj_array, path);
                write_file(export_champ_to_raw(&self.champion_obj_array));
                */
                self.update(Message::Closed)
            }
            Message::OldFileLocationChanged(ofl) => {
                self.old_file_location_string = ofl;
                Command::none()
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
        match self {
            ChampEnum::Wukong => "MonkeyKing".to_string(),
            ChampEnum::NunuWillump => "Nunu".to_string(),
            ChampEnum::KhaZix => "Khazix".to_string(),
            ChampEnum::ChoGath => "Chogath".to_string(),
            ChampEnum::BelVeth => "Belveth".to_string(),
            ChampEnum::KaiSa => "Kaisa".to_string(),
            ChampEnum::LeBlanc => "Leblanc".to_string(),
            ChampEnum::RenataGlasc => "Renata".to_string(),
            ChampEnum::VelKoz => "Velkoz".to_string(),
            _ => format!("{:?}", self),
        }
        
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
            ChampEnum::VelKoz => "Vel\u{0027}Koz",
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

