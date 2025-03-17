use crate::backend;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;

use http_body_util::BodyExt;
use hyper::service::Service;
use hyper::{Method, Request, Response, Uri};

#[derive(Clone)]
pub struct Serv(pub Arc<Mutex<backend::TicketStore>>);

type Res = Response<String>;
type Err = hyper::http::Error;
type Fut = Pin<Box<dyn Future<Output = Result<Res, Err>> + Send>>;
type Req = Request<hyper::body::Incoming>;

macro_rules! respond {
    // boilerplate to make the async response work
    ($self:ident, $method:ident, $response:expr) => {{
        let v = $self.clone().$method($response);
        return Box::pin(async { v.await });
    }};
}

impl Service<Req> for Serv {
    type Response = Res;
    type Error = Err;
    type Future = Fut;
    fn call(&self, x: Req) -> Self::Future {
        let (parts, body) = x.into_parts();
        let bytes = async {
            if let Ok(x) = body.collect().await {
                return x.to_bytes().into_iter().collect();
            }
            println!("body troubles");
            todo!()
        };
        match parts.method {
            Method::GET => respond!(self, get, parts.uri),
            Method::POST => respond!(self, post, bytes),
            Method::OPTIONS => respond!(self, head, ()),
            Method::PUT => respond!(self, put, bytes),
            Method::DELETE => respond!(self, delete, bytes),
            Method::HEAD => respond!(self, head, ()),
            Method::TRACE => respond!(self, head, ()),
            Method::CONNECT => respond!(self, head, ()),
            Method::PATCH => respond!(self, head, ()),
            _ => todo!(),
        }
    }
}

impl Serv {
    async fn get(self, uri: Uri) -> Result<Res, Err> {
        let builder = Response::builder();
        if uri == "/" {
            let ids = (*self.0.lock().expect("poison")).ids();
            return builder.body(serde_json::to_string(&ids).expect("JSON encoding failed"));
        }
        if uri.path() == "/ticket" {
            if let Some(Ok(idx)) = uri.query().map(|x| x.parse::<u64>()) {
                let store = self.0.lock().expect("poison");
                let ticket = &store[backend::TicketId(idx)];
                return builder.body(serde_json::to_string(ticket).expect("JSON encoding failed"));
            }
        }
        builder.status(404).body(String::new())
    }

    async fn post<F: Future<Output = Vec<u8>>>(self, body: F) -> Result<Res, Err> {
        let builder = Response::builder();
        if let Ok(draft) = serde_json::from_reader::<_, backend::TicketDraft>(&body.await as &[u8])
        {
            let mut store = self.0.lock().expect("poison");
            let id = store.add_ticket(draft);
            builder.body(serde_json::to_string(&id).expect("JSON encoding failed"))
        } else {
            builder.body("Invalid draft ticket".to_owned())
        }
    }

    async fn delete<F: Future<Output = Vec<u8>>>(self, body: F) -> Result<Res, Err> {
        let builder = Response::builder();
        if let Ok(id) = serde_json::from_reader::<_, backend::TicketId>(&body.await as &[u8]) {
            let mut store = self.0.lock().expect("poison");
            store.delete(id);
            builder.body(String::new())
        } else {
            builder.body("Invalid ticket".to_owned())
        }
    }

    async fn put<F: Future<Output = Vec<u8>>>(self, body: F) -> Result<Res, Err> {
        let builder = Response::builder();
        if let Ok(ticket) = serde_json::from_reader::<_, backend::Ticket>(&body.await as &[u8]) {
            let mut store = self.0.lock().expect("poison");
            let id = ticket.id;
            store[id] = ticket;
            builder.body(String::new())
        } else {
            builder.body("Invalid ticket".to_owned())
        }
    }

    async fn head(self, _: ()) -> Result<Res, Err> {
        let builder = Response::builder();
        builder.body(String::new())
    }
}
