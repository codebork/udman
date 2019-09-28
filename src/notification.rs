use dbus::blocking::Connection;
use dbus::arg::{Array, Dict, Variant};
use std::time::Duration;
use std::collections::HashMap;
use std::vec::Vec;

pub struct Notification {
    app_name: String,
    replaces_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: HashMap<String, Variant<String>>,
    expire_timeout: i32
}

impl Default for Notification {
    fn default() -> Notification {
        Notification {
            app_name: "mounter".to_string(),
            replaces_id: 0,
            app_icon: "".to_string(),
            summary: "".to_string(),
            body: "".to_string(),
            actions: vec!["".to_string()],
            hints: HashMap::new(),
            expire_timeout: -1
        }
    }
}

impl Notification {
    pub fn new() -> Notification {
        Notification {
            ..Default::default()
        }
    }
    
    pub fn mounted(object_path: String) -> Notification {
        Notification {
            summary: "Filesystem mounted".to_string(),
            body: object_path,
            ..Default::default()
        }
    }
    
    pub fn unmounted(object_path: String) -> Notification {
        Notification {
            summary: "Filesystem unmounted".to_string(),
            body: object_path,
            ..Default::default()
        }
    }

    pub fn send(&self) -> u32 {
        let conn = Connection::new_session().expect("Could not connect to session bus");
        let notifier = conn.with_proxy(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            Duration::from_millis(5000)
        );

        let (notification_id,): (u32,) = notifier.method_call(
            "org.freedesktop.Notifications",
            "Notify",
            (
                &self.app_name,
                &self.replaces_id,
                &self.app_icon,
                &self.summary,
                &self.body,
                Array::new(&self.actions),
                Dict::new(&self.hints),
                &self.expire_timeout
            )
        ).expect("Could not send notification");

        notification_id
    }
}
