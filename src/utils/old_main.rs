
    // let config = SseServerConfig {
    //     bind: config.server_address.parse()?,
    //     sse_path: "/sse".to_string(),
    //     post_path: "/message".to_string(),
    //     ct: tokio_util::sync::CancellationToken::new(),
    // };
    //
    // let (sse_server, router) = SseServer::new(config);
    //
    // // Do something with the router, e.g., add routes or middleware
    //
    // let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;
    //
    // let ct = sse_server.config.ct.child_token();
    //
    // let server = axum::serve(listener, router).with_graceful_shutdown(async move {
    //     ct.cancelled().await;
    //     tracing::info!("sse server cancelled");
    // });
    //
    // tokio::spawn(async move {
    //     if let Err(e) = server.await {
    //         tracing::error!(error = %e, "sse server shutdown with error");
    //     }
    // });
    //
    // let ct = sse_server.with_service(Counter::new);
    //
    // tokio::signal::ctrl_c().await?;
    // ct.cancel();
    // Ok(())
    //
    // // Configure CORS for SSE connections
    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_methods(Any)
    //     .allow_headers(Any);
    //     // .expose_headers(vec![
    //     //     header::CONTENT_TYPE.as_str(),
    //     //     header::CACHE_CONTROL.as_str(),
    //     //     header::CONNECTION.as_str(),
    //     // ]);
    //
    // // Build our application with routes
    // let app = Router::new()
    //     .route("/", get(health_check))
    //     .route("/sse", get(sse_handler))
    //     .with_state(linkedin_server)
    //     .layer(TraceLayer::new_for_http())
    //     .layer(cors);
    //
    // // Run the server
    // let addr: SocketAddr = config.server_address.parse()?;
    // info!("Starting LinkedIn MCP server on {}", addr);
    //
    // // Use tokio's TcpListener for axum
    // let listener = tokio::net::TcpListener::bind(addr).await?;
    // axum::serve(listener, app).await?;
    //
    // Ok(())
