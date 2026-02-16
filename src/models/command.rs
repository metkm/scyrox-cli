#[repr(u8)]
pub enum Command {
    EncryptionData = 0x01,            // 下传加密沟通数据
    PCDriverStatus = 0x02,            // 下传驱动状态的命令（驱动是否处于窗口激活状态）
    DeviceOnLine = 0x03,              // 获取无线鼠标是否在线
    BatteryLevel = 0x04,              // 获取电池电量
    DongleEnterPair = 0x05,           // 设置无线Dongle进入配对状态
    GetPairState = 0x06,              // 获取无线Dongle配对结果
    WriteFlashData = 0x07,            // 设置eeprom内容
    ReadFlashData = 0x08,             // 获取eeprom内容
    ClearSetting = 0x09,              // 恢复出厂设置
    StatusChanged = 0x0A,             // 上报鼠标某些状态改变，如DPI等
    SetDeviceVidPid = 0x0B,           // 设置Dongle的USB VID/PID
    SetDeviceDescriptorString = 0x0C, // 设置Dongle的USB设备描述字符串
    EnterUsbUpdateMode = 0x0D,        // 进入USB升级模式
    GetCurrentConfig = 0x0E,          // 获取当前配置
    SetCurrentConfig = 0x0F,          // 设置当前配置
    ReadCIDMID = 0x10,                // 获取鼠标CID/MID
    EnterMTKMode = 0x11,              // 设置无线Dongle进入EMI/MTK测试模式
    ReadVersionID = 0x12,             // 获取鼠标版本号
    Set4KDongleRGB = 0x14,            // 设置4K dongle RGB灯模式,dongle上有个rgb灯（不是在鼠标上）
    Get4KDongleRGBValue = 0x15,
    SetLongRangeMode = 0x16,
    GetLongRangeMode = 0x17,
    GetDongleVersion = 0x1D, // 获取dongle版本
    MusicColorful = 0xB0,    // 音乐律动全彩
    MusicSingleColor = 0xB1, // 音乐律动全键单色
    WriteKBCIdMID = 0xF0,    // 读取cid mid,cx53710专用
    ReadKBCIdMID = 0xF1,     // 读取cid mid,cx53710专用
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        value as u8
    }
}
