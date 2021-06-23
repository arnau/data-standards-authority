---
type: guidance
identifier: api-management
maintainer: data-standards-authority
status: draft
creation_date: 2021-03-22
update_date: 2021-05-13
authors:
  - Annie Streater
reviewers:
  - Charles Baird
  - Arnau Siches
  - Steve Evans
  - Alicia Matheson
purpose: |
    - the benefits of API management tools
    - how API management fits into the API lifecycle
    - identifying when to use an API Management tool
    - factors to consider when choosing a solution
    - how to approach product selection to consider public-facing, open-source solutions and commercial vendors
    - standards the solution may need to meet, including accessibility
    - potential customisation needs
    - how to plan ahead for flexibility and scale
user_needs: |
   - Technologists in government who currently manage a small number of APIs and are considering how to scale
   - Technologists in government who are already using an API Management solution and are looking to review best practice
   - Senior stakeholders in government who review and approve technology spend and strategies
route: |
    - API design guidance collection
    - Search results on GOV.UK
out_of_scope: |
  - Reference documentation or in-depth technical guidance on how to build
tags:
  - API management
  - API
  - API tools
  - API platform
  - APIM
---
# API Management Guidance

<div class="highlight">

API management tools help reduce development overhead and time to deployment by standardising design, monitoring, security and other aspects of the API lifecycle.

<div>


## Define an API management strategy

Good API governance comes from good API management, from design to deprecation.

If you build or manage any number of APIs, it’s best practice to have API management in place. This guidance will outline some areas you should consider when making decisions about how to manage APIs in your team or organisation.


## Principles for your API strategy

### Build foundations early

If you have any APIs that need access management, monitoring or documentation, you should at the very least use an API gateway. Even a single API will benefit from the standardised tools that an API gateway provides, by not having to develop and deploy those elements of functionality. If you later build more APIs, having an API gateway in place already will mean you don’t have to build those tools again, reducing duplication of effort for future teams.

There will be cases where your current ecosystem doesn’t fit with this model, perhaps because you already have APIs in production which have their own security or monitoring, or because the APIs have been developed using disparate methods and technologies, in which case you should think about how you can put in place standardised patterns. This is about processes and governance that teams will use to build and deploy APIs, rather than the specific technology they use. As an example, you should consider how prescriptive you want your publishing pipeline to be. You might decide to have strict rules only for certain things like security, while allowing other areas to be more flexible.

### Keep the future in mind

When putting together your API strategy, it’s important to consider what your organisation’s future needs are likely to be, not just think about the first few APIs.

Maintain a high level view of your API ecosystem. Good API management provides a way to judge the maturity of an individual API compared to others in the estate, and allows you to determine how much resource needs to be invested in it. APIs at different stages of their lifecycle require different levels of investment - a coherent strategy allows you to identify these needs and meet them.

### Meet user needs

You should also think how your decisions might affect user behaviour. For example, you might decide that only APIs which have been through the process of assurance and testing will be visible in your catalogue. This might make teams reluctant to try new solutions that don’t fit the pattern, which could discourage innovation. Teams who aren’t ready to standardise or hand off management might simply not use it, and instead deploy APIs in an ad-hoc way.

User research can help you gain insight on how teams are producing and consuming APIs, or what technologies they are using or considering using.

A robust API management process will help build credibility and trust with users of your APIs. It’s important to demonstrate that your APIs are well designed and well supported. For example, being clear about how you retire APIs will reassure developers that they will not suddenly lose access to an API without notice.

### Define roles

Depending on the size and structure of your organisation, you might have several teams involved in building and maintaining APIs. Your API management strategy can help set out things like:

- who owns different parts of the API lifecycle
- what skills or roles API teams should include
- where ownership of a particular service lies
- how ownership changes as an API goes into service

For example, it is common for a central team to run the API Gateway, and therefore have control over service levels and capacity. You should then consider how that responsibility might interact with the design and delivery responsibilities of an API owner, such as versioning.

Having a clear structure for escalating issues will save a lot of time and energy in the future. Support from senior management or stakeholders can help formalise organisational structures and policies.

### What API Management strategy can help with

An API management strategy can:

-  provide visibility of all your APIs, encouraging reuse
-  allow you to standardise common design patterns
-  help you automate many administrative and operational tasks
-  provide a central place for you to share API documentation and support
-  provide data and metrics to help you understand your API’s performance and usage
-  help you implement and maintain good security and data protection practices

This guidance includes sections exploring each stage of the API lifecycle:

- Design - guidelines and policies for API production
- Deployment - assessment, testing and rolling out to live
- Management - ongoing maintenance and monitoring
- Discovery - how developers discover your APIs
- Retirement - decommissioning your API when it is no longer needed

Each stage will benefit from having different tools and processes in place.


## Design

Establishing design guidance helps maintain consistency across your API estate. It is difficult to manage APIs that are not consistent, so the more your APIs follow the same design principles and practices, the easier it will be to manage them at scale.

The level of guidance you provide will depend on the size and structure of your organisation. For example, guidance would be very different for a small, homogeneous environment where every aspect of API design is mandated, and a large, federated organisation where many different technologies and approaches are already in use.

## Review and challenge assumptions

Organisations often develop their API strategy based on the first few APIs they built, where choices that teams made in response to specific problems are then mandated across very different domains. If you have an existing API strategy in place, it is worthwhile to think about what assumptions in your API programme might be legacy.

User research can help you challenge these assumptions and define needs. Some questions you might seek to answer include:

- What technologies are in use, or might be in the future?
- What are the different levels of maturity of development teams in the organisation?
- Are the APIs produced primarily internal or external facing, or a mixture?
- What SLAs should APIs meet - do you have different SLA tiers which are appropriate for different use cases?


## Bring developers into the design process

API design guidance reduces friction by providing a framework within which a developer can start to work.

It’s important to provide explanations for design choices. Being open with the decisions behind the guidance increases developer trust and helps get them on board with the process.

## Use standards

Technology standards like OpenAPI can help take a lot of the guesswork out of software decisions, and can also lend credibility to your strategy as they reflect widely accepted ways of doing things.

It is helpful to specify use cases for different API standards. As an example, you might require using REST for microservices and GraphQL for more data intensive processes.

You should also consider what standards make sense for specific technical details. For example, how to use HTTP functionality, or requiring that all APIs follow a consistent approach to versioning.

## Make guidance clear and easily accessible

Setting out decisions like these in a central place helps development teams get started on projects more quickly and with less effort. Some organisations simply keep these guidelines on a central document store or git repository, while some have dedicated tooling. Various tools are available to store and share API specifications, many of which have an open source version which allows you to try them out.

Tooling like this can provide functionality including:

- specification support
- real time validation
- auto-completion
- auto generated documentation

These tools often form part of an API Management suite, which also includes a developer hub and an API gateway, but can also be used as standalone design or governance modules.


## Deployment

Deploying your API involves more than just pushing code. There are a number of things you should put in place to support the publishing process. Assurance and testing processes ensure APIs meet requirements and function properly, and developer resources help consumers through the lifecycle of their integration.

## Assurance and assessment

Assurance is part of governance, and is the process of making sure that APIs meet various organisational standards. For example, an assurance process will help teams make sure they are building APIs to follow the design guidelines of your organisation.

Assurance is likely to be a review session, and might include assessment on API:

- resourcing
- user journeys
- middleware
- domain modelling
- publishing

Your assurance assessment programme should make sense for the teams who need to go through it. If development teams find the assurance process too rigid, they might try to avoid it, which will weaken your organisation’s governance model. Think about how you can make governance as light touch as possible, and how you can make it a positive experience for product teams. If review sessions have good outcomes for teams and provide them with useful feedback, other teams are more likely to submit to it. As a result your governance model will be more successfully embedded into organisational culture.

## Testing

Testing is the process of making sure the APIs function properly, and is more likely to be an automatic process which validates the code. Testing should be an ongoing process, and regular validation of the API should be part of a monitoring scheme.

Usually, testing is done by an automated tool against the specification, sometimes known as the contract. Contract testing in its simplest form includes:

- checking the specification against internal policies to make sure that APIs submitted to the developer portal are compliant with internal standards
- checking the API to make sure its responses match the specification and that deployed APIs are behaving correctly in production

## Developer portal

A developer portal, also sometimes called a developer hub, is where developers can access everything they need to set up and manage their integration with your API. This usually includes:

- documentation for your API
- developer authorisation
- self service tools for things like renewing API keys and changing their password
- a test environment - for example a mock API, a ‘sandbox’ and test users
- issue reporting and ticket support

You can also use your developer portal to gather internal metrics about your API programme. For example, you might want to measure how quickly developers are able to set up a test version of an API. This is sometimes known as “time to live” or “time to 200” and is a useful metric to measure how easy your API is to integrate. It can also help you identify where there might be pain points for users.

## Documentation

Your API documentation is the starting point for developers looking to use your API.

You can use an API specification to generate and auto-update reference documentation for your API as you build and iterate it. However, do not rely solely on auto-generated documentation as it’s important to also include conceptual or practical documentation to provide developers with more context.

You should work with a [technical writer] from the very start of your project. If you don’t have technical writing resources in your organisation, you can reach out to the [cross-government technical writing community].

You can use the [GDS Technical Documentation Template] to generate accessible documentation pages from simple Markdown files. It’s mainly used for standalone documentations sites (for example [GOV.UK Pay’s API documentation]). The Tech Docs Template is a Middleman template, so if you’re using an API management tool, you might be able to build the template into your generated documentation..

Read the guidance on [how to document APIs] and [writing API reference documentation].


[how to document APIs]: https://www.gov.uk/guidance/how-to-document-apis
[writing API reference documentation]: https://www.gov.uk/guidance/writing-api-reference-documentation
[GDS Technical Documentation Template]: https://tdt-documentation.london.cloudapps.digital/#technical-documentation-template
[technical writer]: https://www.gov.uk/guidance/technical-writer
[cross-government technical writing community]: https://www.gov.uk/service-manual/communities/technical-writing-community
[GOV.UK Pay’s API documentation]: https://docs.payments.service.gov.uk/#gov-uk-pay-technical-documentation
[cross-government technical writing community]: https://www.gov.uk/service-manual/communities/technical-writing-community

## Discovery

Making your API easy to find is a critical part of the API management process. If people can’t find your APIs, they can’t use them. You can use an API catalogue to help users discover your APIs, find out what they do and how to use them.

## Decouple your API discovery layer from your dev portal and API gateway

Development teams often rely solely on a developer portal to make their APIs discoverable. Most API management tooling includes modules for the API gateway, the developer portal, and an API catalogue.

However, there are cases where it may not make sense for an API to sit on the gateway or the developer portal, but which should still be discoverable in a catalogue. This might be because the API is still in experimental stages, doesn’t meet standards, or because the team is not ready to hand over management of the API.

It is then a good idea to have the catalogue as a separate entity, so it can make all APIs discoverable without being restricted to those which meet certain criteria.

This helps:

- promote an environment where innovation can happen in the open
- avoids teams developing in silos to get around rules
- helps transparency and reduces duplication of effort

## Consider who will be looking to find APIs in your organisation

Most organisations have three levels of access for their APIs:

- Private - internal and kept entirely within the organisation
- Partner - shared externally with a limited set of trusted users/partners
- Public - open to external users either directly or through a self-service sign up platform

Consider how best to enable discovery for each group. You could do this with separate catalogues, but you should consider the cost and effort it would require to maintain these. You can also use access control to restrict visibility of APIs, or details of the APIs, to registered users at different levels of authentication.

You should aim to expose as much detail to all users as possible. Even for sensitive datasets you should expose basic details of an API with information about how to get access. This helps developers understand the value of the API and start interacting with it while they wait for access approval.

You should consult your internal security teams about what level of exposure is acceptable for each API. You might also find it helpful to review the metadata model available at the [government API catalogue]. This provides a basic discovery framework that does not increase the vulnerability of published resources.

## Link your catalogue to the Government API Catalogue

Your internal catalogue should update the [government API catalogue], either programmatically or with manual submissions. This catalogue is a centralised directory for discovery of government APIs, and any external APIs (whether inter-departmental, restricted or public) should be listed on there.

Contact the API Catalogue team at [api-catalogue@digital.cabinet-office.gov.uk] for more information and help with adding your APIs or connecting your internal catalogue.



[government API catalogue]: https://www.api.gov.uk/
[api-catalogue@digital.cabinet-office.gov.uk]: mailto:api-catalogue@digital.cabinet-office.gov.uk


## Management tools

The day to day operations of API service management are usually handled by an API gateway. Generally, an API gateway service acts as a proxy for APIs, and allows commodity services (for example, network access, security and authentication, throttling and rate limiting, logging and monitoring) to be provisioned centrally, for all the APIs using the service.


Many organisations don’t put an API gateway into production until they have several APIs in use. However, you should consider implementing one at the very beginning of an API programme.


Doing this will allow you to:

- standardise how APIs use central services such as network access, security and authentication, throttling and rate limiting, logging and monitoring
- lower developer costs and reduce time to value, since you won’t need to rebuild these services for each new API
- mandate minimum standards of security, access control and monitoring, while providing an easy way for developers to meet them


While adding an API gateway does mean you will need to deploy and manage another service, it reduces complexity of the API estate and introduces a consistent front end for services.

There are many different API gateway products available, both open source and proprietary. They will often include elements for a dev portal and/or a catalogue. You can read more about these further in this guidance.

At a minimum, an API gateway should provide:

- user and application authentication
- rate limiting and throttling inbound requests
- logging and reporting


## Retirement

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
