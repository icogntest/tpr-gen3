For tauri-dev:
From "tauri-app" dir, run `yarn tauri dev` for dev mode of the tauri app.

For tauri-build:
From "tauri-app" dir, run `yarn tauri build`.

For github actions:

- Go to Settings > Actions > General > Workflow permissions, and change to "Read and write permissions".

TODO (big items):

Ref for auto-update (v1): https://www.youtube.com/watch?v=ZXjlZBisYPQ

Ref for different update channels: https://github.com/tauri-apps/tauri/issues/2595

- guide on auto-updating: https://github.com/tauri-apps/tauri/discussions/2776
- look at SSRando artifact stuff (see if this is possible after looking into auto-updating)
- some sort of script which copies the standalone output to the resources dir (create if needed) in the src-tauri folder.
  Node that node_modules DOES need to be copied in.
  Can possibly remove some files, but can worry about slightly optimizing the installed size at the end.
- package and use drizzle with nextjs website (volume, etc. working with tauri in AppData?)

asdf

- What if only auto-update on stable branch
- People would be allowed to publish the website code as something to switch to.
- No reason to give people the ability to change the main program (swapping between branches, etc.)
- We also then don't need to share the private key info at all

Next steps:

- Store what the package.json version was last time we ran (1.2.0 for example).
- We don't care about whether or not it was "dev", "isaac", etc.; only the number comparison matters
- The name for the branch can be set with a variable in the environment.
  It is called "dev" for all forks, but they can set the exact name in a var.
- This name is used to create the version like "1.2.0-dev.8".
- When we start, if the previous semver exists and is LESS than the current semver one, then reset to 0.
  It will get changed to 1 and never 0, but that is fine.
- We also want to write the version to an artifact "config.json" which will be read by our stable workflow.
  This workflow will update a json file in a branch which will be used by API endpoints to get info about branches.
  For example, site URL, latest version, artifact URLs, possibly store the artifact signatures here as well.
  This would allow us to validate the zip file that we download to make sure what we end up downloading to their
  site matches what we expect.

Next steps:

- Centralized workflow which queues against itself if there are multiple.
- Needs to verify the sender has permission to make the request.
- Need to validate the input from the requester and update a file in a branch for the config
  which is used by the menus
