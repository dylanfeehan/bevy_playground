use bevy::prelude::*;


#[derive(Resource)]
struct GreetTimer(Timer);

// why is this struct Person and not struct Person(u64) like it says in the docs? smh
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Dylan Feehan".to_string())));
    commands.spawn((Person, Name("Richie Goulazian".to_string())));
    commands.spawn((Person, Name("Beverly".to_string())));
}


// Query is a system parameter that provides a selective access to the Component data stored in a world.. 
// component data, remember componets are uhhhh fucking like uhhh things that entities have i guess :
// Query &Name with <Person> ,,, what is person?
/*
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        // it's a tuple struct, so we access it's only data member with name.0
        println!("Hello {}!", name.0);
    }
}
*/

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(self: &Self, app: &mut App) {
        app
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0)
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run();
}
