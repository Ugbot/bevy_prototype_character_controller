use bevy::prelude::*;
use bevy_prototype_character_controller::controller::{controller_to_pitch, controller_to_yaw};

// Take a look at example_utils/utils.rs for details!
#[path = "../example_utils/utils.rs"]
mod utils;
use utils::{build_app, controller_to_kinematic, CharacterSettings};

fn main() {
    let mut app = App::new();
    build_app(&mut app);
 
    app.init_resource::<CharacterSettings>()
        .add_system(controller_to_kinematic)
        .add_system(controller_to_yaw)
        .add_system(controller_to_pitch)
        .run();
}
