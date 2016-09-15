#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Origin<'a> {
    pub stage: &'a str,
    pub room: Option<u8>,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Destination<'a> {
    pub stage: &'a str,
    pub room: u8,
    pub spawn: u8,
    pub fadeout: u8,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Warp<'origin, 'destination> {
    pub origin: Origin<'origin>,
    pub destination: Destination<'destination>,
}

use libtww::std::fmt;

impl<'a> fmt::Display for Origin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}, ", self.stage));
        if let Some(room) = self.room {
            write!(f, "{}", room)
        } else {
            write!(f, "<Stage>")
        }
    }
}

impl<'a> fmt::Display for Destination<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}, {}, {}, {}",
               self.stage,
               self.room,
               self.spawn,
               self.fadeout)
    }
}

impl<'origin, 'destination> fmt::Display for Warp<'origin, 'destination> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.origin, self.destination)
    }
}

macro_rules! parse_room {
    (Stage) => { None };
    ($x:expr) => { Some($x) };
}

macro_rules! warps {
    ($($from_stage:ident,
       $from_room:tt ->
       $to_stage:ident,
       $to_room:expr,
       $start_code:expr,
       $fadeout:expr)*) => {
        &[$(
            (
                Warp {
                    origin: Origin {
                        stage: stringify!($from_stage),
                        room: parse_room!($from_room),
                    },
                    destination: Destination {
                        stage: stringify!($to_stage),
                        room: $to_room,
                        spawn: $start_code,
                        fadeout: $fadeout,
                    },
                }
            )
        ),*]
    }
}

pub static HUB: &Destination = &Destination {
    stage: "K_Test2",
    room: 0,
    spawn: 0,
    fadeout: 0,
};

pub static HUB_WARPS: &[Warp] = warps! {
    K_Test2, 0 -> sea, 0, 0, 0
    K_Test2, 0 -> sea, 1, 0, 0
    K_Test2, 0 -> sea, 2, 0, 0
    K_Test2, 0 -> sea, 3, 0, 0
    K_Test2, 0 -> sea, 4, 0, 0
    K_Test2, 0 -> sea, 5, 0, 0
};

pub static WARPS: &[Warp] = warps! {
    Abesso, 0 -> sea, 33, 1, 0
    Abesso, 0 -> TF_04, 0, 0, 0
//  Abship, Stage -> sea, 1, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 1, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 1, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 1, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 2, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 2, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 2, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 2, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 3, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 3, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 3, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 3, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 4, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 4, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 4, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 4, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 5, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 5, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 5, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 5, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 6, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 6, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 6, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 6, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 7, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 7, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 7, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 7, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 8, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 8, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 8, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 8, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 9, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 9, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 9, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 9, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 10, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 10, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 10, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 10, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 11, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 11, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 11, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 11, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 12, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 12, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 12, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 12, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 13, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 13, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 13, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 13, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 14, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 14, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 14, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 14, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 15, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 15, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 15, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 15, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 16, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 16, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 16, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 16, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 17, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 17, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 17, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 17, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 18, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 18, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 18, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 18, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 19, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 19, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 19, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 19, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 20, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 20, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 20, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 20, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 21, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 21, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 21, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 21, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 22, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 22, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 22, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 22, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 23, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 23, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 23, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 23, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 24, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 24, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 24, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 24, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 25, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 25, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 25, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 25, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 26, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 26, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 26, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 26, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 27, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 27, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 27, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 27, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 28, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 28, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 28, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 28, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 29, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 29, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 29, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 29, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 30, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 30, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 30, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 30, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 31, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 31, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 31, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 31, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 32, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 32, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 32, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 32, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 33, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 33, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 33, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 33, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 34, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 34, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 34, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 34, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 35, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 35, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 35, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 35, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 36, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 36, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 36, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 36, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 37, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 37, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 37, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 37, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 38, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 38, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 38, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 38, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 39, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 39, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 39, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 39, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 40, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 40, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 40, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 40, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 41, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 41, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 41, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 41, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 42, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 42, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 42, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 42, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 43, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 43, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 43, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 43, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 44, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 44, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 44, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 44, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 45, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 45, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 45, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 45, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 46, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 46, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 46, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 46, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 47, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 47, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 47, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 47, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 48, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 48, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 48, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 48, 103, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 49, 100, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 49, 101, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 49, 102, 0 Annoying Ocean Warp
//  Abship, Stage -> sea, 49, 103, 0 Annoying Ocean Warp
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 9, 99, 1
    Abship, Stage -> sea, 11, 99, 1
    Abship, Stage -> sea, 13, 99, 1
    Abship, Stage -> sea, 23, 99, 1
    Abship, Stage -> sea, 26, 99, 1
    Abship, Stage -> sea, 41, 99, 1
    Abship, Stage -> sea, 44, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Abship, Stage -> sea, 1, 99, 1
    Adanmae, 0 -> Atorizk, 0, 2, 8
    Adanmae, 0 -> Atorizk, 0, 3, 8
    Adanmae, 0 -> M_NewD2, 0, 0, 0
    ADMumi, Stage -> ADMumi, 0, 100, 0
//  Amos_T, 0 -> I_TestM, 0, 2, 5 Test Map
//  Amos_T, 0 -> I_TestM, 0, 1, 5 Test Map
    Asoko, Stage -> A_umikz, 255, 0, 0
    Atorizk, 0 -> sea, 13, 1, 9
    Atorizk, 0 -> sea, 13, 2, 9
    Atorizk, 0 -> Adanmae, 0, 0, 9
    Atorizk, 0 -> Adanmae, 0, 1, 9
    Atorizk, 0 -> Comori, 0, 0, 0
    A_mori, 0 -> sea, 44, 8, 9
    A_mori, 0 -> Fairy04, 0, 0, 8
//  A_umikz, Stage -> I_TestM, 255, 255, 0 Test Map
//  A_umikz, Stage -> I_TestM, 255, 255, 0 Test Map
    Cave01, 0 -> sea, 34, 1, 9
    Cave02, 0 -> sea, 2, 1, 9
    Cave03, 0 -> sea, 42, 2, 9
    Cave03, 0 -> sea, 42, 1, 9
    Cave04, 0 -> sea, 16, 1, 9
    Cave05, 0 -> sea, 43, 5, 9
    Cave06, 0 -> sea, 44, 10, 9
    Cave07, 0 -> sea, 12, 5, 9
//  Cave08, 1 -> kaze, 255, 15, 0 Test Map
//  Cave08, 2 -> kaze, 255, 22, 4 Test Map
//  Cave08, 2 -> kaze, 255, 23, 4 Test Map
//  Cave08, 2 -> kaze, 255, 24, 4 Test Map
//  Cave08, 2 -> kazeMB, 6, 0, 0 Test Map
//  Cave08, 3 -> kaze, 255, 18, 5 Test Map
//  Cave08, Stage -> kaze, 255, 15, 0 Test Map
//  Cave08, Stage -> kaze, 255, 16, 0 Test Map
    Cave09, 0 -> Cave09, 1, 0, 0
    Cave09, 0 -> sea, 44, 10, 9
    Cave09, 1 -> Cave09, 2, 0, 0
    Cave09, 2 -> Cave09, 3, 0, 0
    Cave09, 3 -> Cave09, 4, 0, 0
    Cave09, 4 -> Cave09, 5, 0, 0
    Cave09, 5 -> Cave09, 11, 0, 0
    Cave09, 5 -> Cave09, 0, 1, 0
    Cave09, 6 -> Cave09, 7, 0, 0
    Cave09, 7 -> Cave09, 8, 0, 0
    Cave09, 8 -> Cave09, 9, 0, 0
    Cave09, 9 -> Cave09, 10, 0, 0
    Cave09, 10 -> Cave09, 16, 0, 0
    Cave09, 10 -> Cave09, 0, 1, 0
    Cave09, 11 -> Cave09, 12, 0, 0
    Cave09, 12 -> Cave09, 13, 0, 0
    Cave09, 13 -> Cave09, 14, 0, 0
    Cave09, 14 -> Cave09, 15, 0, 0
    Cave09, 15 -> Cave10, 1, 0, 0
    Cave09, 15 -> Cave09, 0, 1, 0
    Cave09, 16 -> Cave09, 17, 0, 0
    Cave09, 17 -> Cave09, 18, 0, 0
    Cave09, 18 -> Cave09, 19, 0, 0
    Cave09, 19 -> Cave09, 20, 0, 0
    Cave09, 20 -> Cave11, 1, 0, 0
    Cave09, 20 -> Cave09, 0, 1, 0
    Cave10, 0 -> Cave10, 1, 0, 0
    Cave10, 0 -> sea, 44, 10, 9
    Cave10, 1 -> Cave10, 2, 0, 0
    Cave10, 2 -> Cave10, 3, 0, 0
    Cave10, 3 -> Cave10, 4, 0, 0
    Cave10, 4 -> Cave10, 5, 0, 0
    Cave10, 5 -> Cave10, 11, 0, 0
    Cave10, 5 -> Cave09, 0, 1, 0
    Cave10, 6 -> Cave10, 7, 0, 0
    Cave10, 7 -> Cave10, 8, 0, 0
    Cave10, 8 -> Cave10, 9, 0, 0
    Cave10, 9 -> Cave10, 10, 0, 0
    Cave10, 10 -> Cave10, 16, 0, 0
    Cave10, 11 -> Cave10, 12, 0, 0
    Cave10, 12 -> Cave10, 13, 0, 0
    Cave10, 13 -> Cave10, 14, 0, 0
    Cave10, 14 -> Cave10, 15, 0, 0
    Cave10, 15 -> Cave09, 6, 0, 0
    Cave10, 15 -> Cave09, 0, 1, 0
    Cave10, 16 -> Cave10, 17, 0, 0
    Cave10, 17 -> Cave10, 18, 0, 0
    Cave10, 18 -> Cave10, 19, 0, 0
    Cave10, 19 -> Cave10, 20, 0, 0
    Cave10, 20 -> sea, 44, 10, 9
    Cave11, 0 -> Cave11, 1, 0, 0
    Cave11, 0 -> sea, 44, 10, 9
    Cave11, 1 -> Cave11, 2, 0, 0
    Cave11, 2 -> Cave11, 3, 0, 0
    Cave11, 3 -> Cave11, 4, 0, 0
    Cave11, 4 -> Cave11, 5, 0, 0
    Cave11, 5 -> Cave11, 11, 0, 0
    Cave11, 5 -> Cave09, 0, 1, 0
    Cave11, 6 -> Cave11, 7, 0, 0
    Cave11, 7 -> Cave11, 8, 0, 0
    Cave11, 8 -> Cave11, 9, 0, 0
    Cave11, 9 -> Cave11, 10, 0, 0
    Cave11, 10 -> Cave11, 11, 0, 0
    Cave11, 10 -> Cave09, 0, 1, 0
    Cave11, 11 -> Cave11, 12, 0, 0
    Cave11, 12 -> Cave11, 13, 0, 0
    Cave11, 13 -> Cave11, 14, 0, 0
    Cave11, 14 -> Cave11, 15, 0, 0
    Cave11, 15 -> Cave10, 6, 0, 0
    Cave11, 15 -> Cave09, 0, 1, 0
    Cave11, 16 -> Cave09, 17, 0, 0
    Cave11, 17 -> Cave09, 18, 0, 0
    Cave11, 18 -> Cave09, 19, 0, 0
    Cave11, 19 -> Cave09, 20, 0, 0
    Cave11, 20 -> Cave09, 21, 0, 0
    Cave11, 20 -> Cave09, 0, 1, 0
    Comori, 0 -> Atorizk, 0, 4, 0
//  DmSpot0, Stage -> A_mori, 255, 0, 255 Test Map
//  DmSpot0, Stage -> LinkRM, 255, 0, 255 Test Map
//  DmSpot0, Stage -> Ojhous, 255, 0, 255 Test Map
//  DmSpot0, Stage -> Onobuta, 255, 1, 255 Test Map
    Edaichi, 0 -> M_Dai, 255, 0, 0
    Edaichi, 0 -> sea, 45, 1, 9
    Edaichi, Stage -> Edaichi, 0, 0, 0
    Ekaze, 0 -> kaze, 15, 15, 0
    Ekaze, 0 -> sea, 4, 1, 9
//  ENDumi, Stage -> ADMumi, 0, 100, 0 Test Map
    Fairy01, 0 -> sea, 3, 1, 9
    Fairy02, 0 -> sea, 19, 1, 9
    Fairy03, 0 -> sea, 15, 1, 9
    Fairy04, 0 -> A_mori, 0, 2, 9
    Fairy05, 0 -> sea, 28, 1, 9
    Fairy06, 0 -> sea, 39, 1, 9
    figureA, 0 -> Pfigure, 0, 1, 0
    figureB, 0 -> Pfigure, 0, 2, 0
    figureC, 0 -> Pfigure, 0, 3, 0
    figureD, 0 -> Pfigure, 0, 4, 0
    figureE, 0 -> Pfigure, 0, 5, 0
    figureF, 0 -> Pfigure, 0, 6, 0
    figureG, 0 -> Pfigure, 0, 7, 0
    GanonA, 0 -> Hyrule, 0, 4, 9
    GanonA, 1 -> GanonB, 0, 0, 0
    GanonA, 1 -> GanonC, 0, 0, 0
    GanonA, 1 -> GanonD, 0, 0, 0
    GanonA, 1 -> GanonE, 0, 0, 0
    GanonA, 1 -> GanonN, 0, 0, 0
    GanonA, Stage -> GanonA, 0, 0, 0
    GanonB, 0 -> GanonA, 1, 1, 0
    GanonB, 0 -> Xboss0, 0, 0, 2
    GanonB, Stage -> GanonA, 0, 0, 0
    GanonC, 0 -> GanonA, 1, 2, 0
    GanonC, 0 -> Xboss3, 0, 0, 2
    GanonC, 1 -> GanonC, 0, 0, 0
    GanonC, 1 -> GanonD, 0, 0, 0
    GanonC, 2 -> GanonC, 0, 0, 0
    GanonC, 2 -> GanonD, 0, 0, 0
    GanonC, 3 -> GanonC, 0, 0, 0
    GanonC, 3 -> GanonD, 0, 0, 0
    GanonC, 4 -> GanonC, 0, 0, 0
    GanonC, 4 -> GanonD, 0, 0, 0
    GanonC, 5 -> GanonC, 0, 0, 0
    GanonC, 5 -> GanonD, 0, 0, 0
    GanonC, Stage -> GanonA, 0, 0, 0
    GanonD, 0 -> GanonA, 1, 3, 0
    GanonD, 0 -> Xboss1, 0, 0, 2
    GanonD, Stage -> GanonA, 0, 0, 0
    GanonE, 0 -> GanonA, 1, 4, 0
    GanonE, 0 -> Xboss2, 0, 0, 2
    GanonE, Stage -> GanonA, 0, 0, 0
    GanonJ, 1 -> GanonM, 0, 2, 0
    GanonJ, 2 -> GanonM, 0, 2, 0
    GanonJ, 4 -> GanonM, 0, 2, 0
    GanonJ, 7 -> GanonM, 0, 2, 0
    GanonJ, 8 -> GanonM, 0, 2, 0
    GanonJ, 11 -> GanonM, 0, 2, 0
    GanonJ, 13 -> GanonM, 0, 3, 0
    GanonJ, Stage -> GanonM, 0, 4, 0
    GanonK, 0 -> GanonL, 0, 1, 0
    GanonK, 0 -> GTower, 0, 0, 9
    GanonK, 0 -> GanonK, 0, 2, 4
    GanonK, 0 -> GanonK, 0, 3, 4
    GanonK, Stage -> GanonM, 0, 4, 0
    GanonL, 0 -> GanonM, 0, 1, 0
    GanonL, 0 -> GanonK, 0, 0, 0
    GanonL, Stage -> GanonM, 0, 4, 0
    GanonM, 0 -> GanonN, 0, 1, 0
    GanonM, 0 -> GanonL, 0, 0, 0
    GanonM, 1 -> GanonJ, 1, 0, 0
    GanonM, 2 -> sea, 1, 16, 1
    GanonM, Stage -> GanonM, 0, 4, 0
    GanonN, 0 -> GanonA, 1, 6, 0
    GanonN, 0 -> GanonM, 0, 0, 0
    GanonN, Stage -> GanonA, 0, 0, 0
    GTower, Stage -> GanonM, 0, 4, 0
    Hyroom, 0 -> Hyrule, 0, 2, 1
    Hyroom, 0 -> Hyrule, 0, 3, 1
    Hyroom, 0 -> kenroom, 0, 0, 0
    Hyroom, Stage -> Hyroom, 0, 10, 0
    Hyrule, 0 -> Hyroom, 0, 0, 0
    Hyrule, 0 -> Hyroom, 0, 1, 0
    Hyrule, 0 -> GanonA, 0, 0, 0
    Hyrule, Stage -> Hyrule, 0, 0, 0
//  ITest61, 0 -> sea, 34, 1, 9 Test Map
//  ITest62, 0 -> sea, 40, 1, 9 Test Map
//  ITest63, 0 -> sea, 38, 5, 9 Test Map
//  I_SubAN, 0 -> sea, 20, 1, 0 Test Map
//  I_SubAN, 1 -> sea, 40, 1, 0 Test Map
//  I_SubAN, 2 -> sea, 5, 1, 0 Test Map
//  I_SubAN, 3 -> sea, 36, 1, 0 Test Map
//  I_SubAN, 4 -> sea, 30, 1, 255 Test Map
//  I_SubAN, 5 -> sea, 34, 1, 0 Test Map
//  I_SubAN, 6 -> sea, 2, 1, 0 Test Map
//  I_SubAN, 7 -> sea, 42, 1, 0 Test Map
//  I_SubAN, 8 -> sea, 16, 1, 0 Test Map
//  I_SubAN, 9 -> sea, 47, 1, 0 Test Map
//  I_SubAN, 10 -> sea, 31, 1, 0 Test Map
//  I_SubAN, 11 -> sea, 7, 1, 0 Test Map
//  I_SubAN, 12 -> sea, 35, 1, 0 Test Map
//  I_SubAN, 12 -> sea, 28, 1, 0 Test Map
//  I_SubAN, 13 -> sea, 19, 1, 0 Test Map
//  I_SubAN, 14 -> sea, 15, 1, 0 Test Map
//  I_SubAN, 15 -> sea, 39, 1, 0 Test Map
//  I_SubAN, 16 -> sea, 35, 1, 0 Test Map
//  I_TestM, 0 -> I_TestM, 0, 2, 5 Test Map
//  I_TestM, 0 -> I_TestM, 0, 1, 5 Test Map
    Kaisen, 0 -> sea, 11, 9, 11
    Kaisen, 0 -> sea, 11, 8, 11
//  Kaisen, Stage -> LinkRM, 255, 89, 0 Weird Warp to Link's House
//  Kaisen, Stage -> LinkRM, 255, 90, 0 Weird Warp to Link's House
//  KATA_RM, 18 -> KATA_RM, 18, 1, 0 Test Map
    kaze, 0 -> kaze, 255, 22, 4
    kaze, 0 -> kaze, 255, 23, 4
    kaze, 0 -> kaze, 255, 24, 4
    kaze, 1 -> kaze, 255, 15, 0
    kaze, 2 -> kaze, 255, 22, 4
    kaze, 2 -> kaze, 255, 23, 4
    kaze, 2 -> kaze, 255, 24, 4
    kaze, 2 -> kazeMB, 6, 0, 0
    kaze, 3 -> kaze, 255, 18, 5
    kaze, 5 -> kaze, 255, 18, 5
    kaze, 6 -> kazeMB, 6, 0, 0
    kaze, 9 -> kaze, 255, 18, 0
    kaze, 11 -> kaze, 255, 18, 5
    kaze, 12 -> kazeB, 0, 0, 0
    kaze, 12 -> kaze, 255, 22, 4
    kaze, 12 -> kaze, 255, 23, 4
    kaze, 12 -> kaze, 255, 24, 4
    kaze, 13 -> kazeB, 0, 0, 0
    kaze, 15 -> Ekaze, 0, 1, 0
    kaze, 16 -> kaze, 255, 18, 5
    kaze, Stage -> kaze, 255, 15, 0
    kaze, Stage -> kaze, 255, 16, 0
    kazeB, Stage -> kaze, 255, 15, 0
    kazeB, Stage -> kazeB, 0, 1, 0
    kazeMB, 6 -> kaze, 255, 20, 0
    kazeMB, Stage -> kaze, 255, 15, 0
    kenroom, 0 -> Hyroom, 0, 2, 0
    kenroom, Stage -> kenroom, 0, 10, 0
    kinBOSS, 0 -> Omori, 0, 6, 0
    kinBOSS, Stage -> kindan, 0, 1, 0
    kinBOSS, Stage -> kinBOSS, 0, 1, 0
    kindan, 0 -> sea, 41, 6, 9
    kindan, 0 -> kindan, 0, 2, 0
    kindan, 0 -> kindan, 5, 1, 4
    kindan, 0 -> kindan, 16, 2, 4
    kindan, 5 -> kindan, 0, 2, 4
    kindan, 5 -> kindan, 5, 1, 0
    kindan, 5 -> kindan, 16, 2, 4
    kindan, 9 -> kinMB, 10, 0, 9
    kindan, 16 -> kinBOSS, 0, 0, 0
    kindan, 16 -> kindan, 0, 2, 4
    kindan, 16 -> kindan, 5, 1, 4
    kindan, 16 -> kindan, 16, 2, 0
    kindan, Stage -> kindan, 0, 1, 0
    kinMB, 10 -> kindan, 9, 1, 8
    kinMB, Stage -> kindan, 0, 1, 0
//  K_Test2, 0 -> K_Test2, 0, 1, 0 Test Map
//  K_Test2, 0 -> K_Test2, 0, 2, 0 Test Map
//  K_Test3, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test3, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test4, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test4, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test5, 0 -> K_Test2, 0, 0, 0 Test Map
//  K_Test5, 0 -> K_Test2, 0, 0, 0 Test Map
//  K_Test6, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test6, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test8, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test8, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test9, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Test9, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Testa, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Testa, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Testb, 0 -> I_TestM, 0, 0, 0 Test Map
//  K_Testb, 0 -> I_TestM, 0, 0, 0 Test Map
    LinkRM, 0 -> sea, 44, 1, 11
    LinkUG, 0 -> sea, 44, 11, 9
    M2ganon, 0 -> M2tower, 0, 22, 0
    M2ganon, Stage -> M2tower, 0, 20, 0
    M2tower, 0 -> ma2room, 0, 17, 0
    M2tower, 0 -> M2ganon, 0, 0, 0
    M2tower, Stage -> M2tower, 0, 20, 0
    ma2room, 0 -> sea, 1, 3, 5
    ma2room, 0 -> sea, 1, 4, 5
    ma2room, 1 -> sea, 1, 2, 5
    ma2room, 1 -> sea, 1, 14, 5
    ma2room, 2 -> sea, 1, 12, 5
    ma2room, 2 -> sea, 1, 13, 5
    ma2room, 2 -> sea, 1, 15, 0
    ma2room, 3 -> sea, 1, 8, 5
    ma2room, 3 -> sea, 1, 9, 0
    ma2room, 3 -> sea, 1, 10, 0
    ma2room, 3 -> sea, 1, 11, 5
    ma2room, 3 -> ma2room, 0, 0, 0
    ma2room, 4 -> sea, 1, 5, 0
    ma2room, 4 -> sea, 1, 6, 0
    ma2room, 4 -> sea, 1, 7, 5
    ma2room, Stage -> sea, 1, 0, 0
    ma3room, 0 -> sea, 1, 3, 11
    ma3room, 0 -> sea, 1, 4, 11
    ma3room, 1 -> sea, 1, 2, 11
    ma3room, 1 -> sea, 1, 14, 11
    ma3room, 2 -> sea, 1, 12, 11
    ma3room, 2 -> sea, 1, 13, 11
    ma3room, 2 -> sea, 1, 15, 9
    ma3room, 3 -> sea, 1, 8, 11
    ma3room, 3 -> sea, 1, 9, 9
    ma3room, 3 -> sea, 1, 10, 9
    ma3room, 3 -> sea, 1, 11, 11
    ma3room, 4 -> sea, 1, 5, 9
    ma3room, 4 -> sea, 1, 6, 9
    ma3room, 4 -> sea, 1, 7, 11
    ma3room, Stage -> sea, 1, 0, 0
    majroom, 0 -> MajyuE, 0, 3, 5
    majroom, 0 -> MajyuE, 0, 4, 5
    majroom, 1 -> MajyuE, 0, 2, 5
    majroom, 1 -> MajyuE, 0, 14, 5
    majroom, 2 -> MajyuE, 0, 12, 5
    majroom, 2 -> MajyuE, 0, 13, 5
    majroom, 2 -> MajyuE, 0, 15, 0
    majroom, 3 -> MajyuE, 0, 8, 5
    majroom, 3 -> MajyuE, 0, 9, 0
    majroom, 3 -> MajyuE, 0, 10, 0
    majroom, 3 -> MajyuE, 0, 11, 5
    majroom, 4 -> MajyuE, 0, 5, 0
    majroom, 4 -> MajyuE, 0, 6, 0
    majroom, 4 -> MajyuE, 0, 7, 5
    majroom, Stage -> MajyuE, 0, 0, 0
    MajyuE, 0 -> majroom, 0, 0, 1
    MajyuE, 0 -> majroom, 0, 1, 1
    MajyuE, 0 -> majroom, 1, 2, 7
    MajyuE, 0 -> majroom, 0, 3, 7
    MajyuE, 0 -> majroom, 0, 4, 7
    MajyuE, 0 -> majroom, 4, 5, 7
    MajyuE, 0 -> majroom, 4, 6, 1
    MajyuE, 0 -> majroom, 4, 7, 7
    MajyuE, 0 -> majroom, 3, 8, 7
    MajyuE, 0 -> majroom, 3, 9, 1
    MajyuE, 0 -> majroom, 3, 10, 1
    MajyuE, 0 -> majroom, 3, 11, 7
    MajyuE, 0 -> majroom, 2, 12, 7
    MajyuE, 0 -> majroom, 2, 13, 7
    MajyuE, 0 -> majroom, 1, 14, 7
    MajyuE, 0 -> majroom, 2, 15, 1
    MajyuE, 0 -> Mjtower, 0, 16, 1
    MajyuE, 0 -> Mjtower, 0, 17, 1
    MajyuE, 0 -> majroom, 0, 18, 1
    MajyuE, 0 -> majroom, 2, 19, 1
    MajyuE, Stage -> MajyuE, 0, 0, 0
    MiniHyo, 0 -> sea, 40, 1, 9
//  MiniHyo, 0 -> ITest62, 0, 0, 0 Test Map
    MiniKaz, 0 -> sea, 20, 1, 9
    Mjtower, 0 -> MajyuE, 0, 17, 0
    Mjtower, Stage -> Mjtower, 0, 17, 0
//  morocam, Stage -> I_TestM, 0, 0, 255 Test Map
    M_Dai, 0 -> Edaichi, 0, 1, 0
    M_Dai, 2 -> M_Dai, 255, 22, 4
    M_Dai, 2 -> M_Dai, 255, 23, 4
    M_Dai, 2 -> M_Dai, 255, 24, 4
    M_Dai, 4 -> M_Dai, 255, 4, 5
    M_Dai, 6 -> M_Dai, 255, 4, 5
    M_Dai, 6 -> M_Dai, 255, 22, 4
    M_Dai, 6 -> M_Dai, 255, 23, 4
    M_Dai, 6 -> M_Dai, 255, 24, 4
    M_Dai, 7 -> M_DaiMB, 12, 0, 0
    M_Dai, 8 -> M_Dai, 255, 12, 0
    M_Dai, 9 -> M_Dai, 255, 4, 0
    M_Dai, 9 -> M_Dai, 255, 7, 0
    M_Dai, 9 -> M_Dai, 255, 9, 0
    M_Dai, 11 -> M_Dai, 255, 4, 5
    M_Dai, 13 -> M_Dai, 255, 4, 5
    M_Dai, 15 -> M_DaiB, 255, 0, 0
    M_Dai, 15 -> M_Dai, 255, 22, 4
    M_Dai, 15 -> M_Dai, 255, 23, 4
    M_Dai, 15 -> M_Dai, 255, 24, 4
    M_Dai, 15 -> M_Dai, 255, 4, 0
    M_Dai, Stage -> M_Dai, 255, 0, 0
    M_Dai, Stage -> M_Dai, 255, 21, 0
    M_Dai, Stage -> M_Dai, 255, 12, 0
    M_Dai, Stage -> M_Dai, 255, 13, 0
    M_DaiB, 0 -> M_Dai, 255, 17, 0
    M_DaiB, Stage -> M_Dai, 255, 0, 0
    M_DaiB, Stage -> M_DaiB, 255, 1, 0
    M_DaiMB, 12 -> M_Dai, 255, 9, 0
    M_DaiMB, Stage -> M_Dai, 255, 0, 0
    M_Dra09, 9 -> M_NewD2, 10, 0, 0
    M_Dra09, 9 -> M_NewD2, 7, 0, 0
    M_Dra09, Stage -> M_NewD2, 0, 0, 0
    M_DragB, 0 -> Adanmae, 0, 100, 0
    M_DragB, Stage -> M_NewD2, 0, 0, 0
    M_DragB, Stage -> M_DragB, 0, 1, 0
    M_NewD2, 0 -> Adanmae, 0, 2, 9
    M_NewD2, 0 -> M_NewD2, 0, 5, 4
    M_NewD2, 0 -> M_NewD2, 2, 5, 4
    M_NewD2, 0 -> M_NewD2, 10, 5, 4
    M_NewD2, 2 -> M_NewD2, 0, 5, 4
    M_NewD2, 2 -> M_NewD2, 2, 5, 4
    M_NewD2, 2 -> M_NewD2, 10, 5, 4
    M_NewD2, 5 -> Adanmae, 0, 2, 1
    M_NewD2, 6 -> M_Dra09, 9, 0, 0
    M_NewD2, 7 -> M_Dra09, 9, 1, 9
    M_NewD2, 10 -> M_Dra09, 9, 0, 9
    M_NewD2, 10 -> M_DragB, 0, 0, 0
    M_NewD2, 10 -> M_NewD2, 0, 5, 4
    M_NewD2, 10 -> M_NewD2, 2, 5, 4
    M_NewD2, 10 -> M_NewD2, 10, 5, 4
    M_NewD2, 15 -> M_Dra09, 9, 1, 0
    M_NewD2, Stage -> M_NewD2, 0, 0, 0
    M_NewD2, Stage -> M_NewD2, 10, 17, 0
    M_NewD2, Stage -> M_NewD2, 255, 18, 0
    Nitiyou, 0 -> sea, 11, 12, 11
    Obombh, 0 -> Obombh, 0, 0, 0
    Obombh, 0 -> sea, 11, 1, 11
    Obombh, 0 -> sea, 11, 2, 4
//  Obombh, Stage -> LinkRM, 255, 80, 0 Weird Warp to Link's House
//  Obombh, Stage -> LinkRM, 255, 81, 0 Weird Warp to Link's House
//  Obshop, Stage -> sea, 1, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 1, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 1, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 1, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 2, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 2, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 2, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 2, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 3, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 3, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 3, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 3, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 4, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 4, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 4, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 4, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 5, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 5, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 5, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 5, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 6, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 6, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 6, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 6, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 7, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 7, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 7, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 7, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 8, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 8, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 8, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 8, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 9, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 9, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 9, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 9, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 10, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 10, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 10, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 10, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 11, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 11, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 11, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 11, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 12, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 12, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 12, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 12, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 13, 0, 0 Test Map
//  Obshop, Stage -> sea, 13, 0, 0 Test Map
//  Obshop, Stage -> sea, 13, 0, 0 Test Map
//  Obshop, Stage -> sea, 13, 0, 0 Test Map
//  Obshop, Stage -> sea, 14, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 14, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 14, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 14, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 15, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 15, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 15, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 15, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 16, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 16, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 16, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 16, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 17, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 17, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 17, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 17, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 18, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 18, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 18, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 18, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 19, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 19, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 19, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 19, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 20, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 20, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 20, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 20, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 21, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 21, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 21, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 21, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 22, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 22, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 22, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 22, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 23, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 23, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 23, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 23, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 24, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 24, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 24, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 24, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 25, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 25, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 25, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 25, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 26, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 26, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 26, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 26, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 27, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 27, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 27, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 27, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 28, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 28, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 28, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 28, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 29, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 29, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 29, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 29, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 30, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 30, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 30, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 30, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 31, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 31, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 31, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 31, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 32, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 32, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 32, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 32, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 33, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 33, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 33, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 33, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 34, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 34, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 34, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 34, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 35, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 35, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 35, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 35, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 36, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 36, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 36, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 36, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 37, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 37, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 37, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 37, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 38, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 38, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 38, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 38, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 39, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 39, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 39, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 39, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 40, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 40, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 40, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 40, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 41, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 41, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 41, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 41, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 42, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 42, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 42, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 42, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 43, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 43, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 43, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 43, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 44, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 44, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 44, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 44, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 45, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 45, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 45, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 45, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 46, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 46, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 46, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 46, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 47, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 47, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 47, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 47, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 48, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 48, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 48, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 48, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 49, 100, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 49, 101, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 49, 102, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 49, 103, 0 Annoying Ocean Warp and Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 9, 99, 1 Test Map
//  Obshop, Stage -> sea, 11, 99, 1 Test Map
//  Obshop, Stage -> sea, 13, 99, 1 Test Map
//  Obshop, Stage -> sea, 23, 99, 1 Test Map
//  Obshop, Stage -> sea, 26, 99, 1 Test Map
//  Obshop, Stage -> sea, 41, 99, 1 Test Map
//  Obshop, Stage -> sea, 44, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Obshop, Stage -> sea, 1, 99, 1 Test Map
//  Ocean, Stage -> LinkRM, 255, 2, 0 Weird Warp to Link's House
    Ocmera, 0 -> sea, 11, 10, 11
    Ocmera, 0 -> sea, 11, 11, 11
    Ocrogh, 0 -> Omori, 0, 0, 0
    Ojhous, 0 -> sea, 44, 3, 11
    Ojhous, 1 -> sea, 44, 2, 5
    Ojhous, 1 -> sea, 44, 10, 1
//  Ojhous2, 0 -> sea, 44, 3, 5 Test Map
//  Ojhous2, 1 -> sea, 44, 2, 11 Test Map
//  Ojhous2, 1 -> sea, 44, 10, 1 Test Map
    Omasao, 0 -> sea, 44, 7, 11
    Omori, 0 -> Ocrogh, 0, 0, 0
    Omori, 0 -> sea, 41, 1, 9
    Omori, 0 -> sea, 41, 2, 9
    Omori, 0 -> sea, 41, 3, 9
    Omori, 0 -> sea, 41, 4, 9
    Omori, 0 -> sea, 41, 5, 9
    Onobuta, 0 -> sea, 44, 4, 11
    Onobuta, 0 -> sea, 44, 5, 0
    Opub, 0 -> sea, 11, 6, 11
    Orichh, 0 -> Orichh, 0, 0, 0
    Orichh, 0 -> sea, 11, 3, 11
    Orichh, 0 -> sea, 11, 4, 11
    Otkura, 0 -> sea, 41, 9, 9
    Pdrgsh, 0 -> sea, 11, 7, 11
    Pfigure, 0 -> sea, 41, 10, 9
    Pfigure, 0 -> figureA, 0, 0, 5
    Pfigure, 0 -> figureB, 0, 0, 5
    Pfigure, 0 -> figureC, 0, 0, 5
    Pfigure, 0 -> figureD, 0, 0, 5
    Pfigure, 0 -> figureE, 0, 0, 5
    Pfigure, 0 -> figureF, 0, 0, 5
    Pfigure, 0 -> figureG, 0, 0, 5
    Pjavdou, 0 -> sea, 44, 9, 1
    Pnezumi, 0 -> sea, 11, 13, 11
    PShip, 0 -> sea, 23, 0, 1
    PShip, 1 -> sea, 23, 0, 1
    PShip, 2 -> sea, 23, 0, 1
//  PShip, Stage -> sea, 1, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 1, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 1, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 1, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 2, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 2, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 2, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 2, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 3, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 3, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 3, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 3, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 4, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 4, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 4, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 4, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 5, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 5, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 5, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 5, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 6, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 6, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 6, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 6, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 7, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 7, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 7, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 7, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 8, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 8, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 8, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 8, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 9, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 9, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 9, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 9, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 10, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 10, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 10, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 10, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 11, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 11, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 11, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 11, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 12, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 12, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 12, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 12, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 13, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 13, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 13, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 13, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 14, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 14, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 14, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 14, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 15, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 15, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 15, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 15, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 16, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 16, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 16, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 16, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 17, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 17, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 17, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 17, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 18, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 18, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 18, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 18, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 19, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 19, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 19, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 19, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 20, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 20, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 20, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 20, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 21, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 21, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 21, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 21, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 22, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 22, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 22, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 22, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 23, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 23, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 23, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 23, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 24, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 24, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 24, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 24, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 25, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 25, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 25, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 25, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 26, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 26, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 26, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 26, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 27, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 27, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 27, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 27, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 28, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 28, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 28, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 28, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 29, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 29, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 29, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 29, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 30, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 30, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 30, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 30, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 31, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 31, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 31, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 31, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 32, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 32, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 32, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 32, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 33, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 33, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 33, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 33, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 34, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 34, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 34, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 34, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 35, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 35, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 35, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 35, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 36, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 36, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 36, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 36, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 37, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 37, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 37, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 37, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 38, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 38, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 38, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 38, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 39, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 39, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 39, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 39, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 40, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 40, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 40, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 40, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 41, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 41, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 41, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 41, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 42, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 42, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 42, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 42, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 43, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 43, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 43, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 43, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 44, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 44, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 44, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 44, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 45, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 45, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 45, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 45, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 46, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 46, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 46, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 46, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 47, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 47, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 47, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 47, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 48, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 48, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 48, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 48, 103, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 49, 100, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 49, 101, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 49, 102, 0 Annoying Ocean Warp
//  PShip, Stage -> sea, 49, 103, 0 Annoying Ocean Warp
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 9, 99, 1
    PShip, Stage -> sea, 11, 99, 1
    PShip, Stage -> sea, 13, 99, 1
    PShip, Stage -> sea, 23, 99, 1
    PShip, Stage -> sea, 26, 99, 1
    PShip, Stage -> sea, 41, 99, 1
    PShip, Stage -> sea, 44, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
    PShip, Stage -> sea, 1, 99, 1
//  PShip2, 0 -> sea, 23, 0, 0 Test Map
//  PShip3, 0 -> sea, 23, 0, 0 Test Map
    sea, 1 -> ma2room, 0, 0, 8
    sea, 1 -> ma2room, 0, 1, 8
    sea, 1 -> ma2room, 1, 2, 10
    sea, 1 -> ma2room, 0, 3, 10
    sea, 1 -> ma2room, 0, 4, 10
    sea, 1 -> ma2room, 4, 5, 10
    sea, 1 -> ma2room, 4, 6, 8
    sea, 1 -> ma2room, 4, 7, 10
    sea, 1 -> ma2room, 3, 8, 10
    sea, 1 -> ma2room, 3, 9, 8
    sea, 1 -> ma2room, 3, 10, 8
    sea, 1 -> ma2room, 3, 11, 10
    sea, 1 -> ma2room, 2, 12, 10
    sea, 1 -> ma2room, 2, 13, 10
    sea, 1 -> ma2room, 1, 14, 10
    sea, 1 -> ma2room, 2, 15, 8
    sea, 1 -> M2tower, 0, 16, 1
    sea, 1 -> GanonM, 2, 1, 1
    sea, 2 -> Cave02, 0, 0, 0
    sea, 3 -> Fairy01, 0, 0, 0
    sea, 4 -> Ekaze, 0, 0, 0
    sea, 7 -> TF_02, 0, 0, 0
    sea, 11 -> sea, 11, 0, 0
    sea, 11 -> Obombh, 0, 0, 10
    sea, 11 -> Obombh, 0, 1, 4
    sea, 11 -> Orichh, 0, 0, 10
    sea, 11 -> Orichh, 0, 1, 10
    sea, 11 -> Pdrgsh, 0, 0, 10
    sea, 11 -> Pnezumi, 0, 0, 10
    sea, 11 -> Opub, 0, 0, 0
    sea, 11 -> Kaisen, 0, 1, 10
    sea, 11 -> Kaisen, 0, 0, 10
    sea, 11 -> Ocmera, 0, 0, 10
    sea, 11 -> Ocmera, 0, 1, 10
    sea, 11 -> Nitiyou, 0, 0, 10
    sea, 12 -> TyuTyu, 0, 0, 0
    sea, 12 -> Cave07, 0, 0, 0
    sea, 13 -> Atorizk, 0, 0, 8
    sea, 13 -> Atorizk, 0, 1, 8
    sea, 13 -> TF_06, 0, 0, 0
    sea, 14 -> sea, 14, 1, 0
    sea, 15 -> Fairy03, 0, 0, 0
    sea, 16 -> Cave04, 0, 0, 0
    sea, 17 -> sea, 17, 0, 0
    sea, 19 -> Fairy02, 0, 0, 0
    sea, 20 -> MiniKaz, 0, 0, 0
    sea, 26 -> Siren, 0, 0, 0
    sea, 28 -> Fairy05, 0, 0, 0
    sea, 29 -> SubD42, 0, 0, 0
    sea, 30 -> ShipD, 0, 0, 0
    sea, 31 -> TF_01, 0, 0, 0
    sea, 33 -> Abesso, 0, 0, 0
    sea, 34 -> Cave01, 0, 0, 0
    sea, 35 -> TF_03, 0, 0, 0
    sea, 36 -> WarpD, 0, 0, 0
//  sea, 38 -> ITest63, 0, 0, 0 Test Map
    sea, 39 -> Fairy06, 0, 0, 0
    sea, 40 -> MiniHyo, 0, 0, 0
//  sea, 41 -> LinkRM, 41, 0, 8 Weird Warp to Link's House
    sea, 41 -> Omori, 0, 1, 8
    sea, 41 -> Omori, 0, 2, 8
    sea, 41 -> Omori, 0, 3, 8
    sea, 41 -> Omori, 0, 4, 8
    sea, 41 -> Omori, 0, 5, 8
    sea, 41 -> kindan, 0, 0, 8
    sea, 41 -> Otkura, 0, 0, 8
    sea, 41 -> Pfigure, 0, 0, 8
    sea, 42 -> Cave03, 0, 0, 0
    sea, 42 -> Cave03, 0, 1, 0
    sea, 43 -> Cave05, 0, 0, 0
    sea, 44 -> LinkRM, 0, 1, 10
    sea, 44 -> Ojhous, 0, 0, 10
//  sea, 44 -> Ojhous2, 1, 0, 10 Test Map
    sea, 44 -> Omasao, 0, 0, 10
    sea, 44 -> Onobuta, 0, 0, 10
    sea, 44 -> Onobuta, 0, 1, 8
    sea, 44 -> A_mori, 0, 0, 0
    sea, 44 -> Pjavdou, 0, 0, 0
    sea, 44 -> Cave09, 0, 0, 0
    sea, 44 -> LinkUG, 0, 1, 0
    sea, 45 -> Edaichi, 0, 0, 0
//  sea, 47 -> I_SubAN, 9, 0, 0 Test Map
    sea, 47 -> SubD43, 0, 0, 0
    sea, 48 -> SubD71, 0, 0, 0
//  sea, Stage -> sea, 1, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 1, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 1, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 1, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 2, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 2, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 2, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 2, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 3, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 3, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 3, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 3, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 4, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 4, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 4, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 4, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 5, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 5, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 5, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 5, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 6, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 6, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 6, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 6, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 7, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 7, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 7, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 7, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 8, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 8, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 8, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 8, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 9, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 9, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 9, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 9, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 10, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 10, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 10, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 10, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 11, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 11, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 11, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 11, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 12, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 12, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 12, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 12, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 13, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 13, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 13, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 13, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 14, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 14, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 14, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 14, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 15, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 15, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 15, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 15, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 16, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 16, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 16, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 16, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 17, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 17, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 17, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 17, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 18, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 18, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 18, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 18, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 19, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 19, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 19, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 19, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 20, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 20, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 20, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 20, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 21, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 21, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 21, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 21, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 22, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 22, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 22, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 22, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 23, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 23, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 23, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 23, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 24, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 24, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 24, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 24, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 25, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 25, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 25, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 25, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 26, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 26, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 26, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 26, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 27, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 27, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 27, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 27, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 28, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 28, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 28, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 28, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 29, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 29, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 29, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 29, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 30, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 30, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 30, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 30, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 31, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 31, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 31, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 31, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 32, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 32, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 32, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 32, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 33, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 33, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 33, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 33, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 34, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 34, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 34, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 34, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 35, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 35, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 35, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 35, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 36, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 36, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 36, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 36, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 37, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 37, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 37, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 37, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 38, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 38, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 38, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 38, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 39, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 39, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 39, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 39, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 40, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 40, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 40, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 40, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 41, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 41, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 41, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 41, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 42, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 42, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 42, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 42, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 43, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 43, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 43, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 43, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 44, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 44, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 44, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 44, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 45, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 45, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 45, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 45, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 46, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 46, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 46, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 46, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 47, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 47, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 47, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 47, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 48, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 48, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 48, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 48, 103, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 49, 100, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 49, 101, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 49, 102, 0 Annoying Ocean Warp
//  sea, Stage -> sea, 49, 103, 0 Annoying Ocean Warp
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 9, 99, 1
    sea, Stage -> sea, 11, 99, 1
    sea, Stage -> sea, 13, 99, 1
    sea, Stage -> sea, 23, 99, 1
    sea, Stage -> sea, 26, 99, 1
    sea, Stage -> sea, 41, 99, 1
    sea, Stage -> sea, 44, 99, 1
    sea, Stage -> sea, 17, 99, 1
    sea, Stage -> sea, 39, 99, 1
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 1, 99, 1
    sea, Stage -> sea, 1, 99, 1
//  sea_E, 44 -> LinkRM, 0, 1, 5 Test Map
//  sea_E, 44 -> Ojhous, 0, 0, 5 Test Map
//  sea_E, 44 -> Ojhous2, 1, 0, 5 Test Map
//  sea_E, 44 -> Omasao, 0, 0, 5 Test Map
//  sea_E, 44 -> Onobuta, 0, 0, 5 Test Map
//  sea_E, 44 -> Onobuta, 0, 1, 0 Test Map
//  sea_E, 44 -> A_mori, 0, 0, 0 Test Map
//  sea_E, 44 -> Pjavdou, 0, 1, 0 Test Map
//  sea_E, 44 -> Cave06, 0, 0, 0 Test Map
//  sea_E, 44 -> LinkUG, 0, 1, 0 Test Map
//  sea_E, Stage -> sea, 1, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 1, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 1, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 1, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 2, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 2, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 2, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 2, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 3, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 3, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 3, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 3, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 4, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 4, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 4, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 4, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 5, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 5, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 5, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 5, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 6, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 6, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 6, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 6, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 7, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 7, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 7, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 7, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 8, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 8, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 8, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 8, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 9, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 9, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 9, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 9, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 10, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 10, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 10, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 10, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 11, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 11, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 11, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 11, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 12, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 12, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 12, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 12, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 13, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 13, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 13, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 13, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 14, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 14, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 14, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 14, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 15, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 15, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 15, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 15, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 16, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 16, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 16, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 16, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 17, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 17, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 17, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 17, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 18, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 18, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 18, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 18, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 19, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 19, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 19, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 19, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 20, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 20, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 20, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 20, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 21, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 21, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 21, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 21, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 22, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 22, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 22, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 22, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 23, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 23, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 23, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 23, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 24, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 24, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 24, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 24, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 25, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 25, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 25, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 25, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 26, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 26, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 26, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 26, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 27, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 27, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 27, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 27, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 28, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 28, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 28, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 28, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 29, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 29, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 29, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 29, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 30, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 30, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 30, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 30, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 31, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 31, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 31, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 31, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 32, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 32, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 32, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 32, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 33, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 33, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 33, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 33, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 34, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 34, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 34, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 34, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 35, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 35, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 35, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 35, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 36, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 36, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 36, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 36, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 37, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 37, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 37, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 37, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 38, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 38, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 38, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 38, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 39, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 39, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 39, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 39, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 40, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 40, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 40, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 40, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 41, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 41, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 41, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 41, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 42, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 42, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 42, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 42, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 43, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 43, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 43, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 43, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 44, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 44, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 44, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 44, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 45, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 45, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 45, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 45, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 46, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 46, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 46, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 46, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 47, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 47, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 47, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 47, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 48, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 48, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 48, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 48, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 49, 100, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 49, 101, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 49, 102, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 49, 103, 0 Annoying Ocean Warp and Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 9, 99, 1 Test Map
//  sea_E, Stage -> sea, 11, 99, 1 Test Map
//  sea_E, Stage -> sea, 13, 99, 1 Test Map
//  sea_E, Stage -> sea, 23, 99, 1 Test Map
//  sea_E, Stage -> sea, 26, 99, 1 Test Map
//  sea_E, Stage -> sea, 41, 99, 1 Test Map
//  sea_E, Stage -> sea, 44, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
//  sea_E, Stage -> sea, 1, 99, 1 Test Map
    ShipD, 0 -> sea, 30, 1, 9
    Siren, 0 -> Siren, 0, 0, 0
    Siren, 0 -> sea, 26, 2, 9
    Siren, 7 -> Siren, 7, 0, 0
    Siren, 7 -> Siren, 17, 0, 1
    Siren, 14 -> SirenMB, 23, 0, 0
    Siren, 17 -> Siren, 17, 0, 0
    Siren, 17 -> Siren, 7, 0, 1
    Siren, 18 -> SirenB, 0, 0, 0
    Siren, 19 -> SirenB, 0, 0, 0
    Siren, Stage -> Siren, 0, 0, 0
    Siren, Stage -> Siren, 17, 0, 0
    Siren, Stage -> Siren, 7, 0, 0
    Siren, Stage -> SirenB, 0, 0, 0
    SirenB, 0 -> sea, 26, 0, 0
    SirenB, Stage -> Siren, 0, 0, 0
    SirenB, Stage -> SirenB, 0, 1, 0
    SirenMB, 23 -> Siren, 14, 1, 0
    SirenMB, Stage -> Siren, 0, 0, 0
    SubD42, 0 -> sea, 29, 5, 9
    SubD42, 0 -> WarpD, 0, 1, 4
    SubD42, 0 -> WarpD, 0, 2, 4
    SubD42, 0 -> WarpD, 0, 3, 4
    SubD42, 0 -> WarpD, 0, 4, 4
    SubD42, 0 -> WarpD, 0, 5, 4
    SubD42, 0 -> WarpD, 0, 6, 4
    SubD42, 0 -> WarpD, 0, 7, 4
    SubD42, 0 -> WarpD, 0, 8, 4
    SubD42, 0 -> WarpD, 0, 9, 4
    SubD42, 0 -> WarpD, 0, 10, 4
    SubD42, 0 -> WarpD, 0, 11, 4
    SubD42, 0 -> WarpD, 0, 12, 4
    SubD42, 0 -> WarpD, 0, 13, 4
    SubD42, 0 -> WarpD, 0, 14, 4
    SubD42, 0 -> WarpD, 0, 15, 4
    SubD42, 0 -> WarpD, 0, 16, 4
    SubD42, 0 -> WarpD, 0, 17, 4
    SubD42, 0 -> WarpD, 0, 18, 4
    SubD42, 0 -> WarpD, 0, 19, 6
    SubD42, 0 -> WarpD, 0, 20, 6
    SubD42, 0 -> WarpD, 0, 21, 6
    SubD43, 0 -> sea, 47, 5, 9
    SubD43, 0 -> SubD43, 0, 1, 0
    SubD43, 0 -> SubD43, 0, 2, 0
    SubD43, 0 -> SubD43, 0, 3, 0
//  SubD44, 0 -> sea, 31, 1, 1 Test Map
//  SubD45, 0 -> sea, 23, 0, 1 Test Map
//  SubD45, 1 -> sea, 23, 0, 1 Test Map
//  SubD45, 2 -> sea, 23, 0, 1 Test Map
//  SubD45, Stage -> sea, 1, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 1, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 1, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 1, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 2, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 2, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 2, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 2, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 3, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 3, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 3, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 3, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 4, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 4, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 4, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 4, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 5, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 5, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 5, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 5, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 6, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 6, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 6, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 6, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 7, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 7, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 7, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 7, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 8, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 8, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 8, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 8, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 9, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 9, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 9, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 9, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 10, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 10, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 10, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 10, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 11, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 11, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 11, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 11, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 12, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 12, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 12, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 12, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 13, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 13, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 13, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 13, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 14, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 14, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 14, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 14, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 15, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 15, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 15, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 15, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 16, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 16, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 16, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 16, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 17, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 17, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 17, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 17, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 18, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 18, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 18, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 18, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 19, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 19, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 19, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 19, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 20, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 20, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 20, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 20, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 21, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 21, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 21, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 21, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 22, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 22, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 22, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 22, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 23, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 23, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 23, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 23, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 24, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 24, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 24, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 24, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 25, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 25, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 25, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 25, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 26, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 26, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 26, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 26, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 27, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 27, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 27, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 27, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 28, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 28, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 28, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 28, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 29, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 29, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 29, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 29, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 30, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 30, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 30, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 30, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 31, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 31, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 31, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 31, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 32, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 32, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 32, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 32, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 33, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 33, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 33, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 33, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 34, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 34, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 34, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 34, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 35, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 35, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 35, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 35, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 36, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 36, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 36, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 36, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 37, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 37, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 37, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 37, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 38, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 38, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 38, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 38, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 39, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 39, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 39, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 39, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 40, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 40, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 40, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 40, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 41, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 41, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 41, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 41, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 42, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 42, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 42, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 42, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 43, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 43, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 43, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 43, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 44, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 44, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 44, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 44, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 45, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 45, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 45, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 45, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 46, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 46, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 46, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 46, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 47, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 47, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 47, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 47, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 48, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 48, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 48, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 48, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 49, 100, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 49, 101, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 49, 102, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 49, 103, 0 Annoying Ocean Warp and Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 9, 99, 1 Test Map
//  SubD45, Stage -> sea, 11, 99, 1 Test Map
//  SubD45, Stage -> sea, 13, 99, 1 Test Map
//  SubD45, Stage -> sea, 23, 99, 1 Test Map
//  SubD45, Stage -> sea, 26, 99, 1 Test Map
//  SubD45, Stage -> sea, 41, 99, 1 Test Map
//  SubD45, Stage -> sea, 44, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD45, Stage -> sea, 1, 99, 1 Test Map
//  SubD51, 0 -> sea, 34, 1, 0 Test Map
    SubD71, 0 -> sea, 48, 5, 9
    SubD71, 1 -> sea, 23, 0, 1
    SubD71, 2 -> sea, 23, 0, 1
    TF_01, 0 -> sea, 31, 1, 9
    TF_02, 0 -> sea, 7, 1, 9
    TF_03, 0 -> sea, 35, 1, 9
    TF_04, 0 -> Abesso, 0, 1, 0
//  TF_05, 0 -> sea, 31, 1, 0 Test Map
    TF_06, 0 -> sea, 13, 5, 9
    TF_06, 0 -> TF_06, 1, 1, 0
    TF_06, 1 -> TF_06, 2, 2, 0
    TF_06, 1 -> TF_06, 3, 3, 0
    TF_06, 1 -> TF_06, 4, 4, 0
    TF_06, 1 -> TF_06, 5, 5, 0
    TF_06, 1 -> TF_06, 0, 1, 0
    TF_06, 1 -> TF_06, 6, 6, 0
    TF_06, 2 -> TF_06, 1, 2, 0
    TF_06, 2 -> TF_06, 3, 3, 0
    TF_06, 2 -> TF_06, 4, 4, 0
    TF_06, 2 -> TF_06, 5, 5, 0
    TF_06, 2 -> TF_06, 0, 1, 0
    TF_06, 2 -> TF_06, 6, 6, 0
    TF_06, 3 -> TF_06, 2, 2, 0
    TF_06, 3 -> TF_06, 1, 3, 0
    TF_06, 3 -> TF_06, 4, 4, 0
    TF_06, 3 -> TF_06, 5, 5, 0
    TF_06, 3 -> TF_06, 0, 1, 0
    TF_06, 3 -> TF_06, 6, 6, 0
    TF_06, 4 -> TF_06, 2, 2, 0
    TF_06, 4 -> TF_06, 3, 3, 0
    TF_06, 4 -> TF_06, 1, 4, 0
    TF_06, 4 -> TF_06, 5, 5, 0
    TF_06, 4 -> TF_06, 0, 1, 0
    TF_06, 4 -> TF_06, 6, 6, 0
    TF_06, 5 -> TF_06, 2, 2, 0
    TF_06, 5 -> TF_06, 3, 3, 0
    TF_06, 5 -> TF_06, 4, 4, 0
    TF_06, 5 -> TF_06, 1, 5, 0
    TF_06, 5 -> TF_06, 0, 1, 0
    TF_06, 5 -> TF_06, 6, 6, 0
    TF_06, 6 -> TF_06, 1, 6, 0
//  TF_07, 1 -> sea, 44, 0, 9 Test Map
//  tincle, 0 -> tincle, 0, 0, 0 Test Map
//  tincle, 0 -> sea, 17, 0, 0 Test Map
    TyuTyu, 0 -> sea, 12, 1, 9
    WarpD, 0 -> sea, 36, 1, 9
    WarpD, 0 -> WarpD, 0, 1, 4
    WarpD, 0 -> WarpD, 0, 2, 4
    WarpD, 0 -> WarpD, 0, 3, 4
    WarpD, 0 -> WarpD, 0, 4, 4
    WarpD, 0 -> WarpD, 0, 5, 4
    WarpD, 0 -> WarpD, 0, 6, 4
    WarpD, 0 -> WarpD, 0, 7, 4
    WarpD, 0 -> WarpD, 0, 8, 4
    WarpD, 0 -> WarpD, 0, 9, 4
    WarpD, 0 -> WarpD, 0, 10, 4
    WarpD, 0 -> WarpD, 0, 11, 4
    WarpD, 0 -> WarpD, 0, 12, 4
    WarpD, 0 -> WarpD, 0, 13, 4
    WarpD, 0 -> WarpD, 0, 14, 4
    WarpD, 0 -> WarpD, 0, 15, 4
    WarpD, 0 -> WarpD, 0, 16, 4
    WarpD, 0 -> WarpD, 0, 17, 4
    WarpD, 0 -> WarpD, 0, 18, 4
    WarpD, 0 -> WarpD, 0, 19, 6
    WarpD, 0 -> WarpD, 0, 20, 6
    WarpD, 0 -> WarpD, 0, 21, 6
    Xboss0, 0 -> GanonA, 1, 5, 2
    Xboss0, Stage -> GanonA, 0, 0, 0
    Xboss1, 0 -> GanonA, 1, 5, 2
    Xboss1, Stage -> GanonA, 0, 0, 0
    Xboss2, 0 -> GanonA, 1, 5, 2
    Xboss2, Stage -> GanonA, 0, 0, 0
    Xboss3, 0 -> GanonA, 1, 5, 2
    Xboss3, Stage -> GanonA, 0, 0, 0

    // New Warps

    // M_NewD2, 8 -> M_Dra09, 9, 0, 9
    // M_NewD2, 2, M_Dra09, 9, 0, 9
    // GanonA, 1 -> Hyrule, 0, 4, 9

    // Needs checking

    // sea -> DRI Inner seems to fail Link::room() not working properly?
    // Warp not found: Omori, <Stage> -> sea, 41, 5, 1
    // sea, 1, 99, 1 Softlock (maybe having KoRL fixes this)
};
