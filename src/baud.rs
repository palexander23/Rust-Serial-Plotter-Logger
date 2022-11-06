use strum::EnumIter;

#[derive(Default, Debug, EnumIter, Clone, Copy)]
pub enum Baud {
    BAUD110 = 110,
    BAUD300 = 300,
    BAUD600 = 600,
    BAUD1200 = 1200,
    BAUD2400 = 2400,
    BAUD4800 = 4800,
    BAUD9600 = 9600,
    BAUD14400 = 14400,
    BAUD19200 = 19200,
    BAUD38400 = 38400,
    BAUD57600 = 57600,
    #[default]
    BAUD115200 = 115200,
    BAUD128000 = 128000,
    BAUD256000 = 256000,
}

impl Into<u32> for Baud {
    fn into(self) -> u32 {
        return self as u32;
    }
}

impl Into<String> for Baud {
    fn into(self) -> String {
        return (self as u32).to_string();
    }
}
