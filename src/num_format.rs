use formato::Formato;

pub trait ToFormattedString {
    fn to_formatted_string(&self) -> String;
}

impl<T: Formato> ToFormattedString for T {
    fn to_formatted_string(&self) -> String {
        self.formato("N0")
    }
}
