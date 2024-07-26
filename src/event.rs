use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "state-events")] {
        
    } else {
    
    }
}
