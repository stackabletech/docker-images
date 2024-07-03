import sys

import conf

if len(sys.argv) != 2:
    sys.exit('Expected product name as input')

for product in conf.products:
    product_name = product['name']
    if product_name != sys.argv[1]:
        continue

    for version in product['versions']:
        print(f"{product_name}#{version['product']}")