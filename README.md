# Tracks

Tracks is a Rust web framework, similar to Ruby on Rails.

## Structure

Tracks is structured similar to your typical Model-View-Controller framework, with some carefully crafted additions.

- Models: These map to your database objects. Most business logic should be done inside models, but authorization and access control should be done by the controller.
- Views: These are rendered ways of viewing parts of your application.
- Controllers: These control access to the business logic of your program. Controllers are primarily concerned with authorization and access control.
- Assets: These are files that need to be pre-processed before being presented in your final application, such as minifying JS/CSS.
- Jobs: These represent either long-running or scheduled tasks
- Watchers: These can be used to handle incoming mail in a folder or S3 bucket, monitor an external endpoint, etc

## Project Goals

- Be as simple as possible to start using and to start building an application.
- Make application development as smooth as possible.

