//
// CRATE FILES
//

mod common {
    pub mod constants;
    pub mod instruction_format;
    pub mod memory_cell;
    pub mod types;
}

mod world {
    pub mod world;
}
mod hardware {
    pub mod cache_line;
    pub mod cpu;
    pub mod machine;
    pub mod memory;
    pub mod network;
    pub mod page_base;
}

mod life_support {
    pub mod system_comm;
}

mod emulator {
    pub mod config;
    pub mod disassembly;
    pub mod emulator;
    pub mod instructions {
        pub mod common;

        // File number matches documentation order
        pub mod exec_01_list;
        pub mod exec_02_interruptible;
        pub mod exec_03_predicate;
        pub mod exec_04_numeric;
        pub mod exec_05_data_movement;
        pub mod exec_06_field_extraction;
        pub mod exec_07_array;
        pub mod exec_08_branch_loop;
        pub mod exec_09_block;
        pub mod exec_10_function_calling;
        pub mod exec_11_binding;
        pub mod exec_12_catch;
        pub mod exec_13_lexical_variable;
        pub mod exec_14_instance_variable;
        pub mod exec_15_subprimitive;

        pub mod def_01_list;
        pub mod def_02_interruptible;
        pub mod def_03_predicate;
        pub mod def_04_numeric;
        pub mod def_05_data_movement;
        pub mod def_06_field_extraction;
        pub mod def_07_array;
        pub mod def_08_branch_loop;
        pub mod def_09_block;
        pub mod def_10_function_calling;
        pub mod def_11_binding;
        pub mod def_12_catch;
        pub mod def_13_lexical_variable;
        pub mod def_14_instance_variable;
        pub mod def_15_subprimitive;

        pub mod def_build_set;
    }
}
