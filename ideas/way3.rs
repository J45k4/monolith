macro_rules! routes {
    ($($route:literal => $handler:ident),*) => {
        |route: &str| {
            let route_string = route.to_string();
            match route {
                $(
                    $route => {
                        let handler: fn($($handler::Input),*) -> $handler::Output = $handler;
                        let mut route_parts = route_string.split('/');
                        route_parts.next(); // skip the empty string at the beginning
                        let mut handler_args = Vec::new();
                        for part in route_parts {
                            handler_args.push(part.to_string());
                        }
                        // `await` the result of the handler function
                        let result = handler(handler_args).await;
                        result
                    },
                )*
                _ => panic!("Invalid route: {}", route),
            }
        }
    }
}

// Usage:
let routes = routes![
    "/search/{param1}/{param2}" => search_page,
    "/models/{param}" => models_page,
    "/test_suite/{test_suite_name}/test/{test_name}" => test_page
];

// Dispatch a request to the appropriate handler function:
routes("/search/foo/bar"); // Calls `search_page("foo", "bar")`
routes("/models/baz"); // Calls `models_page("baz")`
routes("/test_suite/qux/test/quux"); // Calls `test_page("qux", "quux")`
