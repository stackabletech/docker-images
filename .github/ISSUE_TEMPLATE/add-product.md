---
name: Add new Product
about: >-
  This template contains instructions specific to adding a new product.
title: >-
  tracking: Add new product <PRODUCT>
labels: []
# Currently, projects cannot be assigned via front-matter.
projects: ['stackabletech/10']
assignees: ''
---

## Tasks

- [ ] Create a new top-level folder for the product. The name of the folder must
      use the lowercase product name.
- [ ] Create a README.md file outlining special considerations required to
      update or run the product. See existing README files as a guide of
      reference.
- [ ] Add a `versions.py` file to the folder. Add all required key-value pairs.
- [ ] Add a new "Update Product" issue template in `.github/ISSUE_TEMPLATE/`
      folder. See existing ones as a guide of reference.
- [ ] Add a new `build_<PRODUCT>.yml` GitHub Action workflow in the
      `.github/workflows` folder. Use existing local action whenever possible
      or consider creating a new one when there is no fitting action available.
- [ ] Run `.scripts/update_readme_badges.sh` to generate the new status badge.

_Please consider updating this template if these instructions are wrong, or
could be made clearer._
