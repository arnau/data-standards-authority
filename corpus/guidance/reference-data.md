---
type: guidance
identifier: publish-reference-data
description: Share your reference data for use in projects and services outside your organisation.
maintainer: data-standards-authority
status: published
canonical_url: https://www.gov.uk/guidance/publish-reference-data-for-use-across-government
creation_date: 2021-01-22
update_date: 2021-02-01
publication_date: 2021-03-11
standards:
  - uprn
  - usrn
authors:
  - Peter Gothard
  - Lalit Shah
reviewers:
  - Arnau Siches
  - Gareth Heyes
  - Gareth Watkins
  - Emmanuel Silva
purpose: |
  This guidance helps create good practice for publishing reference data across government in a useful form.
  **It should contain:**

  - Information and resources to help readers
    - publish reference data to follow best practice
    - build a publishing, governance and support strategy
target: Anyone in government looking to publish their organisation’s reference data for others to use
user_needs: Best practice in publishing their own organisation’s reference data for the use of others
route: |
  When interested parties search GOV.UK they should get a top hit that leads them to the DSA reference data guidance collection, and a second hit to the Registers page.

  Those going directly to the Registers page should be provided with links to find the DSA guidance.
problem_statements:
  - This is an opportunity or the DSA to introduce guidance for publishing reference data in government, in order to encourage organisations to begin managing and publishng their own data real estates rather than relying on centralised platforms managed by others.
tags:
  - reference data
  - master data
  - registers
---
# Publish reference data for use across government


## Who should use this guidance

This guidance is for government employees who are publishing reference data for the use of others in government. It shares the best practice for creating a strategy to manage and support reference data for publishing.

Reference data is defined as any data used to organise or categorise other data. This can be data which lies inside or outside an organisation.

It usually consists of codes, descriptions and definitions of data, for example the [ISO-3166 country codes], which are an internationally recognised set of codes that can be used to refer to countries and their subdivisions.

## Define a reference data publishing strategy

Reference data is a valuable asset which can inform and aid user decisions. Publishing it for the use of others across government is an excellent way to share its advantages while saving time and cost for others.

Reference data should be published in a way to make it:

* always accessible and up-to-date, maximising its usefulness and avoiding potentially costly decision-making with outdated reference data
* validated and accurate at any point in time, with a history of changes maintained, and expired versions still available for consumption
* findable to its users
* supported by an organisational infrastructure that can handle the processes and demands of creating and maintaining the reference data itself, and user needs in accessing it

### Appoint a reference data owner and steward to manage your publishing strategy

Your publishing strategy should be effectively managed. A good way to do this is by appointing both a reference data owner and a reference data steward to work together to prepare your reference data properly, and support it and its users after publishing.

The reference data owner should decide on definitions, policy and decisions about your reference data based on your organisation's individual responsibility. They should also decide who can access and change the data.

The reference data steward is responsible for carrying out the rules set by the reference data owner. They are accountable for the quality of the data, compliance with regulatory requirements, and conformity to your organisation’s data principles and policies.

They should also resolve any practical data-related issues such as incomplete records or queries raised by users.

The reference data owner and steward do not have to be individuals. The responsibility is best handled by teams, in order to spread the responsibility evenly and improve accountability.

You could also create a reference data ‘forum’ of individuals drawn from relevant parts in your organisation to liaise and discuss the [<u>governance of reference data</u>] more generally, as well as in relation to publishing.

### Establish and use a single, trusted source for your reference data

Your reference data should be created from a single, authoritative data source. This single source should be placed in a storage system or database, and be known as your system of record (SOR).

In some situations, you may need to create a reference data by combining several data sources. In these cases, each published reference data set should have its own SOR, created by combining any existing SORs which contributed to it.

Each record in your reference data should be marked with a unique identifier (UID) to associate it with the same SOR for the life of the data set. This makes it easier for your users to index, search and manage the reference data, as well as tracking changes between published versions.

A UID marks a record as entirely different from every other record in a data set. The syntax of a UID should be made up of words, letters, numbers or a combination of these. Examples include serial numbers, stock keeping units (SKU) as found on barcodes on items for sales, or [currency codes], as found in international currency conversion services.

You should also make UIDs persistent, which means guaranteeing they are managed and kept unchanged for the life of the reference data, in order to ensure accuracy and consistency for your users.

When using data from other organisations, you should follow their own rules for using their data. For example, use of [Ordnance Survey’s UPRN] (unique property reference number) dataset needs to follow the
[Ordnance Survey Open Identifiers policy].

You should also be mindful when creating new SORs and reference data sets to observe government guidance around [reusing data whenever possible], and cutting down on waste and duplication.

## Publish your reference data for useability and security

Your published reference data should be readable by both humans and machines. The Government Digital Service recommends an [API-first approach], publishing reference data in JSON format. You may also want to consider publishing in [CSVW (CSV on the Web)] format, should users need a CSV file. The most important thing is that the format you choose to publish in is most suitable to your users’ needs.

When publishing to the web, you should [follow GOV.UK guidance around best practice in SEO (search engine optimisation)].

You should include metadata with your reference data providing an overview of its contents, as well as contact details of its steward, when it was created, when it was last updated, and a brief description of any new changes in the latest version.

You can find out the best ways to create metadata by [reading metadata guidance published by the Data Standards Authority].

Sometimes, you’ll find that updating a reference data set instead requires publishing it as an entirely new reference data, as changing reference data that is already being used may cause systems or platforms that are using it to malfunction. For example, the UK Standard Industrial Classification of economic activities must still provide [reference data for both 2003 and 2007 to suit different use cases].

When publishing a new version of a reference data set alongside an existing one, you should make sure that:

* the new version is published as a new, standalone reference data, and not a change or variant of the existing one
* both the existing and the new version are available to users
* correlation between the existing and new versions is made clear to users, preferably in an accompanying correlation document. For example, the UK Standard Industrial Classification of economic activities reference data is [published by ONS].

### Provide user support for your reference data

You should provide a simple way for users to give feedback or report errors when using your reference data, such as with an email link or web form.

Knowing who is consuming your reference data is very important. It allows you to provide better support to users and their community, including during upgrades, maintenance and unexpected downtime. A good way to know who is using your reference data is to encourage users to subscribe to it, for example by giving users the option to provide an email address when downloading it.

**:exclamation: You should [follow the Open Data Charter], and never require user registration to use your reference data, but let users decide whether they want to register or not.**

Your published reference data needs to be secure. This means it should be hosted in a secure environment and access to that environment managed
securely, [using HTTPS].

You can learn more about securing data by [reading the GOV.UK Service Manual].



[ISO-3166 country codes]: https://www.iso.org/iso-3166-country-codes.html
[<u>governance of reference data</u>]: https://www.dataversity.net/what-is-data-governance/
[currency codes]: https://www.iso.org/iso-4217-currency-codes.html
[Ordnance Survey’s UPRN]: https://www.ordnancesurvey.co.uk/business-government/products/open-uprn
[Ordnance Survey Open Identifiers policy]: https://www.ordnancesurvey.co.uk/business-government/tools-support/open-mastermap-programme/open-id-policy
[reusing data whenever possible]: https://www.gov.uk/guidance/manage-your-data-for-access-and-reuse
[API-first approach]: https://www.gov.uk/government/collections/api-design-guidance
[CSVW (CSV on the Web)]: https://www.w3.org/TR/tabular-data-primer/
[follow GOV.UK guidance around best practice in SEO (search engine optimisation)]: https://www.gov.uk/government/publications/search-engine-optimisation-for-publishers-best-practice-guide/search-engine-optimisation-seo-for-data-publishers-best-practice-guide
[reading metadata guidance published by the Data Standards Authority]: https://www.gov.uk/guidance/record-information-about-data-sets-you-share-with-others#metadata-you-should-record
[reference data for both 2003 and 2007 to suit different use cases]: https://www.ons.gov.uk/methodology/classificationsandstandards/ukstandardindustrialclassificationofeconomicactivities/
[published by ONS]: https://www.ons.gov.uk/methodology/classificationsandstandards/ukstandardindustrialclassificationofeconomicactivities/uksic2007
[follow the Open Data Charter]: https://www.gov.uk/government/publications/open-data-charter
[using HTTPS]: https://www.gov.uk/service-manual/technology/using-https
[reading the GOV.UK Service Manual]: https://www.gov.uk/service-manual/technology/securing-your-information
