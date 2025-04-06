# Zet - A cli for zettelkasten with easy indexing

> Inspiration taken from the amazing work of [@rwxrob](https://github.com/rwxrob)
> keg project (installable via `go install github.com/rwxrob/keg/cmd/keg@latest`)

## How to install?

Right now we just support installing with the `cargo` package manager, so you
can [setup rust and cargo](https://rustup.rs) and then run the following command 👇

```sh
cargo install zettelkasten-cli
```

## Commands

0. *DONE* `zet (i)nit`: Initialize the repo in the current empty directory
1. *DONE* `zet (t)itles`: Print all the titles from the entries (the tags are presented
alongside the title, for easy filtering in notes like journaling or other tag)
2. `zet (e)dit`: Get a entry by any substring of the available titles.
3. `zet (g)rep`: Search for particular substring or regex pattern within your
entries content
4. *DONE* `zet (c)reate`: Create a new entry, register on the index file and open your
`$EDITOR` on the particular file.
5. `zet (conf)ig`: Sub commands related to the config
      - `zet (conf)ig (p)rint`: Print all the values currently set for the config
    (including defaults, the default should contain a label expliciting its
    default value)
      - `zet (conf)ig (m)odify <key> <value>`: Change or create a particular key on the
    config, errors out if the key doesn't exist on the spec
      - `zet (conf)ig (g)et <key>`: Print the value for a particular config key, errors
out if the key doens't exist on the config file
6. *DONE* `zet (n)ow`: Create a new entry with the title predefined for the current date
and a special tag for journaling, that way it can be filtered differently from
the rest of the notes
7. `zet (a)genda`: List all the checkboxes `- [ ]` in your repository and print
out a list of unfinished todos with optional schedules (with the syntax
`@schedule <date>`)
8. *DONE* `zet (im)port`: Import a directory with a *particular structure* into our another repo
9. *DONE* `zet (s)ync`: Push the changes to a remote if it actually has one configured

> That *particular structure* is: `<numeric_id>/README.md` for each entry,
inside the directory with the numeric id its possible to have assets, we'll
recursively call all of those
> The `README.md` need to have a top level title `#` that will be used to index
on the database.

## The index file

The index file will be a `json` for easier parsing (with `jq` for example) with the following structure:

```json
{
  // ...metadata stuff like author, repo, etc etc
  "entries": [
    {
      "id": 1,
      "title": "Welcome to my blog",
      "created_at": "2024-10-27 18:55:01Z",
      "modified_at": "2025-01-27 18:55:01Z", // can be null
      "dir_path": "../1",
      "entry_file": "README.md"
    }
  ]
}
```
