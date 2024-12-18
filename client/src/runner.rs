// use std::io::{BufWriter,BufReader};
// use std::fs::File; 

// use crate::session::{new_offline, Session, SessionOffline, SessionOnline};
// use crate::task::Task;
//
// struct Runner {
//     session: Box<dyn Session>,  
//     is_active: bool,
//     memory: Memory
// }
//
// pub fn new(filepath: &str) -> Runner {
//     let memory = memory_load(filepath);
//     Runner{
//         session: new_offline("test".to_string()),
//         is_active: true,
//         memory
//     }
// }
//
// #[derive(Serialize, Deserialize)]
// struct Memory {
//     session_offline: Vec<SessionOffline>,
//     session_online: Vec<SessionOnline>,
//     tasks: Vec<Task>
// }
//
// fn memory_load(filepath: &str) -> Memory {
//     let reader = BufReader::new(
//         File::open(filepath).expect("failed to load json file")
//     );
//
//     let mem: Memory = serde_json::from_reader(reader).expect("failed to parse json");
//     mem
// }
//
// fn memory_save(memory: &Memory, filepath: &str) {
//     let writer = BufWriter::new(
//         File::options()
//         .write(true)
//         .truncate(true)
//         .open(filepath)
//         .expect("failed to find filepath to write to")
//     );
//
//     serde_json::to_writer(writer, memory).expect("failed to write data to json");
// }
