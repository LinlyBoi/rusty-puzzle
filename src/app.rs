use std::{
    collections::{HashSet, VecDeque},
    thread,
    time::Duration,
};

use priority_queue::DoublePriorityQueue;

use crate::puzzlin::{
    heuristic::Heust,
    solvers::{solve_aystar, solve_bfs, solve_dfs, Solution},
    Puzzle,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RustyPuzzle {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    puzzle: Puzzle,
    #[serde(skip)]
    search_method: SearchMethod,
    #[serde(skip)]
    solution: Solution,
    #[serde(skip)]
    heut: Heust,
    time: Duration,
}
#[derive(Clone)]
pub enum SearchMethod {
    BFS,
    DFS,
    AYSTAR,
}

impl Default for RustyPuzzle {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            puzzle: Puzzle::default(),
            solution: Solution::default(),
            search_method: SearchMethod::BFS,
            heut: Heust::Mann,
            time: Duration::from_secs(0),
        }
    }
}

impl RustyPuzzle {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for RustyPuzzle {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            puzzle,
            search_method,
            solution,
            heut,
            time,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            ui.heading("Algorithm Selection");
            //
            //

            if ui
                .add(egui::RadioButton::new(
                    matches!(self.search_method, SearchMethod::BFS),
                    "BFS",
                ))
                .clicked()
            {
                self.search_method = SearchMethod::BFS
            }
            if ui
                .add(egui::RadioButton::new(
                    matches!(self.search_method, SearchMethod::DFS),
                    "DFS",
                ))
                .clicked()
            {
                self.search_method = SearchMethod::DFS
            }
            if ui
                .add(egui::RadioButton::new(
                    matches!(self.search_method, SearchMethod::AYSTAR),
                    "A*",
                ))
                .clicked()
            {
                self.search_method = SearchMethod::AYSTAR
            }
            ui.heading("Heuristic Selection");

            if ui
                .add(egui::RadioButton::new(
                    matches!(self.heut, Heust::Mann),
                    "MANN",
                ))
                .clicked()
            {
                self.heut = Heust::Mann
            }
            if ui
                .add(egui::RadioButton::new(
                    matches!(self.heut, Heust::Eucl),
                    "EUCLID",
                ))
                .clicked()
            {
                self.heut = Heust::Eucl
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.hyperlink_to("Github Repo", "https://github.com/LinlyBoi/rustypuzzle");
                    ui.label(".");
                });
            });
            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("wow this is not the 8puzle");
            egui::Grid::new("grid").show(ui, |ui| {
                for i in 0..self.puzzle.clone().getstate().row_len() {
                    for j in 0..self.puzzle.clone().getstate().row_len() {
                        _ = ui.button(format!(
                            " {} ",
                            self.puzzle.clone().getstate()[(i, j)].to_string()
                        ));
                    }
                    ui.end_row();
                }
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.add(egui::Button::new("Solve Puzzle")).clicked() {
                    match self.search_method {
                        SearchMethod::BFS => {
                            let mut frontier = VecDeque::new();
                            frontier.push_back(self.puzzle.clone());
                            self.solution =
                                solve_bfs(frontier, HashSet::new()).expect("NO SOLUTION");
                            self.puzzle = self.solution.goal_path.first().expect("A").clone();
                        }
                        SearchMethod::DFS => {
                            let mut frontier = VecDeque::new();
                            frontier.push_back(self.puzzle.clone());
                            self.solution =
                                solve_dfs(frontier, HashSet::new()).expect("NO SOLUTION");
                            self.puzzle = self.solution.goal_path.first().expect("A").clone();
                        }
                        SearchMethod::AYSTAR => {
                            let mut frontier: DoublePriorityQueue<Puzzle, usize> =
                                DoublePriorityQueue::new();

                            frontier.push(self.puzzle.clone(), 0);
                            self.solution =
                                solve_aystar(frontier, HashSet::new(), self.heut.clone())
                                    .expect("NO SOLUTION");
                            self.puzzle = self.solution.goal_path.first().expect("A").clone();
                        }
                    }
                }
                if ui.add(egui::Button::new("Traverse Path")).clicked() {
                    let mut path = self.solution.clone().get_path();
                    match path.pop() {
                        None => self.puzzle = Puzzle::default(),
                        Some(parent) => {
                            self.puzzle = parent;
                            self.solution.goal_path = path;
                        }
                    }
                }
                ui.label(format!("Cost: {}", self.solution.clone().get_cost()));
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.add(egui::Button::new("Test Case 1")).clicked() {
                    let rows = vec![vec![1, 2, 0], vec![3, 4, 5], vec![6, 7, 8]];
                    self.puzzle = Puzzle::new(rows);
                }
                if ui.add(egui::Button::new("Test Case 2")).clicked() {
                    let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
                    self.puzzle = Puzzle::new(rows);
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
