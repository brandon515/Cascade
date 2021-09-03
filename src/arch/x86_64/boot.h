void check_multiboot(void);
void check_cpuid(void);
void check_long_mode(void);
void error(char);
int enable_paging(void);
void enable_long_mode(uint32_t memory_map, uint32_t page_table, uint32_t gdt);
void load_gdt(void);
void jump_64_bit(uint32_t memory_map, uint32_t page_table, uint32_t gdt);
