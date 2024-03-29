use std::collections::HashMap;

pub fn does_route_match(pattern: &str, route: &str) -> Option<HashMap<String, String>> {
    let mut pattern_chars = pattern.chars().into_iter();
    let mut route_chars = route.chars().into_iter();

    let mut parsing_value = false;
    let mut start_parsing_name = false;
    let mut parsing_name = false;
    let mut param_name = String::new();
    let mut param_value = String::new();
    let mut params = HashMap::new();

    let mut pattern_char = pattern_chars.next().unwrap_or_default();
    let mut route_char = route_chars.next().unwrap_or_default();

    loop {        
        if pattern_char == ':' {
            parsing_value = true;
            start_parsing_name = true;
        }

        if pattern_char == '/' || pattern_char == '\0' {
            parsing_name = false;
        }

        if route_char == '/' || route_char == '\0' {
            parsing_value = false;
        }

        if parsing_name {
            param_name.push(pattern_char);

            pattern_char = pattern_chars.next().unwrap_or_default();
        }

        if parsing_value {
            param_value.push(route_char);

            route_char = route_chars.next().unwrap_or_default();
        }

        if start_parsing_name {
            parsing_name = true;
            start_parsing_name = false;
            pattern_char = pattern_chars.next().unwrap_or_default();
        }

        if parsing_name || parsing_value {
            continue;
        }

        if param_name.len() > 0 {
            params.insert(param_name, param_value);
            param_name = String::new();
            param_value = String::new();

            pattern_char = pattern_chars.next().unwrap_or_default();
            route_char = route_chars.next().unwrap_or_default();

            continue;
        }

        if pattern_char == '\0' && route_char == '\0' {
            break;
        }

        if pattern_char != route_char {
            return None;
        }

        pattern_char = pattern_chars.next().unwrap_or_default();
        route_char = route_chars.next().unwrap_or_default();
    }

    Some(params)
}

pub fn find_matches(patterns: &Vec<String>, route: &str) -> Option<(String, HashMap<String, String>)> {
    for pattern in patterns {
        match does_route_match(&pattern, route) {
            Some(params) => {
                return Some((pattern.to_string(), params));
            },
            None => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_match() {
        let map = does_route_match(
            "/text/:text", 
            "/text/makkara"
        );
        assert_eq!(
            map, 
            Some(
                [("text".to_string(), "makkara".to_string())].iter().cloned().collect()
            )
        );
    }

    #[test]
    fn test_does_match2() {
        let map = does_route_match(
            "/person/:personId/profile", 
            "/person/qwert/profile"
        );

        assert_eq!(
            map, 
            Some(
                [("personId".to_string(), "qwert".to_string())].iter().cloned().collect()
            )
        );
    }

    #[test]
    fn test_does_match3() {
        let map = does_route_match(
            "/person/:personId/profile", 
            "/person/qwert/profile/123"
        );

        assert_eq!(map, None);
    }

    #[test]
    fn test_find_matches() {
        let routes = vec![
            "/hello".to_string(),
            "/text/:textName".to_string(),
        ];
        let text = find_matches(&routes, "/text/makkara");
        assert_eq!(
            text, 
            Some(
                (
                    "/text/:textName".to_string(), 
                    vec![("textName".to_string(), "makkara".to_string())].into_iter().collect()
                )
            )
        );
    }
}