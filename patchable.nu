#!/usr/bin/env nu

use std log

def repo-root [] {
    git rev-parse --show-toplevel
}

def product-root [product: string] {
    $"(repo-root)/($product)"
}

def product-work-root [product: string] {
    $"(product-root $product)/patchable-work"
}

def product-repo [product: string, --upstream: string] {
    let repo_path = $"(product-work-root $product)/product-repo"
    log info $"Repository root for ($product) is ($repo_path)"
    if not ($repo_path | path exists) {
        log info $"Repository root not found, cloning from upstream ($upstream)"
        git clone --bare $upstream $repo_path
    } else {
        log info "Repository root found, reusing"
    }
    $repo_path
}

def product-version-worktree-root [product: string, version: string] {
    $"(product-work-root $product)/worktree/($version)"
}

def product-version-worktree-branch [version: string] {
    $"patchable/($version)"
}

def product-version-patch-dir [product: string, version: string] {
    $"(product-root $product)/stackable/patches/($version)"
}

def product-version-config [product: string, version: string] {
    let path = $"(product-version-patch-dir $product $version)/patchable.toml"
    log info $"Loading patch config from ($path)"
    open $path
}

def "main" [] {
    print "Usage:"
    print "  patchable checkout [--help]"
}

def "main checkout" [
    product: string
    version: string
    --force
] {
    let config = product-version-config $product $version
    let product_repo = (product-repo $product --upstream=$config.upstream)
    let worktree_path = product-version-worktree-root $product $version
    let worktree_branch = product-version-worktree-branch $version
    let $patch_dir = product-version-patch-dir $product $version
    log info $"Worktree root is ($worktree_path)"
    log info $"Worktree branch is ($worktree_branch), from base ($config.base) and patches at ($patch_dir)"
    # These environment variables make git operate on the product worktree from now on
    # $GIT_DIR must be the worktree's .git dir, even if that is just an alias for the backing repo, since each worktree maintains its own index
    $env.GIT_DIR = $"($worktree_path)/.git"
    $env.GIT_WORK_TREE = $worktree_path
    let worktree_git_dir = git rev-parse --git-dir
    let worktree_rebase_progress_dir = $"($worktree_git_dir)/rebase-apply"
    if ($worktree_path | path exists) {
        log info "Worktree root already exists, resetting"
        if ($worktree_rebase_progress_dir | path exists) {
            if $force {
                log warning "Rebase/apply is in progress, aborting it"
                git am --abort
            } else {
                error make {msg: "Rebase/apply is in progress, abort manually or pass --force flag"}
            }
        }
        git checkout $config.base
    } else {
        log info "Worktree root not found, creating"
        # $GIT_DIR won't exist yet, so we need to override it
        git --git-dir $product_repo --work-tree $product_repo worktree add $worktree_path $config.base --detach
    }
    log info $"Creating work branch ($worktree_branch)"
    git checkout -B $worktree_branch
    log info "Importing patches"
    let series_file = $"($patch_dir)/series"
    if ($series_file | path exists) {
        log info $"Series file exists at ($series_file), treating as stgit series"
        git am $"($patch_dir)/series" --patch-format stgit-series
    } else {
        log info $"No series file found at ($series_file), treating as git mailbox"
        git am ...(glob $"($patch_dir)/*.patch" | sort)
    }
}

def "main export"  [
    product: string
    version: string
] {
    let config = product-version-config $product $version
    let product_repo = (product-repo $product --upstream=$config.upstream)
    let worktree_path = product-version-worktree-root $product $version
    let worktree_branch = product-version-worktree-branch $version
    let $patch_dir = product-version-patch-dir $product $version
    log info $"Worktree root is ($worktree_path)"
    log info $"Worktree branch is ($worktree_branch), from base ($config.base) and patches at ($patch_dir)"
    # These environment variables make git operate on the product worktree from now on
    # $GIT_DIR must be the worktree's .git dir, even if that is just an alias for the backing repo, since each worktree maintains its own index
    $env.GIT_DIR = $"($worktree_path)/.git"
    $env.GIT_WORK_TREE = $worktree_path
    log info "Deleting existing patches"
    rm ...(glob $"($patch_dir)/{*.patch,series}" | tee { print $in })
    log info $"Exporting patches to ($patch_dir)"
    git format-patch $config.base -o $patch_dir --base $config.base
}
