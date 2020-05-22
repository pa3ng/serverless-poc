# Management API

At a minimum, the management API(s) of our serverless platform need to allow for basic CRUD operations on serverless webhooks. We need to be able to create, update, read, delete, and list available webhooks. So we need a specialized web server component implementing this API(s). The proxy must be able to route the management API requests to this component.

![Management API](/images/management_api.svg)

Management APIs themselves are stateless. Any durable representation of a serverless webhook, most notably its code, must be stored outside of the context of this component.