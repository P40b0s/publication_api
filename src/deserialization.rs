use utilites::Date;

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    Date::parse(s).ok_or(serde::de::Error::custom(format!("Формат даты {} не поддерживается", s)))
}