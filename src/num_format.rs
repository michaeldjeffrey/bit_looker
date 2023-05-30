use formato::Formato;

pub trait ToFormattedString {
    fn to_formatted_string(&self) -> String;
}

impl ToFormattedString for usize {
    fn to_formatted_string(&self) -> String {
        self.formato("N0")
    }
}

impl ToFormattedString for u32 {
    fn to_formatted_string(&self) -> String {
        self.formato("N0")
    }
}

impl ToFormattedString for u128 {
    fn to_formatted_string(&self) -> String {
        self.formato("N0")
    }
}
