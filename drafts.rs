trait Handler {
    type HttpRequest;
    type HttpResponse;
    type Error;

    fn call(&self, request: Self::HttpRequest);
}

struct RouteHandler<F, RQ, RS>(F, PhantomData<(RQ, RS)>)
where
    F: Fn(RQ) -> RS;

impl<F, RQ, RS> Handler for RouteHandler<F, RQ, RS>
where
    F: Fn(RQ) -> RS,
    RQ: 'static,
    RS: 'static,
{
    type HttpRequest = RQ;
    type HttpResponse = RS;
    type Error = anyhow::Error;

    fn call(&self, request: Self::HttpRequest) {
        todo!()
    }
}
