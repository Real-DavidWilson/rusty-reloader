use std::{
    path::Path,
    process::{Child, Command, Stdio},
    sync::{
        mpsc::{channel, Receiver},
        Mutex,
    },
    time::SystemTime,
};

use notify::{
    recommended_watcher, Error, Event, ReadDirectoryChangesWatcher, RecommendedWatcher,
    RecursiveMode, Watcher,
};

pub struct ReloaderOptions<'a> {
    pub cmd: &'a str,
    pub path: &'a str,
    pub recursive: Option<RecursiveMode>,
}

pub struct Reloader {
    child: Option<Mutex<Child>>,

    cmd_string: String,
    path_string: String,

    recursive: RecursiveMode,

    watcher: ReadDirectoryChangesWatcher,
    event_receiver: Receiver<Result<Event, Error>>,
    last_reload: Option<SystemTime>,
}

impl Reloader {
    pub fn new(options: ReloaderOptions) -> Self {
        let (watcher, event_receiver) = Self::create_watcher();

        Self {
            child: None,

            cmd_string: String::from(options.cmd),
            path_string: String::from(options.path),
            recursive: options.recursive.unwrap_or(RecursiveMode::Recursive),

            watcher,
            event_receiver,
            last_reload: None,
        }
    }

    pub fn watch(&mut self) {
        let result = self
            .watcher
            .watch(Path::new(&*self.path_string), self.recursive);

        if let Err(_) = result {
            panic!("Error")
        }

        self.exec_command();
        self.handle_event();
    }

    fn create_watcher() -> (RecommendedWatcher, Receiver<Result<Event, Error>>) {
        let (tx, rx) = channel();

        let watcher = recommended_watcher(tx);

        if let Err(_) = watcher {
            panic!("Error");
        }

        let watcher = watcher.unwrap();

        (watcher, rx)
    }

    fn handle_event(&mut self) {
        loop {
            let recv = self.event_receiver.recv();

            if let Err(_) = recv {
                break;
            }

            recv.unwrap().unwrap();

            self.last_reload = Some(SystemTime::now());
            self.exec_command();
        }
    }

    fn exec_command(&mut self) {
        self.kill_child();

        let cmd_clone = self.cmd_string.clone();
        let mut cmd_split = cmd_clone.split(' ');

        let cmd = cmd_split.next();

        let child = Command::new(cmd.unwrap())
            .args(cmd_split.into_iter())
            .stdin(Stdio::inherit())
            .spawn()
            .expect("Command error");

        self.child = Some(Mutex::new(child))
    }

    fn kill_child(&mut self) {
        if self.child.is_none() {
            return;
        }

        let child = self.child.as_mut().unwrap();
        let exit_status = child.get_mut().unwrap().try_wait();

        if let Ok(Some(_)) = exit_status {
            self.child = None;

            return;
        }

        child.lock().unwrap().kill().expect("Process does not exit");
        child.lock().unwrap().wait().expect("Process did not exit");

        self.child = None;
    }
}
