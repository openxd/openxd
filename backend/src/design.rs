use std::collections::HashMap;

use crate::dependency::{VersionReq, PackageReq};
use crate::device::{Device, DeviceRule};

pub struct Design {
    /// OpenXD version of the file
    pub version: VersionReq, 
    pub packages: Vec<PackageReq>,
    pub title: String,
    pub devices: Vec<Device>,
    pub device_rules: Vec<DeviceRule>,
    pub screens: Vec<Screen>,
}

pub struct Screen {
    pub title: String,
    pub canvas: Canvas,
}

pub struct Canvas {
}

pub struct Value;

pub struct ItemRef;

pub struct ItemUsed {
    pub implements: Vec<ItemUsed>,
    pub item_ref: ItemRef,
    pub arguments: HashMap<String, Value>,
    pub childs: Vec<ItemUsed>,
}
