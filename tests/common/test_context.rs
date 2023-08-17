use super::create_test_context;
use libbasicweb::context::Context;
use libbasicweb::webapp::WebApp;

pub struct TestContext {
    pub context: Context,
    pub app: WebApp,
}

impl TestContext {
    pub async fn new() -> TestContext {
        let c = create_test_context().await;
        let app = WebApp::new(c.clone());

        TestContext { context: c, app }
    }

    pub fn run(&self) {
        let router = self.app.router.clone();
        let addr = self.app.addr.clone();

        tokio::spawn(async move {
            axum::Server::bind(&addr)
                .serve(router.into_make_service())
                .await
                .unwrap();
        });
    }

    pub fn url(&self) -> String {
        format!("http://localhost:{}", self.context.config.addr.port())
    }
}
