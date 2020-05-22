# Gateway Proxy

Serverless webhooks as well as APIs that allow their management are both exposed over HTTP. HTTP endpoints will be the main entry into the functionality of our VM, and an HTTP proxy is a component that will decide how to process a particular request.

![Gateway Proxy](/images/proxy.svg)

You can use a variety of technologies for the proxy: HAProxy, Nginx, all the way to writing your own.