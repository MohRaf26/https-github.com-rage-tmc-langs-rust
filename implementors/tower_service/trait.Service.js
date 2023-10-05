(function() {var implementors = {
"hyper":[["impl&lt;C, B, T&gt; <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;T&gt; for <a class=\"struct\" href=\"hyper/client/service/struct.Connect.html\" title=\"struct hyper::client::service::Connect\">Connect</a>&lt;C, B, T&gt;<span class=\"where fmt-newline\">where\n    C: MakeConnection&lt;T&gt;,\n    C::Connection: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    C::Future: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    C::Error: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    B: <a class=\"trait\" href=\"hyper/body/trait.HttpBody.html\" title=\"trait hyper::body::HttpBody\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Data\" title=\"type hyper::body::HttpBody::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Error\" title=\"type hyper::body::HttpBody::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,</span>"],["impl&lt;C, B&gt; <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;<a class=\"struct\" href=\"hyper/struct.Request.html\" title=\"struct hyper::Request\">Request</a>&lt;B&gt;&gt; for <a class=\"struct\" href=\"hyper/client/struct.Client.html\" title=\"struct hyper::client::Client\">Client</a>&lt;C, B&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"hyper/client/connect/trait.Connect.html\" title=\"trait hyper::client::connect::Connect\">Connect</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    B: <a class=\"trait\" href=\"hyper/body/trait.HttpBody.html\" title=\"trait hyper::body::HttpBody\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Data\" title=\"type hyper::body::HttpBody::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Error\" title=\"type hyper::body::HttpBody::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,</span>"],["impl <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;<a class=\"struct\" href=\"hyper/client/connect/dns/struct.Name.html\" title=\"struct hyper::client::connect::dns::Name\">Name</a>&gt; for <a class=\"struct\" href=\"hyper/client/connect/dns/struct.GaiResolver.html\" title=\"struct hyper::client::connect::dns::GaiResolver\">GaiResolver</a>"],["impl&lt;C, B&gt; <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;<a class=\"struct\" href=\"hyper/struct.Request.html\" title=\"struct hyper::Request\">Request</a>&lt;B&gt;&gt; for &amp;<a class=\"struct\" href=\"hyper/client/struct.Client.html\" title=\"struct hyper::client::Client\">Client</a>&lt;C, B&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"hyper/client/connect/trait.Connect.html\" title=\"trait hyper::client::connect::Connect\">Connect</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    B: <a class=\"trait\" href=\"hyper/body/trait.HttpBody.html\" title=\"trait hyper::body::HttpBody\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Data\" title=\"type hyper::body::HttpBody::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    B::<a class=\"associatedtype\" href=\"hyper/body/trait.HttpBody.html#associatedtype.Error\" title=\"type hyper::body::HttpBody::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,</span>"],["impl&lt;R&gt; <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;<a class=\"struct\" href=\"hyper/struct.Uri.html\" title=\"struct hyper::Uri\">Uri</a>&gt; for <a class=\"struct\" href=\"hyper/client/connect/struct.HttpConnector.html\" title=\"struct hyper::client::connect::HttpConnector\">HttpConnector</a>&lt;R&gt;<span class=\"where fmt-newline\">where\n    R: Resolve + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    R::Future: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</span>"],["impl&lt;B&gt; <a class=\"trait\" href=\"hyper/service/trait.Service.html\" title=\"trait hyper::service::Service\">Service</a>&lt;<a class=\"struct\" href=\"hyper/struct.Request.html\" title=\"struct hyper::Request\">Request</a>&lt;B&gt;&gt; for <a class=\"struct\" href=\"hyper/client/conn/struct.SendRequest.html\" title=\"struct hyper::client::conn::SendRequest\">SendRequest</a>&lt;B&gt;<span class=\"where fmt-newline\">where\n    B: <a class=\"trait\" href=\"hyper/body/trait.HttpBody.html\" title=\"trait hyper::body::HttpBody\">HttpBody</a> + 'static,</span>"]],
"hyper_rustls":[["impl&lt;T&gt; <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;<a class=\"struct\" href=\"http/uri/struct.Uri.html\" title=\"struct http::uri::Uri\">Uri</a>&gt; for <a class=\"struct\" href=\"hyper_rustls/struct.HttpsConnector.html\" title=\"struct hyper_rustls::HttpsConnector\">HttpsConnector</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;<a class=\"struct\" href=\"http/uri/struct.Uri.html\" title=\"struct http::uri::Uri\">Uri</a>&gt;,\n    T::<a class=\"associatedtype\" href=\"tower_service/trait.Service.html#associatedtype.Response\" title=\"type tower_service::Service::Response\">Response</a>: <a class=\"trait\" href=\"hyper/client/connect/trait.Connection.html\" title=\"trait hyper::client::connect::Connection\">Connection</a> + <a class=\"trait\" href=\"tokio/io/async_read/trait.AsyncRead.html\" title=\"trait tokio::io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio/io/async_write/trait.AsyncWrite.html\" title=\"trait tokio::io::async_write::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + 'static,\n    T::<a class=\"associatedtype\" href=\"tower_service/trait.Service.html#associatedtype.Future\" title=\"type tower_service::Service::Future\">Future</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    T::<a class=\"associatedtype\" href=\"tower_service/trait.Service.html#associatedtype.Error\" title=\"type tower_service::Service::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,</span>"]],
"reqwest":[["impl <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;<a class=\"struct\" href=\"reqwest/struct.Request.html\" title=\"struct reqwest::Request\">Request</a>&gt; for <a class=\"struct\" href=\"reqwest/struct.Client.html\" title=\"struct reqwest::Client\">Client</a>"],["impl <a class=\"trait\" href=\"tower_service/trait.Service.html\" title=\"trait tower_service::Service\">Service</a>&lt;<a class=\"struct\" href=\"reqwest/struct.Request.html\" title=\"struct reqwest::Request\">Request</a>&gt; for &amp;<a class=\"struct\" href=\"reqwest/struct.Client.html\" title=\"struct reqwest::Client\">Client</a>"]],
"tower_service":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()