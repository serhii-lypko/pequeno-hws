trait Handler {
    type HTTPRequest;
    type HTTPResponse;
    type Error;

    fn call(&self, request: Self::HTTPRequest);
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
    type HTTPRequest = RQ;
    type HTTPResponse = RS;
    type Error = anyhow::Error;

    fn call(&self, request: Self::HTTPRequest) {
        todo!()
    }
}
