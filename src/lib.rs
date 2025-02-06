// TODO: create a struct with fields
// required for managing a http client
// including decode response and headers
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Sling {
    // http_client
    method: String,
    raw_url: String,
    // header: map[string][]string,
    // body_provider interface
    // // response_decoder serde json decoder
}
#[allow(unused)]
impl Sling {
    fn new() -> Self {
        Sling {
            method: String::new(),
            raw_url: String::new(),
        }
    }
    fn set_uri(&mut self, url: &str) {
        self.raw_url = url.to_string();
    }
    fn raw_uri(&self) -> String {
        self.raw_url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sling_test() {
        let result = Sling::new();
        assert_eq!(
            result,
            Sling {
                method: "".to_string(),
                raw_url: "".to_string(),
            }
        );
    }

    #[test]
    fn set_uri() {
        let url_value = String::from("rul");
        let value: &str = url_value.as_str();
        let mut sling = Sling::default();
        sling.set_uri(value);
        assert_eq!(sling.raw_uri(), value)
    }
}
