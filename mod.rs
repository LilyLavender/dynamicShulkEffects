use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    std::collections::HashMap
};

const MONADO_TYPES: [&str; 5] = ["jump", "speed", "shield", "buster", "smash"];

#[skyline::hook(replace = sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe fn after_image4_on_arg29_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
    
	if fighter_kind == FIGHTER_KIND_SHULK {
        let tex_map: HashMap<u64, &str> = {
            let mut map = HashMap::new();
            map.insert(0x10884a8b70, "tex_shulk_sword1");
            map.insert(0x106644ea5c, "tex_shulk_sword3");
            map.insert(0x108f274f69, "tex_shulk_sword5");
            map.insert(0x14f8cdba70, "tex_shulk_swordpink1");
            map.insert(0x13723c0845, "tex_shulk_swordred1");
            map
        };
        
        let mut l2c_agent = L2CAgent::new(lua_state);
		let funct_args: Vec<L2CValue> = (0..29).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        if let Some(&tex_str) = tex_map.get(&(funct_args[0].get_int() as u64)) {
			let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) as usize;
            let tex_str = if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
                // Add monado type to end of tex_str
                format!("{}_{}", tex_str, MONADO_TYPES[monado_type])
            } else {
                //format!("{}_white", tex_str) // white version
                tex_str.to_string() // vanilla blue version
            };
                        
            // Push args back into stack
			l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(hash40(&tex_str))); 
            for i in 1..29 {
                l2c_agent.push_lua_stack(&mut funct_args[i].clone());
            }
		}
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::EFFECT_FOLLOW)]
unsafe fn effect_follow_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
    
	if fighter_kind == FIGHTER_KIND_SHULK {
        let eff_map: HashMap<u64, &str> = {
            let mut map = HashMap::new();
            map.insert(0x0ef4e7d064, "shulk_airslash");
            map.insert(0x15fb15c7a1, "shulk_backslash_trace");
            map.insert(0x128b451786, "shulk_monad_circle");
            map.insert(0x116d1f3036, "shulk_monad_sword");
            map.insert(0x12d50234a4, "shulk_monad_sword2");
            map.insert(0x166873ad8c, "shulk_monad_sword2_end");
            map.insert(0x161769cdae, "shulk_monad_sword2_arc");
            map.insert(0x18617c3ae7, "shulk_monad_sword2_arc_2");
            map.insert(0x1ca4651767, "shulk_monad_sword2_lightning");
            map.insert(0x12a2050432, "shulk_monad_sword3");
            map.insert(0x145e303089, "shulk_monad_sword3_2");
            map.insert(0x142937001f, "shulk_monad_sword3_3");
            map.insert(0x14b75395bc, "shulk_monad_sword3_4");
            map.insert(0x165513843c, "shulk_monad_sword3_end");
            map.insert(0x16af8ee802, "shulk_monad_sword3_red");
            map.insert(0x1aa3066864, "shulk_monad_sword3_red_end");
            map.insert(0x17bf3f61fc, "shulk_monad_sword3_pink");
            map.insert(0x1bcb32a662, "shulk_monad_sword3_pink_end");
            map.insert(0x1392da6fd8, "shulk_vision_attack");
            map
        };
        
		let mut l2c_agent = L2CAgent::new(lua_state);
		let funct_args: Vec<L2CValue> = (0..10).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        if let Some(&eff_str) = eff_map.get(&(funct_args[0].get_int() as u64)) {
			let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) as usize;
            let eff_str = if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
                // Add monado type to end of eff_str
                format!("{}_{}", eff_str, MONADO_TYPES[monado_type])
            } else {
                //format!("{}_white", eff_str) // white version
                eff_str.to_string() // vanilla blue version
            };
            
            // Push args back into stack
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(hash40(&eff_str))); 
            for i in 1..10 {
                l2c_agent.push_lua_stack(&mut funct_args[i].clone());
            }
		}
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::EFFECT_OFF_KIND)]
unsafe fn effect_off_kind_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
    
	if fighter_kind == FIGHTER_KIND_SHULK {
        let eff_map: HashMap<u64, &str> = {
            let mut map = HashMap::new();
            map.insert(0x0ef4e7d064, "shulk_airslash");
            map.insert(0x15fb15c7a1, "shulk_backslash_trace");
            map.insert(0x128b451786, "shulk_monad_circle");
            map.insert(0x116d1f3036, "shulk_monad_sword");
            map.insert(0x12d50234a4, "shulk_monad_sword2");
            map.insert(0x166873ad8c, "shulk_monad_sword2_end");
            map.insert(0x161769cdae, "shulk_monad_sword2_arc");
            map.insert(0x18617c3ae7, "shulk_monad_sword2_arc_2");
            map.insert(0x1ca4651767, "shulk_monad_sword2_lightning");
            map.insert(0x12a2050432, "shulk_monad_sword3");
            map.insert(0x145e303089, "shulk_monad_sword3_2");
            map.insert(0x142937001f, "shulk_monad_sword3_3");
            map.insert(0x14b75395bc, "shulk_monad_sword3_4");
            map.insert(0x165513843c, "shulk_monad_sword3_end");
            map.insert(0x16af8ee802, "shulk_monad_sword3_red");
            map.insert(0x1aa3066864, "shulk_monad_sword3_red_end");
            map.insert(0x17bf3f61fc, "shulk_monad_sword3_pink");
            map.insert(0x1bcb32a662, "shulk_monad_sword3_pink_end");
            map.insert(0x1392da6fd8, "shulk_vision_attack");
            map
        };
        
		let mut l2c_agent = L2CAgent::new(lua_state);
		let funct_args: Vec<L2CValue> = (0..3).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
        if let Some(&eff_str) = eff_map.get(&(funct_args[0].get_int() as u64)) {
			let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) as usize;
            let eff_str = if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
                // Add monado type to end of eff_str
                format!("{}_{}", eff_str, MONADO_TYPES[monado_type])
            } else {
                //format!("{}_white", eff_str) // white version
                eff_str.to_string() // vanilla blue version
            };
                        
            // Push args back into stack
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(hash40(&eff_str))); 
            for i in 1..3 {
                l2c_agent.push_lua_stack(&mut funct_args[i].clone());
            }
		}
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        after_image4_on_arg29_replace,
		effect_follow_replace,
        effect_off_kind_replace,
    );
}