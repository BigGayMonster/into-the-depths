use crate::event::Event;

struct Room {
    width: u8,
    height: u8,
    event: Event,
    color: String,
}

struct Level(Vec<Room>);
struct Levels(Vec<Level>);

pub fn render() {}
