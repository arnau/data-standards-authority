---
type: guidance-section
part_of: api-management
identifier: retirement
ordinal: 4
---
# Retirement

Something that is often overlooked when developing and managing APIs is planning how to take them out of service when they are no longer needed. People often use the terms retirement, deprecation, sunsetting, or decommissioning interchangeably.

It may be useful to put together an API retirement workflow or checklist for your organisation to help teams follow a consistent process. Below is an example of what this might look like.

1. Use analytics to support your case for deprecation
2. Publish a blog post to explain the reasons, and offer alternatives where possible
3. Add a deprecation notice to documentation, with the date it will happen
4. Disable sign ups in self-service to stop new users accessing the service
5. Email subscribed users with the date of deprecation - as the date approaches, emails should get more frequent
6. Use Sunset or Deprecated HTTP headers, with a link to the documentation and blog post
7. Wait and see how usage changes - make sure it’s dropping, and reach out to any remaining active users
8. Agree an acceptable number of active users at retirement
9. Keep the API in retired status for a while - this could be months or years
