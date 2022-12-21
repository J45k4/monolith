

// // #[async_trait::async_trait]
// // trait Route {
// //     async fn call(&self);
// // }

// // struct Handler1 {

// // }

// // #[async_trait::async_trait]
// // impl Route for Handler1 {
// //     async fn call(&self) {
// //         println!("Handler1");
// //     }
// // }

// // struct Hadnler2 {

// // }

// // #[async_trait::async_trait]
// // impl Route for Hadnler2 {
// //     async fn call(&self) {
// //         println!("Handler2");
// //     }
// // }

// // #[derive(Clone)]
// // struct Muu {

// // }

// // impl Copy for Muu {}

// // impl Muu {
// //     pub fn call(&self) {
// //         println!("Muu");
// //     }
// // }

// // static MUU: Muu = Muu{};

// // struct Routes {
// //     routes: Vec<Box<dyn Route>>
// // }

// // impl Routes {
// //     pub fn add(&mut self, f: impl Fn()) {

// //         struct s {

// //         }

// //         #[async_trait::async_trait]
// //         impl Route for s {
// //             async fn call(&self) {
// //                 f();
// //             }
// //         }

// //         self.routes.push(Box::new(s{}));
// //     }
// // }

// use std::pin::Pin;

// use futures::Future;

// struct HttpRequest {

// }

// enum HttpResponse {
//     Ok
// }

// struct Router {
//     routes: Vec<Route>,
// }

// struct Route {
//     path: String,
//     handler: Box<dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output=HttpResponse> + Send>> + Send>,
// }

// impl Router {
//     pub fn add_route<F>(&mut self, path: String, handler: F)
//         where F: 'static + Fn(HttpRequest) -> Pin<Box<dyn Future<Output=HttpResponse> + Send>> + Send
//     {
//         self.routes.push(Route {
//             path,
//             handler: Box::new(handler),
//         });
//     }
// }

// async fn hello(req: HttpRequest) -> HttpResponse {
//     HttpResponse::Ok
// }

// #[cfg(test)]
// mod tests {
//     // use super::{Route, Handler1, Hadnler2, MUU};

//     // #[tokio::test]
//     // async fn it_works() {
//     //     let mut v: Vec<Box<dyn Route>> = vec![];

//     //     v.push(Box::new(Handler1{}));
//     //     v.push(Box::new(Hadnler2{}));

//     //     for i in v {
//     //         i.call().await;
//     //     }

//     //     // routes! {
//     //     //     GET "/" => Handler1,
//     //     //     GET "/test" => Hadnler2
//     //     // }
//     // }

//     // #[test]
//     // fn it_works2() {
//     //     MUU.call();

//     //     struct Test {
//     //         muu: Muu
//     //     }
//     // }

//     use super::*;

//     #[test]
//     fn it_works3() {
//         let mut router = Router {
//             routes: vec![],
//         };

//         router.add_route("/".to_string(), hello);
//     }

//     #[test]
//     fn lol() {
//         let router = Router::new();

//         router.use("/")
//     }
// }