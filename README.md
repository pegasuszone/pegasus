
# Peer-2-Peer NFT trade contract

This contract allows you to offer a nft you own for a nft someone else owns
Inspiration took from the nft marketplace "https://github.com/public-awesome/marketplace/blob/bfab827cdff84f76f4a637edb3920042c1606b4c/contracts/marketplace/src/state.rshg "

First, the owner of the nft needs to approve the contract to spend the token (see cw721-base (approve message)[https://github.com/CosmWasm/cw-nfts/blob/4e26419bb02f4b871fda487964a80bd419207428/contracts/cw721-base/src/execute.rs#L50])

Then, the contract will set a Ask offer just like the market place only the prive is a specific nft token. 



## cosmwasm template notes
**Latest: 1.0.0-beta6**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
````

**Older Version**

Pass version as branch flag:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch <version> --name PROJECT_NAME
````

Example:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch 0.16 --name PROJECT_NAME
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful referenced, but please set some
proper description in the README.

## Gitpod integration

[Gitpod](https://www.gitpod.io/) container-based development platform will be enabled on your project by default.

Workspace contains:
 - **rust**: for builds
 - [wasmd](https://github.com/CosmWasm/wasmd): for local node setup and client
 - **jq**: shell JSON manipulation tool

Follow [Gitpod Getting Started](https://www.gitpod.io/docs/getting-started) and launch your workspace.

