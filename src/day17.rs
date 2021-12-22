use crossbeam_channel::{unbounded, Sender};
use legion::systems::CommandBuffer;
use legion::{system, Entity, IntoQuery, Resources, Schedule, World};
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone)]
struct MaxYPos(i16);

#[derive(Debug, Copy, Clone)]
struct InitialVelocity(Velocity);

#[derive(Debug, Copy, Clone)]
struct TargetArea {
    p1: Position,
    p2: Position,
}

#[derive(Debug)]
struct ProbesReachedTargetArea(Sender<(MaxYPos, InitialVelocity)>);

impl TargetArea {
    fn contains(&self, pos: Position) -> bool {
        (self.p1.x..=self.p2.x).contains(&pos.x) && (self.p1.y..=self.p2.y).contains(&pos.y)
    }
}

#[system(for_each)]
fn movement(position: &mut Position, velocity: &mut Velocity) {
    position.x = position.x.saturating_add(velocity.x);
    position.y = position.y.saturating_add(velocity.y);
    velocity.x = if velocity.x > 0 {
        velocity.x.saturating_sub(1)
    } else if velocity.x < 0 {
        velocity.x.saturating_add(1)
    } else {
        velocity.x
    };
    velocity.y = velocity.y.saturating_sub(1);
}

#[system(for_each)]
fn max_y_pos(pos: &Position, max_y_pos: &mut MaxYPos) {
    max_y_pos.0 = i16::max(max_y_pos.0, pos.y);
}

#[system(for_each)]
fn probe_reached_target_area(
    entity: &Entity,
    pos: &Position,
    initial_velocity: &InitialVelocity,
    max_y_pos: &MaxYPos,
    #[resource] target_area: &TargetArea,
    #[resource] probes_reached_target_area: &ProbesReachedTargetArea,
    buffer: &mut CommandBuffer,
) {
    if target_area.contains(*pos) {
        probes_reached_target_area
            .0
            .send((*max_y_pos, *initial_velocity))
            .unwrap();
        buffer.remove(*entity);
    }
}

#[system(for_each)]
fn delete_entities(entity: &Entity, pos: &Position, buffer: &mut CommandBuffer) {
    if pos.y == i16::MIN {
        buffer.remove(*entity);
    }
}

fn load_target_area<R: BufRead>(reader: R) -> TargetArea {
    let line = reader.lines().next().unwrap().unwrap();
    let mut parts = line.split(' ');
    parts.next();
    parts.next();
    let x_text = parts.next().unwrap();
    let y_text = parts.next().unwrap();

    fn parse_range(text: &str) -> (i16, i16) {
        let (_, range) = text.split_once('=').unwrap();
        let (a, b) = range.split_once("..").unwrap();
        (
            a.parse().unwrap(),
            b.split(',').next().unwrap().parse().unwrap(),
        )
    }

    let (x1, x2) = parse_range(x_text);
    let (y1, y2) = parse_range(y_text);

    TargetArea {
        p1: Position { x: x1, y: y1 },
        p2: Position { x: x2, y: y2 },
    }
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> i16 {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(load_target_area(reader));

    let (sender, receiver) = unbounded();
    resources.insert(ProbesReachedTargetArea(sender));

    let mut schedule = Schedule::builder()
        .add_system(movement_system())
        .flush()
        .add_system(max_y_pos_system())
        .flush()
        .add_system(probe_reached_target_area_system())
        .add_system(delete_entities_system())
        .build();

    for x in 0..=200 {
        for y in -100..=500 {
            world.push((
                Position { x: 0, y: 0 },
                Velocity { x, y },
                InitialVelocity(Velocity { x, y }),
                MaxYPos(0),
            ));
        }
    }

    let mut query = <&Velocity>::query();

    while query.iter(&world).next().is_some() {
        schedule.execute(&mut world, &mut resources);
    }

    drop(resources);

    receiver
        .into_iter()
        .map(|(max_y_pos, _)| max_y_pos.0)
        .max()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> usize {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(load_target_area(reader));

    let (sender, receiver) = unbounded();
    resources.insert(ProbesReachedTargetArea(sender));

    let mut schedule = Schedule::builder()
        .add_system(movement_system())
        .flush()
        .add_system(max_y_pos_system())
        .flush()
        .add_system(probe_reached_target_area_system())
        .add_system(delete_entities_system())
        .build();

    for x in 0..=500 {
        for y in -200..=1000 {
            world.push((
                Position { x: 0, y: 0 },
                Velocity { x, y },
                InitialVelocity(Velocity { x, y }),
                MaxYPos(0),
            ));
        }
    }

    let mut query = <&Velocity>::query();

    while query.iter(&world).next().is_some() {
        schedule.execute(&mut world, &mut resources);
    }

    drop(resources);

    receiver.into_iter().count()
}
