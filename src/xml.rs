pub fn string_for_attr(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    name: &str,
) -> Option<String> {
    for attr in attributes {
        if attr.name.local_name == name {
            return Some(attr.value.clone());
        }
    }

    None
}

pub fn num_for_attr<I>(attributes: &Vec<xml::attribute::OwnedAttribute>, name: &str) -> Option<I>
where
    I: atoi::FromRadix10SignedChecked,
{
    if let Some(string) = string_for_attr(attributes, name) {
        atoi::atoi(string.as_bytes())
    } else {
        None
    }
}

pub fn f32_for_attr(attributes: &Vec<xml::attribute::OwnedAttribute>, name: &str) -> Option<f32> {
    if let Some(string) = string_for_attr(attributes, name) {
        if let Ok(f) = string.parse() {
            Some(f)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn num_for_chars<I>(chars: String) -> Option<I>
where
    I: atoi::FromRadix10SignedChecked,
{
    atoi::atoi(chars.as_bytes())
}
