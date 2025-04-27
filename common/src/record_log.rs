use std::sync::{Arc, Mutex};
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use once_cell::sync::Lazy;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;


// 日志文件处理相关内容
lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<Option<(String, File)>> = Mutex::new(None);
}

// 创建新的日志文件
fn create_log_file() -> Option<(String, File)> {
    // 确保日志目录存在
    let log_dir = Path::new("Log");
    if let Err(e) = fs::create_dir_all(log_dir) {
        eprintln!("无法创建日志目录: {}", e);
        return None;
    }
    
    // 创建带有时间戳的文件名
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("Log/log_{}.log", timestamp);
    
    // 打开文件
    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&filename) 
    {
        Ok(file) => Some((filename.clone(), file)),
        Err(e) => {
            eprintln!("无法创建日志文件 {}: {}", filename, e);
            None
        }
    }
}

fn write_to_log_file(message: &str) -> bool {
    let mut file_guard = LOG_FILE.lock().unwrap();
    
    // 检查是否需要创建新的日志文件
    let create_new_file = match &*file_guard {
        Some((filename, _)) => {
            let current_date = chrono::Local::now().format("%Y%m%d").to_string();
            !filename.contains(&current_date)
        },
        None => true
    };
    
    if create_new_file {
        if let Some(new_file) = create_log_file() {
            *file_guard = Some(new_file);
        }
    }
    
    // 写入日志
    if let Some((_, file)) = file_guard.as_mut() {
        if let Err(_) = writeln!(file, "{}", message) {
            return false;
        }
        if let Err(_) = file.flush() {
            return false;
        }
        return true;
    }
    
    false
}

//日志记录器
pub struct LogCollector{
    pub logs: Vec<String>,
}

impl LogCollector{
    pub fn new() -> Self{
        Self { logs: Vec::new() }
    }
    //添加日志
    pub fn add(&mut self, message: String){
        self.logs.push(message);
    }

    //获取日志
    pub fn get_logs(&mut self) -> Option<Vec<String>>{
        if self.logs.is_empty(){
            return None;
        }
        let logs = self.logs.clone();
        
        self.clear_logs();
        Some(logs)
    }

    //清空日志
    pub fn clear_logs(&mut self){
        self.logs.clear();
    }
}

pub static LOG_COLLECTOR: Lazy<Arc<Mutex<LogCollector>>> =   //?
    Lazy::new(|| Arc::new(Mutex::new(LogCollector::new())));


struct CollectorLogger;
impl log::Log for CollectorLogger{
    fn enabled(&self, metadata: &Metadata) -> bool{
        metadata.level() <= Level::Debug
    }
    
    fn log(&self,record: &Record){
        if self.enabled(record.metadata()){
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S:%3f");
            let log_message = format!("[{}] {}: {}", 
                timestamp, record.level(), record.args());

            if let Ok(mut collector) = LOG_COLLECTOR.lock(){
                collector.add(log_message.clone());
            }
            

            println!("{}", log_message);

            //写入到文件
            write_to_log_file(&log_message);
        }
    }

    fn flush(&self) {
        //确保文件被刷新
        let mut file_guard = LOG_FILE.lock().unwrap();
        if let Some((_, file)) = file_guard.as_mut() {
            let _ = file.flush();
        }
    }

}

// 静态日志记录器
static LOGGER: CollectorLogger = CollectorLogger;

// 初始化日志系统
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
}