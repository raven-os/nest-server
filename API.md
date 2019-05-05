# API

This file gives details about all the publicly available routes that form the Nest-Server API.

## `GET /api/`

A quick summary of this repository. Among others, it indicates the name of the repository and a small history of the most recent updates.

*Request parameters*: None

*Response code*: 200 OK

*Response Content-Type*: `application/json`

*Response fields*:

  * `name` (String): the name of the repository, following any conventions specified by the Nest specification.
  * `pretty_name` (String): a "pretty" name, usually the same as `name` without any restriction. It is common to capitalize the name of the repository here.
  * `manifests_count` (String): The total number of packages available in this repository.
  * `history` (Array of Object): An array of the most recent updates. This array is ordered by `date`, and may be of any length.
    * `date` (String): The date when the [`PackageManifest`] was updated (ISO 8601, UTC).
    * `manifest` (Object): A serialized [`PackageManifest`], as described by the Nest specification.

Example:

```json
{
  "history": [
    {
      "date": "2019-04-28T15:37:42Z",
      "manifest": {
        "category": "shell",
        "metadata": {
          "description": "The Bourne Again SHell.",
          "licenses": [
            "gpl_v3"
          ],
          "maintainer": "benjamin.grange@epitech.eu",
          "tags": [
            "gnu"
          ],
          "upstream_url": "https://www.gnu.org/software/bash/bash.html"
        },
        "name": "bash",
        "repository": "example",
        "versions": {
          "5.0.0": {
            "dependencies": {
              "stable::sys-lib/libc": "^2.29.0",
              "stable::sys-lib/ncurses": "^6.1.0",
              "stable::sys-lib/readline": "^8.0.0"
            },
            "kind": "effective",
            "slot": "",
            "wrap_date": "2019-04-28T15:37:42Z"
          }
        }
      }
    },
    {
      "date": "2019-04-25T21:32:15Z",
      "manifest": {
        "category": "sys-lib",
        "metadata": {
          "description": "The GNU Readline library provides a set of functions for use by applications that allow users to edit command lines as they are typed in.",
          "licenses": [
            "gpl_v3"
          ],
          "maintainer": "benjamin.grange@epitech.eu",
          "tags": [
            "gnu",
            "cli"
          ],
          "upstream_url": "https://tiswww.case.edu/php/chet/readline/rltop.html"
        },
        "name": "readline",
        "repository": "example",
        "versions": {
          "8.0.0": {
            "dependencies": {
              "stable::sys-lib/libc": "^2.29.0"
            },
            "kind": "effective",
            "slot": "",
            "wrap_date": "2019-04-25T21:32:15Z"
          }
        }
      }
    },
    {
      "date": "2019-04-24T10:25:16Z",
      "manifest": {
        "category": "sys-lib",
        "metadata": {
          "description": "A free software emulation of curses.",
          "licenses": [
            "mit"
          ],
          "maintainer": "benjamin.grange@epitech.eu",
          "tags": [
            "tui"
          ],
          "upstream_url": "https://www.gnu.org/software/ncurses/"
        },
        "name": "ncurses",
        "repository": "example",
        "versions": {
          "6.1.0": {
            "dependencies": {
              "stable::sys-lib/libc": "^2.29.0"
            },
            "kind": "effective",
            "slot": "",
            "wrap_date": "2019-04-24T10:25:16Z"
          }
        }
      }
    },
    {
      "date": "2019-04-23T15:33:29Z",
      "manifest": {
        "category": "sys-lib",
        "metadata": {
          "description": "The GNU C library",
          "licenses": [
            "gpl_v3"
          ],
          "maintainer": "benjamin.grange@epitech.eu",
          "tags": [
            "gnu"
          ],
          "upstream_url": "https://www.gnu.org/software/libc/"
        },
        "name": "libc",
        "repository": "example",
        "versions": {
          "2.29.0": {
            "dependencies": {},
            "kind": "effective",
            "slot": "",
            "wrap_date": "2019-04-23T15:33:29Z"
          }
        }
      }
    }
  ],
  "manifests_count": 4,
  "name": "example",
  "pretty_name": "Example"
}
```

## `GET /api/pull`

An array of all the [`PackageManifest`]s hosted by this repository.

*Request parameters*: None

*Response code*: 200 OK

*Response Content-Type*: `application/json`

*Response body*: An array of [`PackageManifest`]s, as described by the Nest specification. The elements are not sorted.

Example:

```json
[
  {
    "name": "bash",
    "category": "shell",
    "repository": "example",
    "metadata": {
      "description": "The Bourne Again SHell.",
      "tags": [
        "gnu"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "gpl_v3"
      ],
      "upstream_url": "https://www.gnu.org/software/bash/bash.html"
    },
    "versions": {
      "5.0.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-28T15:37:42Z",
        "dependencies": {
          "stable::sys-lib/libc": "^2.29.0",
          "stable::sys-lib/ncurses": "^6.1.0",
          "stable::sys-lib/readline": "^8.0.0"
        }
      }
    }
  },
  {
    "name": "readline",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "The GNU Readline library provides a set of functions for use by applications that allow users to edit command lines as they are typed in.",
      "tags": [
        "gnu",
        "cli"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "gpl_v3"
      ],
      "upstream_url": "https://tiswww.case.edu/php/chet/readline/rltop.html"
    },
    "versions": {
      "8.0.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-25T21:32:15Z",
        "dependencies": {
          "stable::sys-lib/libc": "^2.29.0"
        }
      }
    }
  },
  {
    "name": "libc",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "The GNU C library",
      "tags": [
        "gnu"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "gpl_v3"
      ],
      "upstream_url": "https://www.gnu.org/software/libc/"
    },
    "versions": {
      "2.29.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-23T15:33:29Z",
        "dependencies": {}
      }
    }
  },
  {
    "name": "ncurses",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "A free software emulation of curses.",
      "tags": [
        "tui"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "mit"
      ],
      "upstream_url": "https://www.gnu.org/software/ncurses/"
    },
    "versions": {
      "6.1.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-24T10:25:16Z",
        "dependencies": {
          "stable::sys-lib/libc": "^2.29.0"
        }
      }
    }
  }
]
```

## `GET /api/p/<category>/<name>`

Return all the metadata of a package identified by its name and category.

*Request parameters*:

  * `category` (String): The name of the category, following any convention described by the Nest specification.
  * `name` (String): The name of the package, following any convention described by the Nest specification.

*Response code*: 200 OK

*Response Content-Type*: `application/json`

*Response body*: A single [`PackageManifest`], as described by the Nest specification.

Example (`GET /api/p/sys-lib/ncurses`):

```json
{
  "category": "sys-lib",
  "metadata": {
    "description": "A free software emulation of curses.",
    "licenses": [
      "mit"
    ],
    "maintainer": "benjamin.grange@epitech.eu",
    "tags": [
      "tui"
    ],
    "upstream_url": "https://www.gnu.org/software/ncurses/"
  },
  "name": "ncurses",
  "repository": "example",
  "versions": {
    "6.1.0": {
      "dependencies": {
        "stable::sys-lib/libc": "^2.29.0"
      },
      "kind": "effective",
      "slot": "",
      "wrap_date": "2019-04-24T10:25:16Z"
    }
  }
}
```

## `GET /api/p/<category>/<name>/<version>`

Test if the version of a package exists.

Note: there is no route to retrieve the metadata of a single version because it has a very limited interest compared to the existing route that
retrieves all metadata for all versions of a package.

*Request parameters*:

  * `category` (String): The name of the category, following any convention described by the Nest specification.
  * `name` (String): The name of the package, following any convention described by the Nest specification.
  * `version` (String): The version of the package, following any convention described by the Nest specification.

*Response code*: 204 No Content

*Response Content-Type*: None

*Response body*: None

## `GET /api/p/<category>/<name>/<version>/download`

Download a package in its NPF (`.nest`) form.

*Request parameters*:

  * `category` (String): The name of the category, following any convention described by the Nest specification.
  * `name` (String): The name of the package, following any convention described by the Nest specification.
  * `version` (String): The version of the package, following any convention described by the Nest specification.

*Response code*: 200 OK

*Response Content-disposition*: `attachment; filename="<name>-<version>.nest"`

*Response body*: The content of the NPF (`.nest`) file

## `GET /api/p/<category>/<name>/<version>/content`

Retrieve the content of a package.

*Request parameters*:

  * `category` (String): The name of the category, following any convention described by the Nest specification.
  * `name` (String): The name of the package, following any convention described by the Nest specification.
  * `version` (String): The version of the package, following any convention described by the Nest specification.

*Response code*: 200 OK

*Response Content-Type*: `application/json`

*Response body*: An array of absolute paths, with each one being a file contained in this package.

Example (`GET /api/p/sys-lib/readline/8.0.0/content`):

```json
[
  "/usr",
  "/usr/share",
  "/usr/share/doc",
  "/usr/share/doc/readline",
  "/usr/share/doc/readline/INSTALL",
  "/usr/share/doc/readline/CHANGES",
  "/usr/share/doc/readline/README",
  "/usr/share/readline",
  "/usr/share/readline/rlversion.c",
  "/usr/share/readline/rlptytest.c",
  "/usr/share/readline/rl.c",
  "/usr/share/readline/rl-callbacktest.c",
  "/usr/share/readline/fileman.c",
  "/usr/share/readline/rlkeymaps.c",
  "/usr/share/readline/rlbasic.c",
  "/usr/share/readline/manexamp.c",
  "/usr/share/readline/rltest.c",
  "/usr/share/readline/rl-fgets.c",
  "/usr/share/readline/histexamp.c",
  "/usr/share/readline/hist_erasedups.c",
  "/usr/share/readline/excallback.c",
  "/usr/share/readline/rlevent.c",
  "/usr/share/readline/hist_purgecmd.c",
  "/usr/share/readline/rlcat.c",
  "/usr/share/man",
  "/usr/share/man/man3",
  "/usr/share/man/man3/history.3",
  "/usr/share/man/man3/readline.3",
  "/usr/share/info",
  "/usr/share/info/history.info",
  "/usr/share/info/rluserman.info",
  "/usr/share/info/readline.info",
  "/usr/lib64",
  "/usr/lib64/pkgconfig",
  "/usr/lib64/pkgconfig/readline.pc",
  "/usr/lib64/libreadline.so",
  "/usr/lib64/libhistory.so.8",
  "/usr/lib64/libreadline.so.8",
  "/usr/lib64/libreadline.so.8.0",
  "/usr/lib64/libhistory.so.8.0",
  "/usr/lib64/libreadline.a",
  "/usr/lib64/libhistory.so",
  "/usr/lib64/libhistory.a",
  "/usr/bin",
  "/usr/include",
  "/usr/include/readline",
  "/usr/include/readline/rlstdc.h",
  "/usr/include/readline/rlconf.h",
  "/usr/include/readline/tilde.h",
  "/usr/include/readline/history.h",
  "/usr/include/readline/keymaps.h",
  "/usr/include/readline/readline.h",
  "/usr/include/readline/rltypedefs.h",
  "/usr/include/readline/chardefs.h"
]
```

## `DELETE /api/p/<category>/<name>/<version>`

Remove a package.

**Note**: This route is protected by an authentification token, which must be specified in the `X-Auth-Token` HTTP header.

*Request parameters*:

  * `category` (String): The name of the category, following any convention described by the Nest specification.
  * `name` (String): The name of the package, following any convention described by the Nest specification.
  * `version` (String): The version of the package, following any convention described by the Nest specification.

*Response code*: 204 No Content

*Response Content-Type*: None

*Response body*: None

## `POST /api/upload`

Upload a package from its NPF (`.nest`) form. Its name, category and version are determined automatically from its content.

**Note**: As of now, multipart is **not** supported. Therefore, the body must have the exact same content than the uploaded file.

**Note**: The package must be in the NPF format. The best way to generate a package under this format is to use [`nbuild`](https://github.com/raven-os/nbuild).

**Note**: This route is protected by an authentification token, which must be specified in the `X-Auth-Token` HTTP header.

*Request parameters*: None

*Response code*: 204 No Content

*Response Content-Type*: None

*Response body*: None

## `GET /api/search&<q>&<search_by>&<exact_match>`

Search for packages.

*Request parameters*:

  * `q` (String): The content to look for.
  * `search_by` (String): The kind of data `q` shall be a part of. As of now, only those value are supported: `name`, `category`, `description`, `tags` and `content`.
  * `exact_match` (Bool): Indicates whether the match can be partial or must be exact. If not specified, the default value is `false`.

*Response code*: 200 OK

*Response Content-Type*: `application/json`

*Response body*: The body depends on the value of `search_by`:

  * If `search_by` is either `name`, `category`, `description` or `tags`, the content of the response body is an array of [`PackageManifest`]s (as described by the Nest specification) that match the given query.
  * Else if `search_by` is `content`, the content of the response body is an array of objects with the following elements:
    * `path` (String): The absolute path that matches the query.
    * `name` (String): The full name (as described by the Nest specification) of the package that matches the query.
    * `all_version` (Bool): A flag that indicates if all versions of the package matched the query, or if only some of them did.

Example 1 (`GET /api/search&q=libreadline.so&search_by=content`)

```json
[
  {
    "path": "/usr/lib64/libreadline.so.8.0",
    "name": "example::sys-lib/readline",
    "all_versions": true
  },
  {
    "path": "/usr/lib64/libreadline.so",
    "name": "example::sys-lib/readline",
    "all_versions": true
  },
  {
    "path": "/usr/lib64/libreadline.so.8",
    "name": "example::sys-lib/readline",
    "all_versions": true
  }
]
```

Example 2 (`GET /api/search&q=/usr/lib64/libreadline.so&search_by=content&exact_match=true`)

```json
[
  {
    "path": "/usr/lib64/libreadline.so",
    "name": "example::sys-lib/readline",
    "all_versions": true
  }
]
```

Example 3 (`GET /api/search&q=sys-lib&search_by=category`)

```json
[
  {
    "name": "readline",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "The GNU Readline library provides a set of functions for use by applications that allow users to edit command lines as they are typed in.",
      "tags": [
        "gnu",
        "cli"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "gpl_v3"
      ],
      "upstream_url": "https://tiswww.case.edu/php/chet/readline/rltop.html"
    },
    "versions": {
      "8.0.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-25T21:32:15Z",
        "dependencies": {
          "stable::sys-lib/libc": "^2.29.0"
        }
      }
    }
  },
  {
    "name": "libc",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "The GNU C library",
      "tags": [
        "gnu"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "gpl_v3"
      ],
      "upstream_url": "https://www.gnu.org/software/libc/"
    },
    "versions": {
      "2.29.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-23T15:33:29Z",
        "dependencies": {}
      }
    }
  },
  {
    "name": "ncurses",
    "category": "sys-lib",
    "repository": "example",
    "metadata": {
      "description": "A free software emulation of curses.",
      "tags": [
        "tui"
      ],
      "maintainer": "benjamin.grange@epitech.eu",
      "licenses": [
        "mit"
      ],
      "upstream_url": "https://www.gnu.org/software/ncurses/"
    },
    "versions": {
      "6.1.0": {
        "slot": "",
        "kind": "effective",
        "wrap_date": "2019-04-24T10:25:16Z",
        "dependencies": {
          "stable::sys-lib/libc": "^2.29.0"
        }
      }
    }
  }
]
```
