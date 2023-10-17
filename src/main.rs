use druid::{AppLauncher, Widget, WindowDesc, EventCtx};
use druid::widget::{Label, Button, Flex};
use druid::Data;
use druid::Lens;
use druid::widget::RadioGroup;
use druid::WidgetExt;

// Hard-code the available options
const AVAILABLE_MAPS: [&str; 3] = ["map01", "map02", "map03"];
const AVAILABLE_ROBOTS: [&str; 4] = ["bender", "h2_d2", "terminator", "wall_e"];

#[derive(Clone, Data, Lens)]
struct AppState {
    selected_map: usize,
    selected_robot1: usize,
    selected_robot2: usize,
    message: String,
}

fn build_ui() -> impl Widget<AppState> {
    // Use the hard-coded options for the RadioGroup widgets
    let map_variants = AVAILABLE_MAPS.iter().enumerate().map(|(i, map)| (map.to_string(), i)).collect::<Vec<_>>();
    let robot_variants = AVAILABLE_ROBOTS.iter().enumerate().map(|(i, robot)| (robot.to_string(), i)).collect::<Vec<_>>();

    let map_radio = RadioGroup::new(map_variants).lens(AppState::selected_map);
    let robot1_radio = RadioGroup::new(robot_variants.clone()).lens(AppState::selected_robot1);
    let robot2_radio = RadioGroup::new(robot_variants).lens(AppState::selected_robot2);

    let start_game_button = Button::new("Start Game")
        .on_click(move |_ctx, data: &mut AppState, _env| {
            let command = format!("./linux_game_engine -f maps/{} -p1 linux_robots/{} -p2 linux_robots/{}",
                                  AVAILABLE_MAPS[data.selected_map],
                                  AVAILABLE_ROBOTS[data.selected_robot1],
                                  AVAILABLE_ROBOTS[data.selected_robot2]);
            //std::process::Command::new("sh").arg("-c").arg(command).status().unwrap();
            data.message = format!("Selected Map: {}\nPlayer 1 Robot: {}\nPlayer 2 Robot: {}",
                                   AVAILABLE_MAPS[data.selected_map],
                                   AVAILABLE_ROBOTS[data.selected_robot1],
                                   AVAILABLE_ROBOTS[data.selected_robot2]);
        });


    Flex::column()
        .with_child(Label::new("Select map:"))
        .with_child(map_radio)
        .with_spacer(8.0)
        .with_child(Label::new("Select player 1 robot:"))
        .with_child(robot1_radio)
        .with_spacer(8.0)
        .with_child(Label::new("Select player 2 robot:"))
        .with_child(robot2_radio)
        .with_spacer(8.0)
        .with_child(start_game_button)
        .with_spacer(8.0)
        .with_child(Label::new(|data: &AppState, _env: &_| data.message.clone()))
}

fn main() {
    let state = AppState {
        selected_map: 0,
        selected_robot1: 0,
        selected_robot2: 0,
        message: "selection text".to_string(),
    };

    let main_window = WindowDesc::new(build_ui)
        .title("Filler Game")
        .window_size((500.0, 600.0));

    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch application");
}
