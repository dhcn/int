use crossbeam_channel::Receiver;

pub struct ConvergeRecorder {
    rec: Receiver<String>,
    //file_writer: FileWriter,
}

impl ConvergeRecorder {
    pub fn new(rec: Receiver<String>) -> ConvergeRecorder {
        ConvergeRecorder {
            rec: rec,
            //file_writer:FileWriter::new("ConvergeSubscriber".to_string()),
        }
    }
    pub fn run(&mut self){
        while let Ok(_r) = self.rec.recv() {
            //self.file_writer.append_data(r.as_bytes());
        }
    }
}