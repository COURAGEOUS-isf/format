# Steps to take to release a new format version
1. Change the version on the `Cargo.toml` file, adhering to semver semantics:

    The **library version**, i.e. the part before the plus in the package version, must change every time a new release is done.
    Changes are considered breaking if they break the crate's public API (i.e. Packages using the previous version may not compile successfully with the new release).
    - If the changes **add functionality** in some **breaking** way, e.g. adds a new member to a structure or renames an existing member,
        - If **before 1.0.0**, increment the MINOR version by one.
        - If **after 1.0.0**, increment the MAJOR version by one.

    - If the changes **add functionality** in some **backwards compatible** way, e.g. adds a new standalone structure or function for the purpose of allowing new functionality,
        - If **before 1.0.0**, increment the MINOR version by one.
        - If **after 1.0.0**, increment the MINOR version by one.

    - If the changes **fix bugs or mishaps** in some **backwards compatible** way, e.g. clarifies documentation or fixes any issue preserving backwards compatibility with the public API,
        - Increment the PATCH version by one.

    For a more complete list of breaking changes, read the [SemVer compatibility chapter in the Cargo book](https://doc.rust-lang.org/cargo/reference/semver.html).

    [`cargo semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) SHOULD be used to check backwards compatibility if releasing a new patch version.

    The **schema version**, i.e. the part after the plus in the package version (also called build metadata), must change every time the schema changes in a release.
    Changes are considered breaking if previously accepted JSON files could be rejected by the new schema.
    - If the changes **add functionality** in some **breaking** way, e.g. adds a non-optional member to a structure or renames an existing member forbidding the previous naming,
        - If **before 1.0.0**, increment the MINOR version by one.
        - If **after 1.0.0**, increment the MAJOR version by one.

    - If the changes **add functionality** in some **backwards compatible** way, e.g. adds a new optional member or changes the schema in any way other than adding or changing documentation for the purpose of allowing new functionality,
        - If **before 1.0.0**, increment the MINOR version by one.
        - If **after 1.0.0**, increment the MINOR version by one.

    - If the changes **fix bugs or mishaps** in some **backwards compatible** way, e.g. clarifies documentation or in any way changes the schema for the purpose of fixing a non-intentional issue,
        - Increment the PATCH version by one.

2. Make sure the `CHANGELOG.md` file is up to date and add the changes belonging to this release below a level 2 markdown heading.

3. Run the following command in order to update the schema.
```sh
cargo run --example schemagen --features schemars
```

4. Create and push a new commit. The commit message does not need to follow any standard.

5. Create a new annotated tag with the name set to the new version preceded by a `v`, e.g. `v0.3.1+schema.0.3.0`.
6. IF the schema version changes, an update to `track2kml` is required to support it. Follow these steps:
    1. On the `track2kml` repository, change `dependencies.courageous-format.tag` to the new package version of this repository.
    2. Update `track2kml` accordingly to support the new changes. Remember to change the format version present in `cli/main.rs` if required.
    3. Update the `track2kml` package version using the same criteria as used for the library version here.
    4. Make sure the `CHANGELOG.md` file is up to date and add the changes belonging to this release below a level 2 markdown heading.
    5. Create and push a new commit. The commit message does not need to follow any standard.
    6. Update the private `track2kml-ext` repository: Change `dependencies.courageous-format.tag`, fix internal errors.
    7. Create and push a new commit on the `track2kml-ext` repository.

7. IF AND ONLY IF the schema version changes, an update to the format page is required. Follow these steps:
    1. Copy the current schema file (in the project directory, named `courageous.schema.json`) to the [page repository's](https://github.com/COURAGEOUS-isf/format-website) `/schemas/` folder.
    2. Rename it to `v{version}.json`, where `{version}` is the schema version (i.e. build metadata of this repository's package)
    3. Create a file of the same name, just with the `html` extension inside `/schemas/changelogs/` using the format found on the other files. Copy the changes you wrote on the `CHANGELOG.md` file of this repository.
    4. If `track2kml` has been updated too, compile a release from the `track2kml-ext` repository using the `generate_release.sh` script. Repeat steps 2 to 4 except using the resulting `release/COURAGEOUS-version.zip` file, using the `.zip` extension and using the `track2kml` folder instead of `schemas`.
    5. Commit and push on the page repository.
    6. Send a zip of the page to imaza at us dot es to update the page.