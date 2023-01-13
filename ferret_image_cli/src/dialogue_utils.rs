use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use strum::IntoEnumIterator;

pub fn enum_select_default<E>(prompt: &str, default: &str) -> Result<Option<E>>
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let mut items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    items.push(default.to_string());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(items.len() - 1)
        .items(items.as_slice())
        .interact()?;
    if selection == items.len() - 1 {
        Ok(None)
    } else {
        Ok(Some(E::iter().nth(selection).expect(
            "Past bounds of enum iterator. Dialoguer may have changed.",
        )))
    }
}

pub fn enum_select<E>(prompt: &str) -> Result<E>
where
    E: IntoEnumIterator + std::fmt::Display,
{
    let items: Vec<String> = E::iter().map(|e| e.to_string()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items.as_slice())
        .interact()?;

    Ok(E::iter()
        .nth(selection)
        .expect("Past bounds of enum iterator. Dialoguer may have changed."))
}

pub fn optional_input(prompt: &str) -> Result<Option<String>> {
    Ok(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default("_".into())
        .interact()
        .map(|s: String| (s != "_").then_some(s))?)
}
