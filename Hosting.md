# Hosting

Hosting a custom instance of Nest-Server is fairly simple.

First, edit the file `Repository.toml`.

Specify the `name` of the repository (remember that it must follow any requirements specified by the Nest specification) and its `pretty_name` (the same than the name, without the restrictions).

You should also set the `auth_token` to some custom and complex secure password. It will be used to remotely upload or remove any package on the server.

The `links` array is used to dynamically configure the "related links" section on the navigation bar of the website.

Example:

```toml
name = "example"
pretty_name = "Example"

package_dir = "./packages/"             # Don't edit this values if you are unsure of what you are doing
cache_dir = "./cache/"                  # (Same here)

auth_token = "a_very_strong_password"   # Definitely edit this one though!

[[links]]
name = "Example"
url = "/"
active = true

[[links]]
name = "Stable"
url = "https://stable.raven-os.org"

[[links]]
name = "Beta"
url = "https://beta.raven-os.org"

[[links]]
name = "Unstable"
url = "https://unstable.raven-os.org"
```

You can also edit `Rocket.toml` to edit any network-related settings.


When you are done, compile and run Nest-Server:

```shell
$ cargo run
```

Remember that you need the latest Rust nightly to properly compile this project.

## Managing packages

There are two ways to add a package:

  * Either by moving its NPF (`.nest`) to the right place in the `package_dir` folder (Note that it **must** be named `<package_dir>/<category>/<name>/<name>-<version>.nest`)
  * Or by uploading it using the `GET /api/upload` route (See `API.md`).

Similarly, there are two ways to remove a package:

  * Either by removing it from the `package_dir` folder
  * Or by using `DELETE /api/p/<category>/<name>/<version>` route (See `API.md`).
