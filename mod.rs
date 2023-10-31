use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
};
use smash::lib::L2CValue;
use smash::lib::L2CAgent;

#[skyline::hook(replace = sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn after_image4_on_arg29_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
	if fighter_kind == FIGHTER_KIND_SHULK && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
		
		let mut l2c_agent = L2CAgent::new(lua_state);
		let hitbox_params: Vec<L2CValue> = (0..29).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		l2c_agent.clear_lua_stack();
		for i in 0..29 {
			if i == 0 {
				let mut newSwordHash: u64 = 0x0;
				let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
				for current_tex in shulk_texs {
					if hitbox_params[0].get_int() == L2CValue::new_int(hash40(current_tex)).get_int() { 
						for (i, suffix) in (0i32..).zip(["jump", "speed", "shield", "buster", "smash"]) {
							if monado_type == i {
								newSwordHash = hash40(format!("{}_{}", current_tex, suffix).as_str());
							}
						}
					}
				}
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(newSwordHash)); 
			} else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
			
		} 
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::EFFECT_FOLLOW)]
unsafe fn effect_follow_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
	if fighter_kind == FIGHTER_KIND_SHULK && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
		let mut l2c_agent = L2CAgent::new(lua_state);
		let mut hitbox_params: Vec<L2CValue> = (0..10).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		if hitbox_params[0].get_int() == L2CValue::new_int(0x128b451786).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x169135fb8f).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x116d1f3036).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x12d50234a4).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x161769cdae).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x18617c3ae7).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x166873ad8c).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x1ca4651767).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x12a2050432).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x145e303089).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x142937001f).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x14b75395bc).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x165513843c).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x17bf3f61fc).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x1bcb32a662).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x16af8ee802).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x1aa3066864).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x0ef4e7d064).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x15fb15c7a1).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x0d6cc4e364).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x15ab1aeaea).get_int() ||
		   hitbox_params[0].get_int() == L2CValue::new_int(0x1392da6fd8).get_int() {
			let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
			let (mut r, mut g, mut b) = (1.0, 1.0, 1.0);
			if monado_type == 0 {
				r = 0.1; g = 1.0; b = 0.1; // Jump
			} else if monado_type == 1 {
				r = 0.0; g = 0.62; b = 1.0; // Speed
			} else if monado_type == 2 {
				r = 1.0; g = 0.9; b = 0.0; // Shield
			} else if monado_type == 3 {
				r = 0.4; g = 0.0; b = 1.0; // Buster
			} else if monado_type == 4 {
				r = 1.0; g = 0.0; b = 0.0; // Smash
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
    skyline::install_hooks!(
        after_image4_on_arg29_replace,
		effect_follow_replace,
    );
}
