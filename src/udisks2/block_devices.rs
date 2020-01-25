use dbus::strings::Path;
use dbus::arg::{Variant, RefArg};
use crate::udisks2::Udisks2InterfacesAndProps;
use crate::udisks2::{Filesystem, Encrypted};

#[derive(Clone, Debug, PartialEq)]
pub enum Interface {
    Filesystem,
    Encrypted
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub interfaces: Vec<Interface>,
    pub object_path: Path<'static>,
    pub uuid: Option<String>,
    pub device: String,
    pub preferred_device: String,
    pub symlinks: Option<Vec<String>>,
    pub device_number: Option<u64>,
    pub label: Option<String>,
    pub fs_info: Option<FsInfo>
}

/*
 * Id                    readable   s
 * Size                  readable   t
 * ReadOnly              readable   b
 * Drive                 readable   o
 * MDRaid                readable   o
 * MDRaidMember          readable   o
 * IdUsage               readable   s
 * IdType                readable   s
 * IdVersion             readable   s
 * Configuration         readable   a(sa{sv})
 * CryptoBackingDevice   readable   o
 * HintPartitionable     readable   b
 * HintSystem            readable   b
 * HintIgnore            readable   b
 * HintAuto              readable   b
 * HintName              readable   s
 * HintIconName          readable   s
 * HintSymbolicIconName  readable   s
 */

impl Block {
    pub fn has_interface(&self, interface: Interface) -> bool {
        self.interfaces.contains(&interface)
    }

    pub fn as_fs(&self) -> Filesystem {
        Filesystem { device: self.to_owned() }
    }

    pub fn as_enc(&self) -> Encrypted {
        Encrypted { device: self.to_owned() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FsInfo {
    pub mount_paths: Option<Vec<String>>
}

pub fn get(object_path: &Path<'static>, interfaces_and_properties: &Udisks2InterfacesAndProps) -> Option<Block> {
    if let Some(block_interface) = interfaces_and_properties.get("org.freedesktop.UDisks2.Block") {
        let mut block = Block {
            object_path: object_path.to_owned(),
            ..Default::default()
        };

        for (key, value) in block_interface {
            match key.as_str() {
                "IdUUID" => block.uuid = get_string(value),
                "IdLabel" => block.label = get_string(value),
                "Device" => block.device = get_byte_string(value)?,
                "PreferredDevice" => block.preferred_device = get_byte_string(value)?,
                "Symlinks" => block.symlinks = get_byte_strings(value),
                "DeviceNumber" => block.device_number = get_u64(value),
                _ => ()
            }
        }

        if let Some(fs_interface) = interfaces_and_properties.get("org.freedesktop.UDisks2.Filesystem") {
            let mut fs = FsInfo::default();

            for (key, value) in fs_interface {
                match key.as_str() {
                    "MountPoints" => fs.mount_paths = get_byte_strings(value),
                    _ => ()
                }
            }

            block.fs_info = Some(fs);
            block.interfaces.push(Interface::Filesystem);
        } 

        if interfaces_and_properties.contains_key("org.freedesktop.UDisks2.Encrypted") {
            block.interfaces.push(Interface::Encrypted)
        } 

        Some(block)
    } else {
        None
    }
}

fn get_byte_strings(arg: &Variant<Box<dyn RefArg>>) -> Option<Vec<String>> {
    arg.0.as_iter().and_then(|t| {
        let hold: Vec<String> = t.map(|r| {
            r.as_iter().and_then(|bytes| {
                let mut inner_vec: Vec<u8> = bytes.flat_map(|byte| byte.as_u64().map(|x| x as u8)).collect();

                if inner_vec.last() == Some(&0) {
                    inner_vec.pop();
                }

                String::from_utf8(inner_vec).ok()
            })
        }).filter_map(|x| x).collect();

        if hold.len() > 0 {
            return Some(hold)
        }

        None
    })
}

fn get_u64(arg: &Variant<Box<dyn RefArg>>) -> Option<u64> {
    arg.0.as_u64()
}

fn get_string(arg: &Variant<Box<dyn RefArg>>) -> Option<String> {
    arg.0.as_str().and_then(|s| {
        if s.is_empty() { None } else { Some(s.to_owned()) }
    })
}

fn get_byte_string(arg: &Variant<Box<dyn RefArg>>) -> Option<String> {
    arg.0.as_iter().and_then(|bytes| {
        let mut inner_vec: Vec<u8> = bytes.flat_map(|byte| byte.as_u64().map(|x| x as u8)).collect();

        if inner_vec.last() == Some(&0) {
            inner_vec.pop();
        }

        String::from_utf8(inner_vec).ok()
    })
}