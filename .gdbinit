target remote :2331
load
mon reset

tbreak cortex_m_rt::reset_handler
