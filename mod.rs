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
				if hitbox_params[0].get_int() == L2CValue::new_int(0x10884a8b70).get_int() { // tex_shulk_sword1
					if monado_type == 0 {
						newSwordHash = 0x15b74bbdad; // tex_shulk_sword1_jump
					} else if monado_type == 1 {
						newSwordHash = 0x1653856c49; // tex_shulk_sword1_speed
					} else if monado_type == 2 {
						newSwordHash = 0x174511df67; // tex_shulk_sword1_shield
					} else if monado_type == 3 {
						newSwordHash = 0x1712bd2dce; // tex_shulk_sword1_buster
					} else if monado_type == 4 {
						newSwordHash = 0x16e3d1b22b; // tex_shulk_sword1_smash
					} else {
						newSwordHash = 0x10884a8b70;
					}
				} else if hitbox_params[0].get_int() == L2CValue::new_int(0x106644ea5c).get_int() { // tex_shulk_sword3
					if monado_type == 0 {
						newSwordHash = 0x15fa831ca6; // tex_shulk_sword3_jump
					} else if monado_type == 1 {
						newSwordHash = 0x16c41a7d60; // tex_shulk_sword3_speed
					} else if monado_type == 2 {
						newSwordHash = 0x170734d81a; // tex_shulk_sword3_shield
					} else if monado_type == 3 {
						newSwordHash = 0x1750982ab3; // tex_shulk_sword3_buster
					} else if monado_type == 4 {
						newSwordHash = 0x16744ea302; // tex_shulk_sword3_smash
					} else {
						newSwordHash = 0x106644ea5c;
					}
				} else if hitbox_params[0].get_int() == L2CValue::new_int(0x108f274f69).get_int() { // tex_shulk_sword5
					if monado_type == 0 {
						newSwordHash = 0x152cdaffbb; // tex_shulk_sword5_jump
					} else if monado_type == 1 {
						newSwordHash = 0x16a7ca485a; // tex_shulk_sword5_speed
					} else if monado_type == 2 {
						newSwordHash = 0x17c15bd19d; // tex_shulk_sword5_shield
					} else if monado_type == 3 {
						newSwordHash = 0x1796f72334; // tex_shulk_sword5_buster
					} else if monado_type == 4 {
						newSwordHash = 0x16179e9638; // tex_shulk_sword5_smash
					} else {
						newSwordHash = 0x108f274f69;
					}
				} else if hitbox_params[0].get_int() == L2CValue::new_int(0x14f8cdba70).get_int() { // tex_shulk_swordpink1
					if monado_type == 0 {
						newSwordHash = 0x193499569a; // tex_shulk_swordpink1_jump
					} else if monado_type == 1 {
						newSwordHash = 0x1aebbb1bad; // tex_shulk_swordpink1_speed
					} else if monado_type == 2 {
						newSwordHash = 0x1be2cec771; // tex_shulk_swordpink1_shield
					} else if monado_type == 3 {
						newSwordHash = 0x1bb56235d8; // tex_shulk_swordpink1_buster
					} else if monado_type == 4 {
						newSwordHash = 0x1a5befc5cf; // tex_shulk_swordpink1_smash
					} else {
						newSwordHash = 0x14f8cdba70;
					}
				} else if hitbox_params[0].get_int() == L2CValue::new_int(0x13723c0845).get_int() { // tex_shulk_swordred1
					if monado_type == 0 {
						newSwordHash = 0x189f498e5b; // tex_shulk_swordred1_jump
					} else if monado_type == 1 {
						newSwordHash = 0x1907733953; // tex_shulk_swordred1_speed
					} else if monado_type == 2 {
						newSwordHash = 0x1ab827d048; // tex_shulk_swordred1_shield
					} else if monado_type == 3 {
						newSwordHash = 0x1aef8b22e1; // tex_shulk_swordred1_buster
					} else if monado_type == 4 {
						newSwordHash = 0x19b727e731; // tex_shulk_swordred1_smash
					} else {
						newSwordHash = 0x13723c0845;
					}
				} else {
					newSwordHash = hitbox_params[0].get_int();
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