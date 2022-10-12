// Structure of a virtual machine
pub struct VLM {
    cpu: None,
    tag_space: [TagType; 2 ^ 32],
    data_space: [LispObj; 2 ^ 32],
    vm_attribute_table: [VMAttribute; ox8_00_00],

    unmapped_world_words: u32,
    mapped_world_words: u32,
    file_map_entries: u32,
    swap_map_entries: u32,
}
