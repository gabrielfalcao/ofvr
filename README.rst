OFVR: One File VeRsioning
-------------------------

CLI tool that versions one single file within one single file.


0xf09f85b6


WHY?
====

It appears that it wouldn't be entirely unreasonable to suppose that GIT is
pretty ubiquitous nowadays and have been so for a while.

It appears that it also would not be entirely unreasonable to state
that it is not uncommon to see software developers versioning the git
configuration itself (.i.e.: `~/.gitconfig`) in a github repository
named "dotfiles".

But what if version control could be done in a per-file basis, kept
offline and entirely private?

In other words, what if files such as `~/.gitconfig`, `~/.bashrc` and
`~/.vimrc` could be versioned separately and managed with git-like
commands such as `diff`, `commit`, `apply`, `log`, `restore`, `checkout`,
`reset` and so on?

On top of that, what if binary files could also be versioned, so that
when you update an application in your computer, you could hold
vendors accountable to supply-chain attacks as you keep track of
changes in binary files?

With that in mind OFVR is a tool to version individual files, even
binary files, browse their diffs, calculate checksums, apply binary or
text diffs and become a forensic master of your own domain.
