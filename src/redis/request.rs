use reqwest::{
    header::{
        HeaderMap as Headers,
        HeaderValue,
        AUTHORIZATION,
        CONTENT_LENGTH,
        CONTENT_TYPE,
        //USER_AGENT,
    },
    Url,
};
use reqwest::{Client, RequestBuilder as ReqwestRequestBuilder};
use tracing::instrument;
use super::{
    routing::RouteInfo,
    HttpError
};
use std::sync::Arc;

pub(crate) struct RequestBuilder<'a> {
    body: Option<&'a [u8]>,
    headers: Option<Headers>,
    route: RouteInfo<'a>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(route_info: RouteInfo<'a>) -> Self {
        Self {
            body: None,
            headers: None,
            route: route_info,
        }
    }

    pub fn build(self) -> Request<'a> {
        Request::new(self)
    }

    pub fn body(&mut self, body: Option<&'a [u8]>) -> &mut Self {
        self.body = body;

        self
    }

    pub fn headers(&mut self, headers: Option<Headers>) -> &mut Self {
        self.headers = headers;

        self
    }

    pub fn route(&mut self, route_info: RouteInfo<'a>) -> &mut Self {
        self.route = route_info;

        self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Request<'a> {
    pub(super) body: Option<&'a [u8]>,
    pub(super) headers: Option<Headers>,
    pub(super) route: RouteInfo<'a>,
}

impl<'a> Request<'a> {
    pub fn new(builder: RequestBuilder<'a>) -> Self {
        let RequestBuilder {
            body,
            headers,
            route,
        } = builder;

        Self {
            body,
            headers,
            route,
        }
    }

    #[instrument(skip(token))]
    pub fn build(
        &'a self,
        client: &Arc<Client>,
        uri: &str,
        token: &str
    ) -> Result<ReqwestRequestBuilder, HttpError> {
        let Request {
            body,
            headers: ref request_headers,
            route: ref route_info,
        } = *self;

        let (method, path) = route_info.deconstruct(uri.to_string());

        let mut builder = client.request(method.reqwest_method(), Url::parse(&path)?);

        if let Some(bytes) = body {
            builder = builder.body(Vec::from(bytes));
        }

        let mut headers = Headers::with_capacity(4);
        headers
            .insert(AUTHORIZATION, HeaderValue::from_str(token).map_err(HttpError::InvalidHeader)?);

        if self.body.is_some() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&body.unwrap_or(&Vec::new()).len().to_string())
                .map_err(HttpError::InvalidHeader)?,
        );

        if let Some(ref request_headers) = request_headers {
            headers.extend(request_headers.clone());
        }

        Ok(builder.headers(headers))
    }
}