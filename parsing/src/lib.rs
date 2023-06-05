use std::todo;

pub enum EnvFile {
    RequirementsTxt,
    EnvironmentYaml,
    PyprojectToml,
}

pub struct RequirementsTxt {
    path: String,
    name: String,
}

pub struct EnvironmentYaml {
    path: String,
    name: String,
}

pub struct PyprojectToml {
    path: String,
    name: String,
}

fn parse_from_txt() -> Vec<String> {
    todo!()
}

fn parse_from_yaml() -> Vec<String> {
    todo!()
}

fn parse_from_toml() -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {

}
