TODO
~~~~

- [ ] diff file mode git-style

::
    diff --git a/run-examples.sh b/run-examples.sh
    old mode 100644
    new mode 100755


- [ ] command to print/show/display line number of file at given commit hash
  - signature: ``ofvr show <STATE_PATH> {hash}:{linenumber}``
  - example: ``ofvr show tests/test_models_commit.state ee5603939657736d:16`` # show line 16 of commit hash `ee5603939657736d5ecf039f47404ead012538ff52fcbf86ca200d7c4b186df3`
