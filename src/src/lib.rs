#![no_std]
#![feature(drop_types_in_const, naked_functions, prelude_import, static_in_const)]

#[macro_use]
extern crate libtww;
#[prelude_import]
#[allow(unused)]
use libtww::prelude::*;

mod warp;

use libtww::rand::{XorShiftRng, SeedableRng, Rng, sample};
use warp::{Origin, Destination, Warp, WARPS, HUB, HUB_WARPS};

use libtww::Addr;
use libtww::system::tww::{report, dStage_dt_c_stageLoader, dSv_player_get_item_c_onItem,
                          dSv_player_return_place_c_set};
use libtww::std::collections::HashMap;
use libtww::system::memory;
use libtww::warping::{Entrance, FadeOut, Warp as GameWarp};
use libtww::link::{item, Link};
use libtww::link::inventory::Inventory;
use libtww::link::quest_items::{QuestItems, Sword};
use libtww::game::{flag, controller};

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

    let hub_destinations = sample(&mut rng,
                                  WARPS.iter().map(|w| &w.destination),
                                  HUB_WARPS.len());
    let hub_warps = HUB_WARPS.iter().zip(hub_destinations);

    let warps = normal_warps.chain(hub_warps);

    (warps.collect(), *name)
}

fn is_hub() -> bool {
    memory::read_str(&Entrance::last_entrance().stage as *const u8) == "K_Test2"
}

fn on_item_hook(item: u8, slot_id: i32) {
    if is_hub() {
        Inventory::get().bottle4_slot = item;
    } else {
        dSv_player_get_item_c_onItem(0x803b8159, slot_id, 0);
        Inventory::set_by_slot_id(slot_id as usize, item);
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_boomerang() {
    on_item_hook(item::BOOMERANG, 5);
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_bombs() {
    on_item_hook(item::BOMBS, 13);
    let inventory = Inventory::get();
    if inventory.bomb_capacity < 30 {
        inventory.bomb_capacity = 30;
        inventory.bomb_count = inventory.bomb_capacity;
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_bow() {
    on_item_hook(item::BOW, 12);
    let inventory = Inventory::get();
    if inventory.arrow_capacity < 30 {
        inventory.arrow_capacity = 30;
        inventory.arrow_count = inventory.arrow_capacity;
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_deku_leaf() {
    on_item_hook(item::DEKU_LEAF, 6);
    let link = Link::get();
    if link.max_magic < 16 {
        link.max_magic = 16;
        link.magic = link.max_magic;
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_hookshot() {
    on_item_hook(item::HOOKSHOT, 19);
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn on_skull_hammer() {
    on_item_hook(item::SKULL_HAMMER, 20);
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

const FLAG_NONE: u16 = 0;
const FLAG_BOMBS: u16 = 1 << 1;
const FLAG_HOOKSHOT: u16 = 1 << 3;
const FLAG_BOW: u16 = 1 << 5;
const FLAG_SKULL_HAMMER: u16 = 1 << 7;
const FLAG_BOOMERANG: u16 = 1 << 9;
const FLAG_DEKU_LEAF: u16 = 1 << 11;

#[no_mangle]
#[inline(never)]
pub extern "C" fn stage_load_hook(a: Addr, b: Addr) {
    if is_hub() {
        let inventory = Inventory::get();
        let flag = match inventory.bottle4_slot {
            item::BOMBS => FLAG_BOMBS,
            item::HOOKSHOT => FLAG_HOOKSHOT,
            item::BOW => FLAG_BOW,
            item::SKULL_HAMMER => FLAG_SKULL_HAMMER,
            item::BOOMERANG => FLAG_BOOMERANG,
            item::DEKU_LEAF => FLAG_DEKU_LEAF,
            _ => FLAG_NONE,
        };
        // let flag = 0;
        *memory::reference(0x803B8882) = flag;
    }

    report("Load stage");
    dStage_dt_c_stageLoader(a, b);
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

fn warp_to_hub() {
    let warp = GameWarp {
        entrance: Entrance {
            stage: *b"K_Test2\0",
            entrance: HUB.spawn as u16,
            room: HUB.room,
        },
        layer_override: -1,
        enabled: true,
        fadeout: FadeOut::Wobble,
    };

    warp.execute();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    let name = *get_name();
    let recalculate = unsafe { WARP_TABLE.as_ref() }
        .map(|&(_, ref name)| name != get_name())
        .unwrap_or(true) && name != [0; 8];

    if controller::is_pressed(controller::DPAD_DOWN) {
        warp_to_hub();
    }

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
#[inline(never)]
pub extern "C" fn on_save(addr: Addr) {
    dSv_player_return_place_c_set(addr, b"K_Test2\0".as_ptr(), 0, 0)
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn start() {
    use libtww::std::mem::uninitialized as empty;

    game_loop();
    set_next_stage_hook();
    stage_load_hook(empty(), empty());

    on_boomerang();
    on_bombs();
    on_bow();
    on_deku_leaf();
    on_hookshot();
    on_skull_hammer();

    on_save(empty());
}
