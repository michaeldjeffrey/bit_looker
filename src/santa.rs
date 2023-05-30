use crate::app::MyStyles;
use egui_extras::Column;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
    phone_number: String,
    exclude: HashSet<String>,
}

impl Person {
    fn display_excludes(&self) -> String {
        self.exclude
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct State {
    people: Vec<Person>,
    matches: Vec<String>,
    exluding: Option<usize>,
}

impl State {
    pub fn side_panel(&mut self, _ui: &mut egui::Ui) {}

    pub fn main_view(&mut self, ui: &mut egui::Ui, _styles: &mut MyStyles) {
        ui.heading("Secret Santa");
        ui.add(display_actions(self));
        ui.add(display_people(self));

        ui.separator();
        ui.add(display_matches(self));
    }
}
enum Status {
    Inactive,                  // no one is exluding
    WeAreActive,               // we are excluding
    SomeoneElseActiveIncluded, // someone else active, we are included
    SomeoneElseActiveExcluded, // someone else active, we are excluded
}

fn t_heading(header: &mut egui_extras::TableRow<'_, '_>, text: &str) {
    header.col(|ui| {
        ui.heading(text);
    });
}

fn b_button(ui: &mut egui::Ui, text: &str, mut action: impl FnMut()) {
    if ui.button(text).clicked() {
        action();
    }
}
fn disliking_status(excluding: &Option<usize>, people: &[Person], person_idx: usize) -> Status {
    if let Some(active_idx) = excluding {
        if active_idx == &person_idx {
            Status::WeAreActive
        } else if people[*active_idx]
            .exclude
            .contains(&people[person_idx].name)
        {
            Status::SomeoneElseActiveExcluded
        } else {
            Status::SomeoneElseActiveIncluded
        }
    } else {
        Status::Inactive
    }
}

impl State {
    fn example_people() -> Vec<Person> {
        vec![
            Person {
                name: String::from("Alice"),
                phone_number: String::from("111-1111"),
                exclude: HashSet::from([String::from("Bob"), String::from("Charlie")]),
            },
            Person {
                name: String::from("Bob"),
                phone_number: String::from("222-2222"),
                exclude: HashSet::new(),
            },
            Person {
                name: String::from("Charlie"),
                phone_number: String::from("333-3333"),
                exclude: HashSet::new(),
            },
            Person {
                name: String::from("Dave"),
                phone_number: String::from("444-4444"),
                exclude: HashSet::new(),
            },
        ]
    }
}
fn display_people(state: &mut State) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        ui.vertical(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.
                egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::remainder())
                    .header(20.0, |mut header| {
                        t_heading(&mut header, "Name");
                        t_heading(&mut header, "Phone Number");
                        t_heading(&mut header, "Exclusions");
                    })
                    .body(|mut body| {
                        let mut active_dislikes = if let Some(active_idx) = state.exluding {
                            state.people[active_idx].exclude.clone()
                        } else {
                            Default::default()
                        };

                        let mut people = state.people.clone();

                        for (idx, person) in people.iter_mut().enumerate() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut person.name);
                                });
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut person.phone_number);
                                });
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        match disliking_status(&state.exluding, &state.people, idx)
                                        {
                                            Status::Inactive => {
                                                b_button(ui, "Start", || {
                                                    state.exluding = Some(idx)
                                                });
                                            }
                                            Status::WeAreActive => {
                                                b_button(ui, "Stop", || state.exluding = None);
                                            }
                                            Status::SomeoneElseActiveIncluded => {
                                                b_button(ui, "- Exclude", || {
                                                    active_dislikes.insert(person.name.clone());
                                                });
                                            }
                                            Status::SomeoneElseActiveExcluded => {
                                                b_button(ui, "+ Include", || {
                                                    active_dislikes.remove(&person.name);
                                                });
                                            }
                                        }

                                        ui.label(person.display_excludes());
                                    });
                                });
                            });
                        }

                        state.people = people;
                        if let Some(active_idx) = state.exluding {
                            state.people[active_idx].exclude = active_dislikes;
                        }
                    });
                if ui.button("Add Person").clicked() {
                    state.people.push(Person::default());
                }
            })
        })
        .response
    }
}

fn display_actions(state: &mut State) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            if ui.button("Use Example Data").clicked() {
                *state = Default::default();
                state.people = State::example_people();
            }

            if ui.button("Clear Data").clicked() {
                *state = Default::default();
            }

            if ui.button("Clear Exclusions").clicked() {
                for person in state.people.iter_mut() {
                    person.exclude.clear();
                }
            }
            if ui.button("Match").clicked() {
                state.matches = do_make_matches(&state.people);
            }
        })
        .response
    }
}

fn display_matches(state: &State) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        ui.vertical(|ui| {
            for matched in state.matches.iter() {
                ui.label(matched);
            }
        })
        .response
    }
}

fn do_make_matches(people: &[Person]) -> Vec<String> {
    for idx in 0..10 {
        match make_matches(people) {
            Ok(done) => return done,
            Err(e) => eprintln!("attempt {idx}: {e}"),
        }
    }
    vec![]
}

fn make_matches(people: &[Person]) -> Result<Vec<String>, String> {
    let mut phone_book: HashMap<String, String> = HashMap::new();
    let mut dislikes_map: HashMap<String, HashSet<String>> = HashMap::new();

    for person in people {
        phone_book.insert(person.name.clone(), person.phone_number.clone());
        dislikes_map.insert(person.name.clone(), person.exclude.clone());
    }

    let mut rng = rand::thread_rng();
    let mut available_people: Vec<String> = phone_book.clone().into_keys().collect();
    let mut starters: Vec<&String> = phone_book.keys().collect();

    starters.shuffle(&mut rng);
    available_people.shuffle(&mut rng);

    let mut matched = vec![];

    for starter in starters {
        let dislikes = dislikes_map.get(starter).unwrap();
        let available: Vec<String> = available_people
            .clone()
            .into_iter()
            .filter(|x| x != starter && !dislikes.contains(x))
            .collect();

        if available.is_empty() {
            return Err(format!("Unable to find a match for {}", starter));
        }

        let person_b = available.choose(&mut rng).unwrap();

        available_people.retain(|p| p != person_b);
        matched.push(format!("{starter}: {person_b}"));
    }

    Ok(matched)
}
