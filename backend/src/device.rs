pub enum Device {
    Ref(DeviceRef),
    Custom(CustomDevice)
}

pub struct DeviceRef {
}

pub struct CustomDevice {}

pub struct DeviceRule;
