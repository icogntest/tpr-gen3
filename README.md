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
