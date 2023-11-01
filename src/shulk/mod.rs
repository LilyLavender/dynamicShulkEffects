use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::{
    app::{lua_bind::*, sv_animcmd::*, *},
    hash40,
    lib::lua_const::*,
    phx::*,
};

#[skyline::hook(replace = sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn after_image4_on_arg29_replace(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(boma);
    if fighter_kind == FIGHTER_KIND_SHULK
        && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
    {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let hitbox_params: Vec<L2CValue> =
            (0..29).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        l2c_agent.clear_lua_stack();
        let mut newSwordHash: u64 = hitbox_params[0].get_int();
        let monado_type =
            WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
        for current_tex in [
            "tex_shulk_sword1",
            "tex_shulk_sword3",
            "tex_shulk_sword5",
            "tex_shulk_swordpink1",
            "tex_shulk_swordred1",
        ] {
            if hitbox_params[0].get_int() == L2CValue::new_int(hash40(current_tex)).get_int() {
                for (i, suffix) in (0i32..).zip(["jump", "speed", "shield", "buster", "smash"]) {
                    if monado_type == i {
                        newSwordHash = hash40(format!("{}_{}", current_tex, suffix).as_str());
                    }
                }
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(newSwordHash));
        for i in 1..29 {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::EFFECT_FOLLOW)]
unsafe fn effect_follow_replace(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(boma);
    if fighter_kind == FIGHTER_KIND_SHULK
        && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE)
    {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let mut hitbox_params: Vec<L2CValue> =
            (0..10).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        if [
            Hash40::new("shulk_airslash").hash,
            Hash40::new("shulk_backslash_trace").hash,
            Hash40::new("shulk_counter_success").hash,
            Hash40::new("shulk_counter").hash,
            Hash40::new("shulk_monad_circle_red").hash,
            Hash40::new("shulk_monad_circle").hash,
            Hash40::new("shulk_monad_sword").hash,
            Hash40::new("shulk_monad_sword2_arc_2").hash,
            Hash40::new("shulk_monad_sword2_arc").hash,
            Hash40::new("shulk_monad_sword2_end").hash,
            Hash40::new("shulk_monad_sword2_lightning").hash,
            Hash40::new("shulk_monad_sword2").hash,
            Hash40::new("shulk_monad_sword3_2").hash,
            Hash40::new("shulk_monad_sword3_3").hash,
            Hash40::new("shulk_monad_sword3_4").hash,
            Hash40::new("shulk_monad_sword3_end").hash,
            Hash40::new("shulk_monad_sword3_pink_end").hash,
            Hash40::new("shulk_monad_sword3_pink").hash,
            Hash40::new("shulk_monad_sword3_red_end").hash,
            Hash40::new("shulk_monad_sword3_red").hash,
            Hash40::new("shulk_monad_sword3").hash,
            Hash40::new("shulk_vision_attack").hash,
        ]
        .contains(&hitbox_params[0].get_int())
        {
            let monado_type =
                WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
            let (mut r, mut g, mut b) = (1.0, 1.0, 1.0);
            if monado_type == 0 {
                // Jump
                r = 0.1;
                g = 1.0;
                b = 0.1;
            } else if monado_type == 1 {
                // Speed
                r = 0.0;
                g = 0.62;
                b = 1.0;
            } else if monado_type == 2 {
                // Shield
                r = 1.0;
                g = 0.9;
                b = 0.0;
            } else if monado_type == 3 {
                // Buster
                r = 0.4;
                g = 0.0;
                b = 1.0;
            } else if monado_type == 4 {
                // Smash
                r = 1.0;
                g = 0.0;
                b = 0.0;
            }
            l2c_agent.clear_lua_stack();
            for i in 0..10 {
                l2c_agent.push_lua_stack(&mut hitbox_params[i]);
            }
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(r));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(g));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(b));
            EFFECT_FOLLOW_COLOR(lua_state);
        } else {
            original!()(lua_state);
        }
    } else {
        original!()(lua_state);
    }
}

pub fn install() {
    skyline::install_hooks!(after_image4_on_arg29_replace, effect_follow_replace,);
}
