mod entities;
mod contracts;
mod contracts_impl;
mod http;

use ohkami::{Ohkami, Route, fang::Context};
use contracts_impl::managed_connection_pool::ManagedConnectionPool;
use http::RequestContext;
use std::sync::Arc;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        // Инициализируем наш кастомный управляемый пул изолированных соединений SQLite
        let pool = ManagedConnectionPool::initialize("pharmacy.db".into(), 10).await;
        let request_context = RequestContext::new(Arc::new(pool));

        println!("HTTP сервер запущен на http://localhost:3000");

        Ohkami::new((
            Context::new(request_context),
            "/add-medicine".POST(http::add_medicine),
            "/medicines/:id"
                .GET(http::get_medicine)
                .PUT(http::update_medicine)
                .DELETE(http::remove_medicine),
        ))
        .howl("localhost:3000")
        .await;

        Ok(())
    })
}
