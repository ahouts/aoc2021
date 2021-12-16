use legion::world::SubWorld;
use legion::{system, Entity, IntoQuery, Query, Resources, Schedule, World};
use std::io::BufRead;

#[derive(Debug, Clone, Default)]
struct Adjacencies {
    n: Option<Entity>,
    nw: Option<Entity>,
    ne: Option<Entity>,
    e: Option<Entity>,
    se: Option<Entity>,
    s: Option<Entity>,
    sw: Option<Entity>,
    w: Option<Entity>,
}

impl Adjacencies {
    fn iter(&self) -> impl Iterator<Item = Entity> {
        [
            self.n, self.nw, self.ne, self.e, self.se, self.s, self.sw, self.w,
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Copy, Clone)]
struct EnergyLevel(u8);

#[derive(Debug, Copy, Clone)]
struct Flashed(bool);

#[derive(Debug, Copy, Clone)]
struct FlashCount(u32);

#[derive(Debug, Default)]
struct EntitiesWithFlashingNeighbor(Vec<Entity>);

#[derive(Debug, Copy, Clone)]
struct AllFlashed(bool);

#[system(for_each)]
fn increment_energy_level(energy_level: &mut EnergyLevel) {
    energy_level.0 += 1;
}

#[system]
fn flash(
    world: &mut SubWorld,
    query: &mut Query<(&EnergyLevel, &Adjacencies, &mut Flashed)>,
    #[resource] flash_count: &mut FlashCount,
    #[resource] entities_with_flashing_neighbor: &mut EntitiesWithFlashingNeighbor,
) {
    for (energy_level, adjacencies, flashed) in query.iter_mut(world) {
        let energy_level: &EnergyLevel = energy_level;
        let adjacencies: &Adjacencies = adjacencies;
        if energy_level.0 > 9 && !flashed.0 {
            flashed.0 = true;
            flash_count.0 += 1;
            for adjacent_entity in adjacencies.iter() {
                entities_with_flashing_neighbor.0.push(adjacent_entity);
            }
        }
    }
}

#[system]
fn accumulate_flashes(
    world: &mut SubWorld,
    query: &mut Query<(&mut EnergyLevel,)>,
    #[resource] entities_with_flashing_neighbor: &mut EntitiesWithFlashingNeighbor,
) {
    for entity in entities_with_flashing_neighbor.0.drain(..) {
        query.get_mut(world, entity).unwrap().0 .0 += 1;
    }
}

#[system]
fn check_all_flashed(
    world: &SubWorld,
    query: &mut Query<&Flashed>,
    #[resource] all_flashed: &mut AllFlashed,
) {
    for flashed in query.iter(world) {
        if !flashed.0 {
            return;
        }
    }
    all_flashed.0 = true;
}

#[system(for_each)]
fn drain(flashed: &mut Flashed, energy_level: &mut EnergyLevel) {
    if flashed.0 {
        flashed.0 = false;
        energy_level.0 = 0;
    }
}

fn load_entities<R: BufRead>(reader: R, world: &mut World) {
    let entities = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| {
            line.bytes()
                .map(|e| e - b'0')
                .map(|e| world.push((EnergyLevel(e), Flashed(false), Adjacencies::default())))
                .collect::<Vec<Entity>>()
        })
        .collect::<Vec<Vec<Entity>>>();

    let mut adjacencies_query = <&mut Adjacencies>::query();
    for i in 0..entities.len() {
        for j in 0..entities[0].len() {
            for (oi, oj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            {
                let a = i as i32 + oi;
                let b = j as i32 + oj;
                if a < 0 || b < 0 {
                    continue;
                }
                let a = a as usize;
                let b = b as usize;
                if let Some(neighbor) = entities.get(a).map(|row| row.get(b)).flatten().copied() {
                    let adj = adjacencies_query.get_mut(world, entities[i][j]).unwrap();
                    match (oi, oj) {
                        (-1, -1) => adj.nw = Some(neighbor),
                        (-1, 0) => adj.n = Some(neighbor),
                        (-1, 1) => adj.ne = Some(neighbor),
                        (0, -1) => adj.w = Some(neighbor),
                        (0, 1) => adj.e = Some(neighbor),
                        (1, -1) => adj.sw = Some(neighbor),
                        (1, 0) => adj.s = Some(neighbor),
                        (1, 1) => adj.se = Some(neighbor),
                        _ => panic!(),
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u32 {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(FlashCount(0));
    resources.insert(EntitiesWithFlashingNeighbor::default());

    load_entities(reader, &mut world);

    let mut increment_schedule = Schedule::builder()
        .add_system(increment_energy_level_system())
        .build();

    let mut flash_schedule = Schedule::builder()
        .add_system(flash_system())
        .flush()
        .add_system(accumulate_flashes_system())
        .build();

    let mut cleanup_schedule = Schedule::builder().add_system(drain_system()).build();

    for _ in 0..100 {
        increment_schedule.execute(&mut world, &mut resources);

        let mut previous_flashes = resources.get_mut::<FlashCount>().unwrap().0;
        loop {
            flash_schedule.execute(&mut world, &mut resources);

            let new_flashes = resources.get_mut::<FlashCount>().unwrap().0;
            if previous_flashes == new_flashes {
                break;
            }
            previous_flashes = new_flashes;
        }

        cleanup_schedule.execute(&mut world, &mut resources);
    }

    let flash_count = resources.get_mut::<FlashCount>().unwrap().0;
    flash_count
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u32 {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(FlashCount(0));
    resources.insert(EntitiesWithFlashingNeighbor::default());
    resources.insert(AllFlashed(false));

    load_entities(reader, &mut world);

    let mut increment_schedule = Schedule::builder()
        .add_system(increment_energy_level_system())
        .build();

    let mut flash_schedule = Schedule::builder()
        .add_system(flash_system())
        .flush()
        .add_system(accumulate_flashes_system())
        .build();

    let mut cleanup_schedule = Schedule::builder()
        .add_system(check_all_flashed_system())
        .flush()
        .add_system(drain_system())
        .build();

    let mut current_step = 0;
    loop {
        current_step += 1;
        increment_schedule.execute(&mut world, &mut resources);

        let mut previous_flashes = resources.get_mut::<FlashCount>().unwrap().0;
        loop {
            flash_schedule.execute(&mut world, &mut resources);

            let new_flashes = resources.get_mut::<FlashCount>().unwrap().0;
            if previous_flashes == new_flashes {
                break;
            }
            previous_flashes = new_flashes;
        }

        cleanup_schedule.execute(&mut world, &mut resources);

        if resources.get::<AllFlashed>().unwrap().0 {
            break;
        }
    }

    current_step
}
