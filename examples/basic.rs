extern crate pi_ecs_old;
extern crate pi_map;
#[macro_use]
extern crate pi_ecs_derive_old;

use pi_ecs_old::component::Component;
use pi_map::vecmap::VecMap;
// use hashmap::HashMap;

#[derive(Component)]
pub struct Position{
    x: usize,
    y: usize,
}

#[derive(Component)]
pub struct ZIndex(usize);

pub struct ZIndex1(ZIndex);

component!{
    struct ZIndex1(usize);
}

pub struct Position1(Position);

component!{
    struct Position1{
        x:usize,
        y:usize,
    }
}


#[derive(Write)]
pub struct Single{
    x: usize,
    y: usize,
}

#[derive(Write)]
pub struct Single1(String, f32);

pub struct Single2{
    x: usize,
    y: usize,
}

write_trait!{
    pub struct Single2{
        x: usize,
        y: usize,
    }
}

pub struct Single3(String, f32);

write_trait!{
    pub struct Single3(String, f32);
}

fn main() { 

}