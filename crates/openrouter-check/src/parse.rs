use openrouter_models::{known_capability_names, Namespace};

use crate::error::Error;
use crate::format::sorted_names;
use crate::suggest::closest_name;

pub const CAPABILITY_PATH_MESSAGE: &str =
    "expected OpenRouter capability path in the form `param::name`, `input::name`, or `output::name`";

pub fn parse_capability(path: &str) -> Result<(Namespace, String), Error> {
    let mut segments = path.split("::");
    let Some(namespace) = segments.next() else {
        return Err(Error::invalid_capability_path(CAPABILITY_PATH_MESSAGE));
    };
    let Some(name) = segments.next() else {
        return Err(Error::invalid_capability_path(CAPABILITY_PATH_MESSAGE));
    };
    if namespace.is_empty() || name.is_empty() || segments.next().is_some() {
        return Err(Error::invalid_capability_path(CAPABILITY_PATH_MESSAGE));
    }

    Ok((
        parse_namespace(namespace)
            .map_err(|error| Error::invalid_capability_path(error.to_string()))?,
        name.to_owned(),
    ))
}

pub fn parse_namespace(text: &str) -> Result<Namespace, Error> {
    match text {
        "param" => Ok(Namespace::Param),
        "input" => Ok(Namespace::Input),
        "output" => Ok(Namespace::Output),
        unknown => Err(Error::unknown_namespace(unknown)),
    }
}

pub fn validate_capability_name(namespace: Namespace, name: &str) -> Result<(), Error> {
    let known_names = known_capability_names(namespace).map_err(Error::ModelLookup)?;

    if !known_names.contains(name) {
        return Err(Error::unknown_capability(
            namespace,
            name,
            unknown_capability_suggestion(namespace, name, known_names),
        ));
    }

    Ok(())
}

fn unknown_capability_suggestion(
    namespace: Namespace,
    name: &str,
    known_names: &std::collections::HashSet<String>,
) -> String {
    closest_name(name, known_names)
        .map(|known_name| {
            format!(
                "; did you mean `{}`?",
                crate::format::format_capability(namespace, known_name)
            )
        })
        .unwrap_or_else(|| {
            format!(
                "; known {} capabilities: {}",
                namespace.as_str(),
                sorted_names(known_names).join(", ")
            )
        })
}
