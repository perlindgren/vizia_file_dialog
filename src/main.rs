use std::env;

use bytesize::ByteSize;
use chrono::offset::Local;
use chrono::DateTime;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::fs::{self, FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use vizia::prelude::*;
use vizia::state::Data;

const THEME: &str = r#"

    label {
        background-color: white;
    }

    label:hover {
        background-color: grey;
    }

    label:checked {
        background-color: grey;
    }

    button:checked {
        background-color: grey;
    }

    .entry {
        background-color: blue;
    }
    
    .entry:hover {
        background-color: grey;
    }

    .entry:checked {
        background-color: grey;
    }

"#;

#[derive(Clone, Debug)]
pub struct DirEntryInfo {
    file_type: FileType,
    metadata: Metadata,
    file_name: OsString,
}
impl Display for DirEntryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name.clone().into_string().unwrap())
    }
}

impl Data for DirEntryInfo {
    fn same(&self, other: &Self) -> bool {
        self.file_name == other.file_name
    }
}

#[derive(Lens)]
pub struct AppData {
    pub path_list: Vec<String>, // path segments (may be longer than path_len)
    pub path_len: usize,        // length of current path_list
    pub entries: Vec<DirEntryInfo>, // files in current dir
    pub selected: usize,        // selected entry in current dir
    pub file: DirEntryInfo,
}

impl AppData {
    fn update_path(&mut self, index: usize) {
        println!("Path index{}", index);
        self.path_len = index;
        let mut new_path_list = self.path_list.clone();
        new_path_list.truncate(index);

        println!("new_path_list {:?}", new_path_list);
        let current_dir: PathBuf = new_path_list.iter().collect();
        println!("current_dir {:?}", current_dir);
        self.entries = folders(&current_dir).unwrap();
        println!("entries \n{:?}", self.entries);
    }
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    PathSelect(usize),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::KeyDown(code, _) => {
                if *code == Code::Space {
                    println!("Pressed Space key");
                }
            }

            _ => {}
        });

        event.map(|app_event, _| match app_event {
            AppEvent::Select(index) => {
                println!("select {}", index);
                let entry = self.entries.get(*index).unwrap();
                if entry.file_type.is_dir() {
                    self.path_list.truncate(self.path_len);
                    self.path_list
                        .push(entry.file_name.to_string_lossy().to_string());
                    self.update_path(self.path_list.len());
                    self.selected = 0;
                } else {
                    self.selected = *index;
                }
            }
            AppEvent::PathSelect(index) => {
                self.update_path(*index + 1);
            }
        })
    }
}

#[derive(Lens)]
pub struct FileData {
    text: String,
}

pub enum FileEvent {
    SetText(String),
}

impl Model for FileData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            FileEvent::SetText(text) => {
                self.text = text.clone();
            }
        });
    }
}

fn folders(dir: &Path) -> Result<Vec<DirEntryInfo>, io::Error> {
    fs::read_dir(dir)?
        .into_iter()
        .map(|result_entry| {
            result_entry.map(|entry| DirEntryInfo {
                file_type: entry.file_type().unwrap(), // fix
                metadata: entry.metadata().unwrap(),
                file_name: entry.file_name(),
            })
        })
        .collect()
}

fn main() {
    let current_dir = env::current_dir().unwrap();

    let current_dir_vec: Vec<String> = current_dir
        .iter()
        .map(|p| p.clone().to_str().unwrap().to_string())
        .collect();

    let path_strings = folders(&current_dir).unwrap();

    let first_sel = path_strings.get(0).unwrap().clone();

    Application::new(|cx| {
        Keymap::from(vec![(
            KeyChord::new(Modifiers::empty(), Code::Escape),
            KeymapEntry::new(Action::OnEsc, |_| println!("Escape")),
        )])
        .build(cx);

        FileData {
            text: String::new(),
        }
        .build(cx);

        AppData {
            path_len: current_dir_vec.len(),
            path_list: current_dir_vec,
            entries: path_strings,
            selected: 0,
            file: first_sel,
        }
        .build(cx);

        cx.add_theme(THEME);

        VStack::new(cx, |cx| {
            Textbox::new(cx, FileData::text)
                .on_edit(|_cx, text| {
                    println!("{:?}", text);
                })
                .width(Pixels(200.0))
                .height(Pixels(30.0));

            List::new(cx, AppData::path_list, |cx, index, item| {
                Button::new(
                    cx,
                    move |cx| {
                        println!("action {}", index);
                        cx.emit(AppEvent::PathSelect(index))
                    },
                    |cx| Label::new(cx, item),
                ) // Set the checked state based on whether this item is selected
                .checked(AppData::path_len.map(move |p_len| *p_len == index + 1));
            })
            .layout_type(LayoutType::Row)
            .col_between(Pixels(5.0));

            VStack::new(cx, |cx| {
                // Header
                HStack::new(cx, |cx| {
                    Label::new(cx, "Name").width(Pixels(400.0));
                    Label::new(cx, "Size").width(Pixels(100.0));
                    Label::new(cx, "Modified").width(Pixels(100.0));
                }).size(Auto);

                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    List::new(cx, AppData::entries, |cx, index, item| {
                        let info = item.get_val(cx);

                        HStack::new(cx, |cx| {
                            let l1 = Label::new(cx, item)
                                .on_press(move |cx| cx.emit(AppEvent::Select(index)))
                                .text_wrap(false)
                                .font_weight(if info.file_type.is_dir() {
                                    Weight::BOLD
                                } else {
                                    Weight::THIN
                                })
                                .width(Pixels(400.0))
                                .hoverable(false);

                            let mut size = "".to_string();
                            if info.file_type.is_dir() {
                                l1.color(
                                    Color::rgb(100, 100, 100), // TODO use Style
                                )
                                .font_weight(Weight::BOLD);
                            } else {
                                size = format!("{}", ByteSize::b(info.metadata.len()));
                            }

                            Label::new(cx, &size).width(Pixels(100.0)).hoverable(false);

                            let system_date: DateTime<Local> = SystemTime::now().into();
                            let modified_date: DateTime<Local> =
                                info.metadata.modified().unwrap().into();
                            let modified = if system_date.date_naive() == modified_date.date_naive()
                            {
                                format!("{}", modified_date.format("%T"))
                            } else {
                                format!("{}", modified_date.format("%d/%m/%Y"))
                            };
                            Label::new(cx, &modified).width(Pixels(100.0)).hoverable(false);
                        })
                        .class("entry")
                        // Set the checked state based on whether this item is selected       
                        .checked(AppData::selected.map(move |selected| *selected == index))
                        .on_press(move |cx| cx.emit(AppEvent::Select(index)));
                    })
                    .row_between(Pixels(2.0))
                    .on_double_click(|_, _| println!("double click"));       

                    // TODO increment/decrement to navigate directory entries
                    // .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
                    // .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection));
                    // .on_increment(move |cx| println!("increment"))
                    // .on_decrement(move |cx| println!("decrement"));
                });
                //.border_color(Color::black())
                //.border_radius(Pixels(2.0));
            })
            // .size(Auto)
            // .row_between(Pixels(5.0));
            ;
        });
    })
    .run();
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    OnEsc,
}
