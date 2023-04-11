use bytesize::ByteSize;
use chrono::{offset::Local, DateTime};
use std::env;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::fs::{self, FileType, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::state::Data;

#[derive(Clone, Debug, Lens)]
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Sorting {
    NameUp,
    NameDown,
    SizeUp,
    SizeDown,
    DateUp,
    DateDown,
}

impl Data for Sorting {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Lens)]
pub struct AppData {
    pub path_list: Vec<String>, // path segments (may be longer than path_len)
    pub path_len: usize,        // length of current path_list
    pub entries: Vec<DirEntryInfo>, // files in current dir
    pub selected: usize,        // selected entry in current dir
    pub file: DirEntryInfo,
    pub sorting: Sorting,
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
        self.sort();
        println!("entries \n{:?}", self.entries);
    }

    fn sort(&mut self) {
        self.entries.sort_by(|a, b| match self.sorting {
            Sorting::NameDown => a.file_name.cmp(&b.file_name),
            Sorting::NameUp => b.file_name.cmp(&a.file_name),
            Sorting::SizeUp => a.metadata.len().cmp(&b.metadata.len()),
            Sorting::SizeDown => b.metadata.len().cmp(&a.metadata.len()),

            // not sure if comparisons on SystemTime works
            Sorting::DateUp => a
                .metadata
                .modified()
                .unwrap()
                .cmp(&b.metadata.modified().unwrap()),
            Sorting::DateDown => b
                .metadata
                .modified()
                .unwrap()
                .cmp(&a.metadata.modified().unwrap()),
        });
    }
}

#[derive(Debug)]
pub enum AppEvent {
    Select(usize),
    PathSelect(usize),
    SortName,
    SortSize,
    SortDate,
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
            AppEvent::SortName => {
                self.sorting = match self.sorting {
                    Sorting::NameUp => Sorting::NameDown,
                    _ => Sorting::NameUp,
                };
                self.sort();
            }
            AppEvent::SortSize => {
                self.sorting = match self.sorting {
                    Sorting::SizeUp => Sorting::SizeDown,
                    _ => Sorting::SizeUp,
                };
                self.sort();
            }
            AppEvent::SortDate => {
                self.sorting = match self.sorting {
                    Sorting::DateUp => Sorting::DateDown,
                    _ => Sorting::DateUp,
                };
                self.sort();
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

        let mut app_data = AppData {
            path_len: current_dir_vec.len(),
            path_list: current_dir_vec,
            entries: path_strings,
            selected: 0,
            file: first_sel,
            sorting: Sorting::NameDown,
        };

        app_data.sort();
        app_data.build(cx);

        cx.add_stylesheet("resources/file_dialog.css").unwrap();

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
                let min_width_name = 200.0;
                let min_width_size = 50.0;
                let min_width_modified = 60.0;

                // Header
                HStack::new(cx, |cx| {
                    // Name
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::SortName);
                        },
                        |cx| {
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Name").size(Auto);
                                Label::new(
                                    cx,
                                    AppData::sorting.map(move |sorting| match sorting {
                                        Sorting::NameUp => UP,
                                        Sorting::NameDown => DOWN,
                                        _ => MINUS,
                                    }),
                                )
                                .class("icon")
                                .right(Pixels(0.0));
                            })
                            .col_between(Stretch(1.0))
                        },
                    )
                    .min_width(Pixels(min_width_name))
                    .width(Stretch(0.7));

                    // Size
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::SortSize);
                        },
                        |cx| {
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Size").size(Auto);
                                Label::new(
                                    cx,
                                    AppData::sorting.map(move |sorting| match sorting {
                                        Sorting::SizeUp => UP,
                                        Sorting::SizeDown => DOWN,
                                        _ => MINUS,
                                    }),
                                )
                                .class("icon")
                                .right(Pixels(0.0));
                            })
                            .col_between(Stretch(1.0))
                        },
                    )
                    .min_width(Pixels(min_width_size))
                    .width(Stretch(0.1));

                    // Modified
                    Button::new(
                        cx,
                        |cx| {
                            cx.emit(AppEvent::SortDate);
                        },
                        |cx| {
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Modified").size(Auto);
                                Label::new(
                                    cx,
                                    AppData::sorting.map(move |sorting| match sorting {
                                        Sorting::DateUp => UP,
                                        Sorting::DateDown => DOWN,
                                        _ => MINUS,
                                    }),
                                )
                                .class("icon")
                                .right(Pixels(0.0));
                            })
                            .col_between(Stretch(1.0))
                        },
                    )
                    .min_width(Pixels(min_width_modified))
                    .width(Stretch(0.2));
                })
                .col_between(Pixels(3.0))
                .right(Pixels(20.0))
                .width(Stretch(1.0))
                .height(Auto);

                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    VStack::new(cx, |cx| {
                        Binding::new(cx, AppData::entries, move |cx, list_lens| {
                            for (index, item) in list_lens.get(cx).iter().enumerate() {
                                HStack::new(cx, |cx| {
                                    // Name
                                    let symlink = format!("{}@", item.file_name.to_str().unwrap());
                                    let l1 = Label::new(
                                        cx,
                                        if item.file_type.is_symlink() {
                                            &symlink
                                        } else {
                                            item.file_name.to_str().unwrap()
                                        },
                                    )
                                    .min_width(Pixels(min_width_name))
                                    .text_wrap(false)
                                    .width(Stretch(0.7))
                                    .hoverable(false);

                                    let mut size = "".to_string();
                                    if item.file_type.is_file() {
                                        size = format!("{}", ByteSize::b(item.metadata.len()));
                                    } else {
                                        l1.color(if item.file_type.is_symlink() {
                                            Color::rgb(80, 80, 80) // TODO use Style
                                        } else {
                                            Color::rgb(100, 100, 100) // TODO use Style
                                        })
                                        .font_weight(Weight::BOLD);
                                    }

                                    // Size
                                    Label::new(cx, &size)
                                        .min_width(Pixels(min_width_size))
                                        .text_wrap(false)
                                        .width(Stretch(0.1))
                                        .hoverable(false);

                                    // Modified
                                    let system_date: DateTime<Local> = SystemTime::now().into();
                                    let modified_date: DateTime<Local> =
                                        item.metadata.modified().unwrap().into();
                                    let modified =
                                        if system_date.date_naive() == modified_date.date_naive() {
                                            format!("{}", modified_date.format("%T"))
                                        } else {
                                            format!("{}", modified_date.format("%d/%m/%Y"))
                                        };
                                    Label::new(cx, &modified)
                                        .right(Pixels(0.0))
                                        .min_width(Pixels(min_width_modified))
                                        .text_wrap(false)
                                        .width(Stretch(0.2))
                                        .hoverable(false);
                                })
                                .col_between(Pixels(3.0))
                                .right(Pixels(20.0))
                                .class("entry")
                                .checked(AppData::selected.map(move |selected| *selected == index))
                                .on_press(move |cx| cx.emit(AppEvent::Select(index)));
                            }
                        });
                    })
                    .row_between(Pixels(2.0));
                    //     // TODO increment/decrement to navigate directory entries
                    //     // .on_increment(move |cx| cx.emit(AppEvent::IncrementSelection))
                    //     // .on_decrement(move |cx| cx.emit(AppEvent::DecrementSelection));
                    //.on_increment(move |cx| println!("increment"))
                    //.on_decrement(move |cx| println!("decrement"));
                });
                //.border_color(Color::black())
                //.border_radius(Pixels(2.0));
            });
            //.size(Auto);
            //.top(Pixels(0.0));
            // .row_between(Pixels(5.0));
        });
    })
    .run();
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    OnEsc,
}
