use crate::{DefaultArg, ExtractInnerText};

impl From<DefaultArg> for () {
    fn from(_: DefaultArg) -> Self {
        
    }
}

impl From<DefaultArg> for &'static ExtractInnerText {
    fn from(_: DefaultArg) -> Self {
        const INSTANCE: ExtractInnerText = ExtractInnerText;
        &INSTANCE
    }
}
