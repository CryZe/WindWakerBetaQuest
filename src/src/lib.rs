#![no_std]
#![feature(drop_types_in_const, naked_functions, prelude_import)]

#[macro_use]
extern crate libtww;
#[prelude_import]
#[allow(unused)]
use libtww::prelude::*;

mod warp;

use libtww::rand::{XorShiftRng, SeedableRng, Rng, sample};
use warp::{Origin, Destination, Warp, WARPS, HUB, HUB_WARPS};

use libtww::system::tww::report;
use libtww::std::collections::HashMap;
use libtww::system::memory;
use libtww::warping::{Entrance, Warp as GameWarp};
use libtww::link::{item, Link};
use libtww::link::inventory::Inventory;
use libtww::link::quest_items::{QuestItems, Sword};
use libtww::game::flag;

type WarpTable = HashMap<&'static Warp<'static, 'static>, &'static Destination<'static>>;
type Name = [u8; 8];
type WarpTableWithName = (WarpTable, Name);

static mut WARP_TABLE: Option<(WarpTable, Name)> = None;

fn make_warp_table(name: &Name) -> (WarpTable, Name) {
    let a = (name[0] as u32) << 24 | (name[1] as u32) << 16 | (name[2] as u32) << 8 |
            (name[3] as u32);
    let b = (name[4] as u32) << 24 | (name[5] as u32) << 16 | (name[6] as u32) << 8 |
            (name[7] as u32);
    let seed = [a, b, b, a];
    let mut rng = XorShiftRng::from_seed(seed);

    let mut destinations = WARPS.iter().map(|w| &w.destination).collect::<Vec<_>>();
    rng.shuffle(&mut destinations);

    let normal_warps = WARPS.iter().zip(destinations.into_iter());

    let hub_destinations = sample(&mut rng, WARPS.iter().map(|w| &w.destination), HUB_WARPS.len());
    let hub_warps = HUB_WARPS.iter().zip(hub_destinations);

    let warps = normal_warps.chain(hub_warps);

    (warps.collect(), *name)
}

fn get_name() -> &'static Name {
    memory::reference(0x803B8264)
}

fn look_up_warp(table: &WarpTable, warp: &mut Warp) -> Option<&'static Destination<'static>> {
    if warp.destination.stage == "sea" && warp.destination.spawn == 206 {
        Some(HUB)
    } else if let Some(destination) = table.get(warp) {
        Some(destination)
    } else {
        warp.origin.room = None;
        table.get(warp).cloned()
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn set_next_stage_hook() {
    if let Some(&(ref table, _)) = unsafe { WARP_TABLE.as_ref() } {
        let entrance = Entrance::last_entrance();
        let exit = GameWarp::last_exit();

        let mut warp = Warp {
            origin: Origin {
                stage: memory::read_str(&entrance.stage as *const u8),
                room: Some(Link::room()),
            },
            destination: Destination {
                stage: memory::read_str(&exit.entrance.stage as *const u8),
                room: exit.entrance.room,
                spawn: exit.entrance.entrance as u8, // TODO Check Type
                fadeout: exit.fadeout as u8,
            },
        };

        if let Some(destination) = look_up_warp(table, &mut warp) {
            let mut stage_buffer = [0; 8];

            for (target, source) in stage_buffer.iter_mut()
                .zip(destination.stage.as_bytes()) {
                *target = *source;
            }

            let new_warp = GameWarp {
                entrance: Entrance {
                    stage: stage_buffer,
                    entrance: destination.spawn as u16,
                    room: destination.room,
                },
                layer_override: exit.layer_override, // TODO Fix this
                enabled: true,
                fadeout: unsafe { std::mem::transmute(destination.fadeout) },
            };

            new_warp.execute();

            report(&format!("Replaced Warp by: {}", destination));
        } else {
            report(&format!("Warp not found: {}", warp));
        }
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    let name = *get_name();
    let recalculate = unsafe { WARP_TABLE.as_ref() }
        .map(|&(_, ref name)| name != get_name())
        .unwrap_or(true) && name != [0; 8];

    if recalculate {
        report("Recalculate Warp Table");
        unsafe {
            WARP_TABLE = Some(make_warp_table(get_name()));
        }

        let inventory = Inventory::get();
        inventory.wind_waker_slot = item::WIND_WAKER;

        let quest_items = QuestItems::get();
        if quest_items.sword == Sword::None {
            quest_items.sword = Sword::HerosSword;
            Link::get().sword_id = Sword::HerosSword.item_id();
        }

        flag::SAIL_INTRODUCTION_TEXT_AND_MAP_UNLOCKED.activate();
    }
}

#[no_mangle]
#[naked]
pub extern "C" fn start() {
    game_loop();
    set_next_stage_hook();
}
