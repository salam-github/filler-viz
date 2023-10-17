use druid::{Data, Lens, Widget, WidgetExt, widget::{RadioGroup, Flex, Button}};
use druid::{AppLauncher, WindowDesc};

use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_map: String,
    selected_robot1: String,
    selected_robot2: String,
    maps: Arc<Vec<String>>,
    robots: Arc<Vec<String>>,
}

fn build_ui() -> impl Widget<AppState> {
    // Create the variants for the RadioGroup
    let map_variants = |data: &AppState| data.maps.iter().map(|m| (m.clone(), m.clone())).collect::<Vec<_>>();
let robot_variants = |data: &AppState| data.robots.iter().map(|r| (r.clone(), r.clone())).collect::<Vec<_>>();


    let map_radio = RadioGroup::new(map_variants).lens(AppState::selected_map);
    let robot1_radio = RadioGroup::new(robot_variants.clone()).lens(AppState::selected_robot1);
    let robot2_radio = RadioGroup::new(robot_variants).lens(AppState::selected_robot2);

    let run_button = Button::new("Run Game").on_click(|_ctx, data: &mut AppState, _env| {
        let command = format!("./linux_game_engine -f {} -p1 {} -p2 {}", data.selected_map, data.selected_robot1, data.selected_robot2);
        std::process::Command::new("sh").arg("-c").arg(command).status().unwrap();
    });

    Flex::column()
        .with_child(map_radio)
        .with_child(robot1_radio)
        .with_child(robot2_radio)
        .with_child(run_button)
}


fn main() {
    let maps = list_files("maps").unwrap();
    let robots = list_files("linux_robots").unwrap();

    let main_window = WindowDesc::new(build_ui)
        .title("Game Selection")
        .window_size((300.0, 200.0));

    let state = AppState {
        selected_map: maps[0].clone(),
        selected_robot1: robots[0].clone(),
        selected_robot2: robots[0].clone(),
        maps: Arc::new(maps),
        robots: Arc::new(robots),
    };

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}

fn list_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    std::fs::read_dir(dir)?.map(|entry| {
        let entry = entry?;
        Ok(entry.path().display().to_string())
    }).collect()
}
