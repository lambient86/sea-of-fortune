use crate::wind::components::CountdownTimer;
use crate::wind::components::Wind;
use bevy::prelude::*;
use rand::Rng;

pub fn init_wind(mut commands: Commands) {
    commands.insert_resource(Wind {
        direction: Vec2::new(0.0, 90.0),
    });
}

pub fn change_wind_dir(
    time: Res<Time>,
    mut query: Query<&mut CountdownTimer>,
    mut wind: ResMut<Wind>,
) {
    for mut countdown in query.iter_mut() {
        // Tick the timer
        countdown.timer.tick(time.delta());

        // Check if the timer has finished
        if countdown.timer.finished() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0.0..=360.0);
            let y = rng.gen_range(0.0..=360.0);
            wind.direction = Vec2::new(x, y);
            print!("changing wind {}\n", wind.direction);
        }
    }
}

pub fn init_timer(mut commands: Commands) {
    let timer = Timer::from_seconds(30.0, TimerMode::Repeating);

    commands.spawn(CountdownTimer { timer });
}
